use crate::{AFF4_IMAGE, Error, Metadata};
use std::{
    collections::BTreeMap,
    fs::File,
    io::{Read, Seek},
    path::Path,
};
use zip::{ZipArchive, result::ZipError};

/// Explicitly recognized on-disk version profiles; not conformance guarantees.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Profile {
    Aff4V1,
    Aff4LogicalV1,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Version {
    pub major: u32,
    pub minor: u32,
    pub tool: Option<String>,
    /// Vendor-specific fields, excluding major, minor and tool.
    pub extensions: BTreeMap<String, String>,
}

impl Version {
    fn parse(bytes: &[u8]) -> Result<Self, Error> {
        let text =
            std::str::from_utf8(bytes).map_err(|_| Error::InvalidVersion("not UTF-8".into()))?;
        let mut fields = BTreeMap::new();
        for line in text.lines().filter(|l| !l.trim().is_empty()) {
            let (key, value) = line
                .split_once('=')
                .ok_or_else(|| Error::InvalidVersion("expected key=value".into()))?;
            if key.is_empty() || fields.insert(key.to_owned(), value.to_owned()).is_some() {
                return Err(Error::InvalidVersion(format!(
                    "empty or duplicate key: {key}"
                )));
            }
        }
        let mut number = |key| -> Result<u32, Error> {
            let value = fields
                .remove(key)
                .ok_or_else(|| Error::InvalidVersion(format!("missing {key}")))?;
            if value.is_empty() || !value.bytes().all(|b| b.is_ascii_digit()) {
                return Err(Error::InvalidVersion(format!(
                    "{key} must be an unsigned decimal integer"
                )));
            }
            value
                .parse()
                .map_err(|_| Error::InvalidVersion(format!("{key} exceeds u32")))
        };
        let major = number("major")?;
        let minor = number("minor")?;
        Ok(Self {
            major,
            minor,
            tool: fields.remove("tool"),
            extensions: fields,
        })
    }
    pub fn profile(&self) -> Result<Profile, Error> {
        match (self.major, self.minor) {
            (1, 0) => Ok(Profile::Aff4V1),
            (2, 1) => Ok(Profile::Aff4LogicalV1),
            (major, minor) => Err(Error::UnsupportedVersion { major, minor }),
        }
    }
}

/// Bounds for decompressed bootstrap data and parsed metadata.
/// ZIP central-directory allocation happens inside the ZIP dependency before
/// the member-count check. These limits are not a total process memory cap.
#[derive(Clone, Copy, Debug)]
pub struct OpenOptions {
    pub max_members: usize,
    pub max_metadata_bytes: u64,
    pub max_metadata_triples: usize,
    pub max_bootstrap_bytes: u64,
}
impl Default for OpenOptions {
    fn default() -> Self {
        Self {
            max_members: 1_000_000,
            max_metadata_bytes: 64 * 1024 * 1024,
            max_metadata_triples: 1_000_000,
            max_bootstrap_bytes: 64 * 1024,
        }
    }
}

/// Owned ZIP-backed AFF4 container. This first reader inspects primary metadata;
/// it does not yet decode image streams, follow imports, or verify hashes.
pub struct Container<R: Read + Seek> {
    pub(crate) archive: ZipArchive<R>,
    volume_iri: String,
    version: Version,
    metadata: Metadata,
}

impl Container<File> {
    pub fn open(path: impl AsRef<Path>) -> Result<Self, Error> {
        Self::from_reader(File::open(path)?)
    }
    pub fn open_with_options(path: impl AsRef<Path>, options: OpenOptions) -> Result<Self, Error> {
        Self::from_reader_with_options(File::open(path)?, options)
    }
}

impl<R: Read + Seek> Container<R> {
    pub fn from_reader(reader: R) -> Result<Self, Error> {
        Self::from_reader_with_options(reader, OpenOptions::default())
    }
    pub fn from_reader_with_options(reader: R, options: OpenOptions) -> Result<Self, Error> {
        let mut archive = ZipArchive::new(reader)?;
        if archive.len() > options.max_members {
            return Err(Error::LimitExceeded {
                resource: "ZIP members",
                limit: options.max_members as u64,
            });
        }
        let version = Version::parse(&required(
            &mut archive,
            "version.txt",
            options.max_bootstrap_bytes,
        )?)?;
        version.profile()?;
        let description = member(
            &mut archive,
            "container.description",
            options.max_bootstrap_bytes,
        )?;
        let description = description.as_deref().map(identity).transpose()?;
        // The base specification puts the ARN at offset zero of the comment.
        // Canonical base images terminate it with NUL; trailing comment data is not identity.
        let comment = archive
            .comment()
            .split(|b| *b == 0)
            .next()
            .unwrap_or_default();
        let comment = if comment.starts_with(b"aff4://") {
            Some(identity(comment)?)
        } else {
            None
        };
        let volume_iri = match (description, comment) {
            (Some(a), Some(b)) if a != b => return Err(Error::ConflictingIdentity),
            (Some(a), _) | (_, Some(a)) => a,
            _ => {
                return Err(Error::InvalidIdentity(
                    "neither description nor comment contains an AFF4 ARN".into(),
                ));
            }
        };
        let turtle = required(
            &mut archive,
            "information.turtle",
            options.max_metadata_bytes,
        )?;
        let metadata = Metadata::parse(&turtle, &volume_iri, options.max_metadata_triples)?;
        Ok(Self {
            archive,
            volume_iri,
            version,
            metadata,
        })
    }
    pub fn volume_iri(&self) -> &str {
        &self.volume_iri
    }
    pub fn version(&self) -> &Version {
        &self.version
    }
    pub fn metadata(&self) -> &Metadata {
        &self.metadata
    }
    pub fn images(&self) -> impl Iterator<Item = &str> {
        self.metadata.subjects_of_type(AFF4_IMAGE)
    }
    pub fn member_names(&self) -> impl Iterator<Item = &str> {
        self.archive.file_names()
    }
    /// Consume the handle and return the underlying source. No files are extracted.
    pub fn into_inner(self) -> R {
        self.archive.into_inner()
    }
}

fn identity(bytes: &[u8]) -> Result<String, Error> {
    let text = std::str::from_utf8(bytes)
        .map_err(|_| Error::InvalidIdentity("not UTF-8".into()))?
        .trim_end_matches(['\r', '\n']);
    let uuid = text
        .strip_prefix("aff4://")
        .ok_or_else(|| Error::InvalidIdentity("expected aff4:// UUID".into()))?;
    if uuid.len() != 36
        || !uuid.bytes().enumerate().all(|(i, b)| {
            if [8, 13, 18, 23].contains(&i) {
                b == b'-'
            } else {
                b.is_ascii_hexdigit()
            }
        })
    {
        return Err(Error::InvalidIdentity("expected a UUID volume ARN".into()));
    }
    Ok(text.to_owned())
}

fn required<R: Read + Seek>(
    archive: &mut ZipArchive<R>,
    name: &'static str,
    limit: u64,
) -> Result<Vec<u8>, Error> {
    member(archive, name, limit)?.ok_or(Error::MissingMember(name))
}
fn member<R: Read + Seek>(
    archive: &mut ZipArchive<R>,
    name: &'static str,
    limit: u64,
) -> Result<Option<Vec<u8>>, Error> {
    let file = match archive.by_name(name) {
        Ok(file) => file,
        Err(ZipError::FileNotFound) => return Ok(None),
        Err(error) => return Err(error.into()),
    };
    if file.size() > limit {
        return Err(Error::LimitExceeded {
            resource: name,
            limit,
        });
    }
    let mut bytes = Vec::new();
    file.take(limit.saturating_add(1)).read_to_end(&mut bytes)?;
    if bytes.len() as u64 > limit {
        return Err(Error::LimitExceeded {
            resource: name,
            limit,
        });
    }
    Ok(Some(bytes))
}
