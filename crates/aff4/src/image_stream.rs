use crate::{Container, Error, Metadata, Profile, RDF_TYPE, Term};
use std::io::{self, Read, Seek, SeekFrom};
use zip::{CompressionMethod, ZipArchive};

const NS: &str = "http://aff4.org/Schema#";

/// Allocation limits for a stream. At most one index and one decoded chunk are
/// cached. Loading also needs one encoded chunk and a temporary decoded buffer.
#[derive(Clone, Copy, Debug)]
pub struct StreamOptions {
    pub max_chunk_bytes: u32,
    pub max_encoded_chunk_bytes: u32,
    pub max_index_bytes: u64,
}
impl Default for StreamOptions {
    fn default() -> Self {
        Self {
            max_chunk_bytes: 64 * 1024 * 1024,
            max_encoded_chunk_bytes: 64 * 1024 * 1024,
            max_index_bytes: 64 * 1024 * 1024,
        }
    }
}
#[derive(Clone, Copy)]
enum Codec {
    Stored,
    Deflate,
    Zlib,
    Snappy,
    Lz4,
}

/// Seekable bytes of an explicitly named `aff4:ImageStream`.
///
/// This borrows its container exclusively. It does not resolve Maps or verify
/// AFF4 content hashes. Stored ZIP bevies are accessed directly; ZIP-compressed
/// bevies require decoding their prefix on each chunk read.
pub struct ImageStream<'a, R: Read + Seek> {
    container: &'a mut Container<R>,
    urn: String,
    size: u64,
    chunk_size: u32,
    chunks_per_bevy: u64,
    codec: Codec,
    options: StreamOptions,
    position: u64,
    index: Option<(u64, String, Vec<u8>)>,
    chunk: Option<(u64, Vec<u8>)>,
}

impl<R: Read + Seek> Container<R> {
    pub fn open_image_stream(&mut self, urn: &str) -> Result<ImageStream<'_, R>, Error> {
        self.open_image_stream_with_options(urn, StreamOptions::default())
    }
    pub fn open_image_stream_with_options(
        &mut self,
        urn: &str,
        options: StreamOptions,
    ) -> Result<ImageStream<'_, R>, Error> {
        let metadata = self.metadata();
        if !metadata
            .values(urn, RDF_TYPE)
            .any(|t| matches!(t, Term::Iri(iri) if iri == "http://aff4.org/Schema#ImageStream"))
        {
            return Err(invalid(
                urn,
                "object is not explicitly typed aff4:ImageStream",
            ));
        }
        let size = integer(metadata, urn, "size")?;
        let chunk_size = integer(metadata, urn, "chunkSize")?;
        let chunks_per_bevy = integer(metadata, urn, "chunksInSegment")?;
        if chunk_size == 0 || chunks_per_bevy == 0 {
            return Err(invalid(
                urn,
                "chunkSize and chunksInSegment must be positive",
            ));
        }
        if chunk_size > u64::from(options.max_chunk_bytes) {
            return Err(Error::LimitExceeded {
                resource: "chunkSize",
                limit: options.max_chunk_bytes.into(),
            });
        }
        // Bound index allocation even before any payload is read.
        let index_bytes = size
            .div_ceil(chunk_size)
            .min(chunks_per_bevy)
            .checked_mul(12)
            .ok_or_else(|| invalid(urn, "index length overflows u64"))?;
        if index_bytes > options.max_index_bytes {
            return Err(Error::LimitExceeded {
                resource: "bevy index",
                limit: options.max_index_bytes,
            });
        }
        if let Some(value) = one(metadata, urn, "stored")? {
            if !matches!(value, Term::Iri(iri) if iri == self.volume_iri()) {
                return Err(invalid(
                    urn,
                    "stream is stored in another volume or has invalid stored metadata",
                ));
            }
        }
        let method = match one(metadata, urn, "compressionMethod")? {
            None => None,
            Some(Term::Iri(iri)) => Some(iri.as_str()),
            _ => return Err(invalid(urn, "compressionMethod must be an IRI")),
        };
        let codec = match method {
            None
            | Some(
                "http://aff4.org/Schema#NullCompressor"
                | "http://aff4.org/Schema#compression/stored",
            ) => Codec::Stored,
            Some("https://tools.ietf.org/html/rfc1951") => Codec::Deflate,
            Some("https://www.ietf.org/rfc/rfc1950.txt") => Codec::Zlib,
            Some("http://code.google.com/p/snappy/") => Codec::Snappy,
            Some("https://code.google.com/p/lz4/") => Codec::Lz4,
            Some(other) => return Err(Error::UnsupportedCompression(other.into())),
        };
        Ok(ImageStream {
            container: self,
            urn: urn.into(),
            size,
            chunk_size: chunk_size as u32,
            chunks_per_bevy,
            codec,
            options,
            position: 0,
            index: None,
            chunk: None,
        })
    }
}

fn invalid(urn: &str, reason: impl Into<String>) -> Error {
    Error::InvalidStream {
        urn: urn.into(),
        reason: reason.into(),
    }
}
fn one<'a>(metadata: &'a Metadata, urn: &str, property: &str) -> Result<Option<&'a Term>, Error> {
    // Query output borrows only graph storage; find directly to avoid tying its
    // lifetime to a temporary predicate string.
    let predicate = format!("{NS}{property}");
    let mut values = metadata
        .triples()
        .filter(|t| {
            matches!(&t.subject, crate::Subject::Iri(iri) if iri == urn) && t.predicate == predicate
        })
        .map(|t| &t.object);
    let value = values.next();
    if values.next().is_some() {
        return Err(invalid(urn, format!("multiple {property} values")));
    }
    Ok(value)
}
fn integer(metadata: &Metadata, urn: &str, property: &str) -> Result<u64, Error> {
    if let Some(Term::Literal {
        value,
        datatype,
        language: None,
    }) = one(metadata, urn, property)?
    {
        let kind = datatype.strip_prefix("http://www.w3.org/2001/XMLSchema#");
        if matches!(
            kind,
            Some(
                "integer"
                    | "nonNegativeInteger"
                    | "positiveInteger"
                    | "long"
                    | "int"
                    | "short"
                    | "byte"
                    | "unsignedLong"
                    | "unsignedInt"
                    | "unsignedShort"
                    | "unsignedByte"
            )
        ) {
            let lexical = value.trim().strip_prefix('+').unwrap_or(value.trim());
            if !lexical.is_empty() && lexical.bytes().all(|b| b.is_ascii_digit()) {
                if let Ok(number) = lexical.parse::<u64>() {
                    let max = match kind {
                        Some("long") => i64::MAX as u64,
                        Some("int") => i32::MAX as u64,
                        Some("short") => i16::MAX as u64,
                        Some("byte") => i8::MAX as u64,
                        Some("unsignedInt") => u32::MAX as u64,
                        Some("unsignedShort") => u16::MAX as u64,
                        Some("unsignedByte") => u8::MAX as u64,
                        _ => u64::MAX,
                    };
                    if number <= max && !(kind == Some("positiveInteger") && number == 0) {
                        return Ok(number);
                    }
                }
            }
        }
    }
    Err(invalid(
        urn,
        format!("{property} must be a nonnegative integer within its datatype and u64 range"),
    ))
}

impl<R: Read + Seek> ImageStream<'_, R> {
    pub fn len(&self) -> u64 {
        self.size
    }
    pub fn is_empty(&self) -> bool {
        self.size == 0
    }
    pub fn urn(&self) -> &str {
        &self.urn
    }

    fn member_name(&self, urn: &str) -> Result<String, Error> {
        if let Some(term) = one(self.container.metadata(), urn, "fileName")? {
            return match term {
                Term::Literal {
                    value,
                    language: None,
                    ..
                } => Ok(value.clone()),
                _ => Err(invalid(
                    &self.urn,
                    "explicit segment fileName must be a literal",
                )),
            };
        }
        let relative = urn
            .strip_prefix(self.container.volume_iri())
            .and_then(|suffix| suffix.strip_prefix('/'));
        let name = relative.unwrap_or(urn);
        if self.container.version().profile()? == Profile::Aff4LogicalV1 {
            return Ok(name.into());
        }
        let mut result = String::new();
        for (i, byte) in name.bytes().enumerate() {
            // Encode :// in a foreign resource's prefix, preserve path separators.
            if byte.is_ascii_alphanumeric()
                || b"-._~".contains(&byte)
                || (byte == b'/' && (relative.is_some() || i >= 7))
            {
                result.push(char::from(byte));
            } else {
                use std::fmt::Write;
                write!(result, "%{byte:02X}").expect("writing a String cannot fail");
            }
        }
        Ok(result)
    }

    fn load_index(&mut self, bevy: u64) -> Result<(), Error> {
        if self
            .index
            .as_ref()
            .is_some_and(|(cached, _, _)| *cached == bevy)
        {
            return Ok(());
        }
        let first_chunk = bevy
            .checked_mul(self.chunks_per_bevy)
            .ok_or_else(|| invalid(&self.urn, "bevy position overflow"))?;
        let total = self.size.div_ceil(self.chunk_size.into());
        let count = total
            .checked_sub(first_chunk)
            .ok_or_else(|| invalid(&self.urn, "bevy beyond image"))?
            .min(self.chunks_per_bevy);
        let expected = count
            .checked_mul(12)
            .ok_or_else(|| invalid(&self.urn, "index length overflow"))?;
        if expected > self.options.max_index_bytes {
            return Err(Error::LimitExceeded {
                resource: "bevy index",
                limit: self.options.max_index_bytes,
            });
        }
        let bevy_urn = format!("{}/{bevy:08}", self.urn.trim_end_matches('/'));
        let name = self.member_name(&bevy_urn)?;
        let index_name = self.member_name(&format!("{bevy_urn}.index"))?;
        let mut file = self
            .container
            .archive
            .by_name(&index_name)
            .map_err(|e| invalid(&self.urn, format!("{index_name}: {e}")))?;
        if file.size() != expected {
            return Err(invalid(
                &self.urn,
                format!(
                    "{index_name}: expected {expected} bytes, got {}",
                    file.size()
                ),
            ));
        }
        let mut bytes = Vec::new();
        (&mut file)
            .take(expected.saturating_add(1))
            .read_to_end(&mut bytes)?;
        if bytes.len() as u64 != expected {
            return Err(invalid(
                &self.urn,
                format!("{index_name}: truncated or oversized index"),
            ));
        }
        drop(file);
        let bevy_size = self
            .container
            .archive
            .by_name(&name)
            .map_err(|e| invalid(&self.urn, format!("{name}: {e}")))?
            .size();
        let mut end = 0;
        for entry in bytes.chunks_exact(12) {
            let (offset, length) = index_entry(entry);
            if length == 0 || length > self.options.max_encoded_chunk_bytes {
                return Err(invalid(
                    &self.urn,
                    format!("{index_name}: invalid or over-limit chunk length {length}"),
                ));
            }
            if offset < end {
                return Err(invalid(
                    &self.urn,
                    format!("{index_name}: overlapping or unordered chunk offsets"),
                ));
            }
            end = offset.checked_add(length.into()).ok_or_else(|| {
                invalid(&self.urn, format!("{index_name}: chunk offset overflow"))
            })?;
            if end > bevy_size {
                return Err(invalid(
                    &self.urn,
                    format!("{index_name}: chunk exceeds bevy length"),
                ));
            }
        }
        self.index = Some((bevy, name, bytes));
        Ok(())
    }

    fn load_chunk(&mut self, chunk: u64) -> Result<(), Error> {
        if self
            .chunk
            .as_ref()
            .is_some_and(|(cached, _)| *cached == chunk)
        {
            return Ok(());
        }
        self.load_index(chunk / self.chunks_per_bevy)?;
        let (_, name, index) = self.index.as_ref().expect("index loaded");
        let entry_offset = usize::try_from(
            (chunk % self.chunks_per_bevy)
                .checked_mul(12)
                .ok_or_else(|| invalid(&self.urn, "index position overflow"))?,
        )
        .map_err(|_| invalid(&self.urn, "index position exceeds usize"))?;
        let entry = index
            .get(
                entry_offset
                    ..entry_offset
                        .checked_add(12)
                        .ok_or_else(|| invalid(&self.urn, "index position overflow"))?,
            )
            .ok_or_else(|| invalid(&self.urn, "missing chunk index entry"))?;
        let (offset, length) = index_entry(entry);
        let encoded = read_range(&mut self.container.archive, name, offset, length)
            .map_err(|e| invalid(&self.urn, format!("{name}, chunk {chunk}: {e}")))?;
        let decoded = decode(self.codec, &encoded, self.chunk_size as usize)
            .map_err(|reason| invalid(&self.urn, format!("{name}, chunk {chunk}: {reason}")))?;
        self.chunk = Some((chunk, decoded));
        Ok(())
    }
}
fn index_entry(entry: &[u8]) -> (u64, u32) {
    (
        u64::from_le_bytes(entry[..8].try_into().expect("validated 12-byte entry")),
        u32::from_le_bytes(entry[8..12].try_into().expect("validated 12-byte entry")),
    )
}
fn read_range<R: Read + Seek>(
    archive: &mut ZipArchive<R>,
    name: &str,
    offset: u64,
    length: u32,
) -> Result<Vec<u8>, Error> {
    let stored = archive.by_name(name)?.compression() == CompressionMethod::Stored;
    let mut bytes = vec![0; length as usize];
    if stored {
        let mut file = archive.by_name_seek(name)?;
        file.seek(SeekFrom::Start(offset))?;
        file.read_exact(&mut bytes)?;
    } else {
        let mut file = archive.by_name(name)?;
        let skipped = io::copy(&mut (&mut file).take(offset), &mut io::sink())?;
        if skipped != offset {
            return Err(io::Error::from(io::ErrorKind::UnexpectedEof).into());
        }
        file.read_exact(&mut bytes)?;
    }
    Ok(bytes)
}
fn decode(codec: Codec, encoded: &[u8], size: usize) -> Result<Vec<u8>, String> {
    // AFF4 uses the encoded length to identify stored chunks even when the
    // stream selects a codec. The final chunk is padded to chunkSize.
    if encoded.len() == size {
        return Ok(encoded.to_vec());
    }
    let mut output = vec![0; size];
    let length = match codec {
        Codec::Stored => return Err("stored chunk length differs from chunkSize".into()),
        Codec::Snappy => {
            if snap::raw::decompress_len(encoded).map_err(|e| e.to_string())? != size {
                return Err("Snappy output length differs from chunkSize".into());
            }
            snap::raw::Decoder::new()
                .decompress(encoded, &mut output)
                .map_err(|e| e.to_string())?
        }
        Codec::Lz4 => {
            lz4_flex::block::decompress_into(encoded, &mut output).map_err(|e| e.to_string())?
        }
        Codec::Deflate | Codec::Zlib => {
            let mut decoder = flate2::Decompress::new(matches!(codec, Codec::Zlib));
            let status = decoder
                .decompress(encoded, &mut output, flate2::FlushDecompress::Finish)
                .map_err(|e| e.to_string())?;
            if status != flate2::Status::StreamEnd || decoder.total_in() != encoded.len() as u64 {
                return Err("incomplete, oversized or trailing compressed data".into());
            }
            decoder.total_out() as usize
        }
    };
    if length != size {
        return Err("decoded chunk length differs from chunkSize".into());
    }
    Ok(output)
}
impl<R: Read + Seek> Read for ImageStream<'_, R> {
    fn read(&mut self, buffer: &mut [u8]) -> io::Result<usize> {
        if buffer.is_empty() || self.position >= self.size {
            return Ok(0);
        }
        // Return at most one chunk per call: if the next chunk is corrupt, a
        // later call returns an error without concealing already delivered bytes.
        let number = self.position / u64::from(self.chunk_size);
        self.load_chunk(number)
            .map_err(|error| io::Error::new(io::ErrorKind::InvalidData, error))?;
        let start = (self.position % u64::from(self.chunk_size)) as usize;
        let available = (u64::from(self.chunk_size) - start as u64).min(self.size - self.position);
        let count = buffer.len().min(available as usize);
        let data = &self.chunk.as_ref().expect("chunk loaded").1;
        buffer[..count].copy_from_slice(&data[start..start + count]);
        self.position += count as u64;
        Ok(count)
    }
}
impl<R: Read + Seek> Seek for ImageStream<'_, R> {
    fn seek(&mut self, position: SeekFrom) -> io::Result<u64> {
        let target = match position {
            SeekFrom::Start(offset) => i128::from(offset),
            SeekFrom::End(offset) => i128::from(self.size) + i128::from(offset),
            SeekFrom::Current(offset) => i128::from(self.position) + i128::from(offset),
        };
        let target = u64::try_from(target).map_err(|_| {
            io::Error::new(
                io::ErrorKind::InvalidInput,
                "seek would exceed u64 or precede start",
            )
        })?;
        self.position = target;
        Ok(target)
    }
}
