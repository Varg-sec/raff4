use std::{error, fmt, io};

/// Failures opening or inspecting a container. Parsing does not authenticate it.
#[derive(Debug)]
#[non_exhaustive]
pub enum Error {
    Io(io::Error),
    Zip(zip::result::ZipError),
    MissingMember(&'static str),
    InvalidVersion(String),
    UnsupportedVersion { major: u32, minor: u32 },
    InvalidIdentity(String),
    ConflictingIdentity,
    InvalidMetadata(String),
    InvalidStream { urn: String, reason: String },
    UnsupportedCompression(String),
    LimitExceeded { resource: &'static str, limit: u64 },
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Io(e) => write!(f, "container I/O: {e}"),
            Self::Zip(e) => write!(f, "ZIP container: {e}"),
            Self::MissingMember(name) => write!(f, "missing required member: {name}"),
            Self::InvalidVersion(reason) => write!(f, "invalid version.txt: {reason}"),
            Self::UnsupportedVersion { major, minor } => {
                write!(f, "unsupported AFF4 version {major}.{minor}")
            }
            Self::InvalidIdentity(reason) => write!(f, "invalid volume identity: {reason}"),
            Self::ConflictingIdentity => {
                f.write_str("container.description and ZIP comment identify different volumes")
            }
            Self::InvalidMetadata(reason) => write!(f, "invalid information.turtle: {reason}"),
            Self::InvalidStream { urn, reason } => write!(f, "image stream {urn}: {reason}"),
            Self::UnsupportedCompression(method) => {
                write!(f, "unsupported image compression: {method}")
            }
            Self::LimitExceeded { resource, limit } => {
                write!(f, "{resource} exceeds configured limit {limit}")
            }
        }
    }
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            Self::Io(e) => Some(e),
            Self::Zip(e) => Some(e),
            _ => None,
        }
    }
}
impl From<io::Error> for Error {
    fn from(value: io::Error) -> Self {
        Self::Io(value)
    }
}
impl From<zip::result::ZipError> for Error {
    fn from(value: zip::result::ZipError) -> Self {
        Self::Zip(value)
    }
}
impl From<rio_turtle::TurtleError> for Error {
    fn from(value: rio_turtle::TurtleError) -> Self {
        Self::InvalidMetadata(value.to_string())
    }
}
