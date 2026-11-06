//! Rust library for AFF4 forensic containers.
//!
//! The initial reader opens ZIP/ZIP64 volumes and inspects version, identity,
//! and primary Turtle metadata. Image content decoding, imported metadata,
//! writing and integrity verification are not implemented yet.
//! Opening a container does not establish conformance or authenticity.
//!
//! ```no_run
//! use aff4::{Container, Error};
//! # fn main() -> Result<(), Error> {
//! let container = Container::open("evidence.aff4")?;
//! for image in container.images() {
//!     println!("{image}");
//! }
//! # Ok(())
//! # }
//! ```

mod container;
mod error;
mod image_stream;
mod metadata;

pub use container::{Container, OpenOptions, Profile, Version};
pub use error::Error;
pub use image_stream::{ImageStream, StreamOptions};
pub use metadata::{AFF4_IMAGE, Metadata, RDF_TYPE, Subject, Term, Triple};
