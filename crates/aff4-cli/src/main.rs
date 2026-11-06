//! Inspect an AFF4 container's identity, version and primary metadata.
//!
//! This is an inspection tool, not a verifier: opening a container does not
//! establish its integrity or conformance.

use aff4::{Container, Subject, Term};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path = std::env::args_os()
        .nth(1)
        .ok_or("usage: aff4 <container.aff4>")?;
    let container = Container::open(path)?;
    println!("Volume: {}", container.volume_iri());
    println!(
        "Version: {}.{}",
        container.version().major,
        container.version().minor
    );
    println!("Primary metadata triples: {}", container.metadata().len());
    for image in container.images() {
        println!("Image: {image}");
    }
    for triple in container.metadata().triples() {
        if let (Subject::Iri(subject), Term::Literal { value, .. }) =
            (&triple.subject, &triple.object)
        {
            if triple.predicate == "http://aff4.org/Schema#size" {
                println!("Size: {subject} = {value}");
            }
        }
    }
    Ok(())
}
