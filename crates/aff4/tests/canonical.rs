use aff4::{Container, Profile, Term};
use sha2::{Digest, Sha256};
use std::{
    fs::File,
    io::{Read, Seek, SeekFrom},
    path::PathBuf,
};

/// The independent oracle and exact source revision are recorded in the manifest.
/// Missing inputs fail an explicitly requested canonical run instead of passing.
#[test]
#[ignore = "requires the pinned reference checkout; see fixtures/README.md"]
fn canonical_base_metadata_matches_independent_oracle() {
    let root = PathBuf::from(
        std::env::var_os("AFF4_REFERENCE_IMAGES")
            .expect("set AFF4_REFERENCE_IMAGES to the reference checkout"),
    );
    let manifest: serde_json::Value =
        serde_json::from_str(include_str!("../../../fixtures/base-metadata.json")).unwrap();
    let fixtures = manifest["fixtures"].as_array().unwrap();
    assert_eq!(fixtures.len(), 6, "unexpected fixture coverage change");
    for fixture in fixtures {
        let path = root.join(fixture["path"].as_str().unwrap());
        let mut file = File::open(&path).unwrap_or_else(|e| panic!("{}: {e}", path.display()));
        let mut hasher = Sha256::new();
        let mut buffer = [0u8; 64 * 1024];
        loop {
            let length = file.read(&mut buffer).unwrap();
            if length == 0 {
                break;
            }
            hasher.update(&buffer[..length]);
        }
        assert_eq!(
            format!("{:x}", hasher.finalize()),
            fixture["sha256"].as_str().unwrap(),
            "fixture changed: {}",
            path.display()
        );
        file.seek(SeekFrom::Start(0)).unwrap();
        let container = Container::from_reader(file).unwrap();
        assert_eq!(container.volume_iri(), fixture["volume"].as_str().unwrap());
        assert_eq!(container.version().profile().unwrap(), Profile::Aff4V1);
        assert_eq!(
            container.metadata().len() as u64,
            fixture["triples"].as_u64().unwrap()
        );
        let images: Vec<_> = container.images().collect();
        let expected: Vec<_> = fixture["images"]
            .as_array()
            .unwrap()
            .iter()
            .map(|v| v.as_str().unwrap())
            .collect();
        assert_eq!(images, expected);
        for image in images {
            let values: Vec<_> = container
                .metadata()
                .values(image, "http://aff4.org/Schema#size")
                .map(|term| match term {
                    Term::Literal {
                        value,
                        datatype,
                        language: None,
                    } => {
                        assert_eq!(datatype, "http://www.w3.org/2001/XMLSchema#long");
                        value.as_str()
                    }
                    other => panic!("invalid size term: {other:?}"),
                })
                .collect();
            let expected: Vec<_> = fixture["image_sizes"][image]
                .as_array()
                .unwrap()
                .iter()
                .map(|v| v.as_str().unwrap())
                .collect();
            assert_eq!(values, expected);
        }
    }
}
