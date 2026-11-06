use aff4::{Container, Error, OpenOptions, Profile, Subject, Term};
use std::io::{Cursor, Write};
use zip::{CompressionMethod, ZipWriter, write::SimpleFileOptions};

const VOLUME: &str = "aff4://00000000-0000-0000-0000-000000000001";
const IMAGE: &str = "aff4://00000000-0000-0000-0000-000000000002";
const TURTLE: &str = r#"
@prefix aff4: <http://aff4.org/Schema#> .
@prefix xsd: <http://www.w3.org/2001/XMLSchema#> .
<aff4://00000000-0000-0000-0000-000000000002>
    a aff4:Image, aff4:DiskImage ; aff4:size "00042"^^xsd:long ;
    <https://vendor.example/note> "Grüße"@DE ;
    <https://vendor.example/unknown> [ <https://vendor.example/key> "value" ] .
<> a aff4:ZipVolume .
"#;

fn archive(members: &[(&str, &[u8])], comment: &str, method: CompressionMethod) -> Vec<u8> {
    let mut writer = ZipWriter::new(Cursor::new(Vec::new()));
    writer.set_comment(comment);
    for (name, bytes) in members {
        writer
            .start_file(
                *name,
                SimpleFileOptions::default()
                    .compression_method(method)
                    .large_file(true),
            )
            .unwrap();
        writer.write_all(bytes).unwrap();
    }
    writer.finish().unwrap().into_inner()
}
fn fixture(version: &[u8], metadata: &[u8]) -> Vec<u8> {
    archive(
        &[
            ("container.description", VOLUME.as_bytes()),
            ("version.txt", version),
            ("information.turtle", metadata),
        ],
        VOLUME,
        CompressionMethod::Deflated,
    )
}
fn read(bytes: Vec<u8>) -> Result<Container<Cursor<Vec<u8>>>, Error> {
    Container::from_reader(Cursor::new(bytes))
}

#[test]
fn inspect_zip64_metadata_through_public_api() {
    let bytes = fixture(
        b"major=1\nminor=0\ntool=test writer\nvendor=data=a\n",
        TURTLE.as_bytes(),
    );
    let container = read(bytes).unwrap();
    assert_eq!(container.volume_iri(), VOLUME);
    assert_eq!(container.version().profile().unwrap(), Profile::Aff4V1);
    assert_eq!(container.version().tool.as_deref(), Some("test writer"));
    assert_eq!(container.version().extensions["vendor"], "data=a");
    assert_eq!(container.images().collect::<Vec<_>>(), vec![IMAGE]);
    assert_eq!(
        container
            .metadata()
            .values(IMAGE, "http://aff4.org/Schema#size")
            .collect::<Vec<_>>(),
        vec![&Term::Literal {
            value: "00042".into(),
            datatype: "http://www.w3.org/2001/XMLSchema#long".into(),
            language: None
        }]
    );
    assert!(
        matches!(container.metadata().values(IMAGE, "https://vendor.example/note").next(), Some(Term::Literal { value, language: Some(lang), .. }) if value == "Grüße" && lang == "de")
    );
    assert!(
        container
            .metadata()
            .triples()
            .any(|t| matches!(t.subject, Subject::BlankNode(_)))
    );
    assert_eq!(
        container
            .metadata()
            .subjects_of_type("http://aff4.org/Schema#ZipVolume")
            .collect::<Vec<_>>(),
        vec![VOLUME]
    );
    assert!(container.member_names().any(|name| name == "version.txt"));
    assert!(!container.into_inner().into_inner().is_empty());
}

#[test]
fn recognizes_logical_profile_without_claiming_stream_support() {
    let reader = read(fixture(b"major=2\r\nminor=1\r\n", TURTLE.as_bytes())).unwrap();
    assert_eq!(reader.version().profile().unwrap(), Profile::Aff4LogicalV1);
}

#[test]
fn accepts_comment_only_identity_and_nul_termination() {
    let bytes = archive(
        &[
            ("version.txt", b"major=1\nminor=0"),
            ("information.turtle", TURTLE.as_bytes()),
        ],
        &format!("{VOLUME}\0comment data"),
        CompressionMethod::Stored,
    );
    assert_eq!(read(bytes).unwrap().volume_iri(), VOLUME);
}

#[test]
fn accepts_description_only_identity() {
    let bytes = archive(
        &[
            ("container.description", format!("{VOLUME}\n").as_bytes()),
            ("version.txt", b"major=1\nminor=0"),
            ("information.turtle", b""),
        ],
        "ordinary ZIP comment",
        CompressionMethod::Stored,
    );
    assert_eq!(read(bytes).unwrap().volume_iri(), VOLUME);
}

#[test]
fn rejects_conflicting_or_malformed_identity() {
    let bytes = archive(
        &[
            ("container.description", VOLUME.as_bytes()),
            ("version.txt", b"major=1\nminor=0"),
            ("information.turtle", b""),
        ],
        IMAGE,
        CompressionMethod::Stored,
    );
    assert!(matches!(read(bytes), Err(Error::ConflictingIdentity)));
    for identity in [b"not-an-arn".as_slice(), b"aff4://bad", b"\xff"] {
        let bytes = archive(
            &[
                ("container.description", identity),
                ("version.txt", b"major=1\nminor=0"),
                ("information.turtle", b""),
            ],
            "",
            CompressionMethod::Stored,
        );
        assert!(matches!(read(bytes), Err(Error::InvalidIdentity(_))));
    }
}

#[test]
fn rejects_bad_zip_and_missing_required_members() {
    assert!(matches!(read(b"not a zip".to_vec()), Err(Error::Zip(_))));
    let bytes = archive(&[], VOLUME, CompressionMethod::Stored);
    assert!(matches!(
        read(bytes),
        Err(Error::MissingMember("version.txt"))
    ));
    let bytes = archive(
        &[("version.txt", b"major=1\nminor=0")],
        VOLUME,
        CompressionMethod::Stored,
    );
    assert!(matches!(
        read(bytes),
        Err(Error::MissingMember("information.turtle"))
    ));
}

#[test]
fn rejects_invalid_and_unsupported_versions() {
    for version in [
        "major=1",
        "major=1\nminor=0\nmajor=2",
        "major=-1\nminor=0",
        "major=4294967296\nminor=0",
        "major=1\nminor=no",
        "major=1\nminor=0\ninvalid",
    ] {
        assert!(
            matches!(
                read(fixture(version.as_bytes(), b"")),
                Err(Error::InvalidVersion(_))
            ),
            "{version}"
        );
    }
    for (major, minor) in [(1, 1), (2, 0), (3, 0)] {
        assert!(matches!(
            read(fixture(
                format!("major={major}\nminor={minor}").as_bytes(),
                b""
            )),
            Err(Error::UnsupportedVersion { .. })
        ));
    }
    assert!(matches!(
        read(fixture(b"\xff", b"")),
        Err(Error::InvalidVersion(_))
    ));
}

#[test]
fn rejects_malformed_turtle_and_rdf_star() {
    for turtle in [
        b"not turtle".as_slice(),
        b"\xff",
        b"<< <urn:a> <urn:b> <urn:c> >> <urn:d> <urn:e> .",
    ] {
        assert!(matches!(
            read(fixture(b"major=1\nminor=0", turtle)),
            Err(Error::InvalidMetadata(_))
        ));
    }
}

#[test]
fn enforces_configured_limits() {
    for options in [
        OpenOptions {
            max_members: 2,
            ..OpenOptions::default()
        },
        OpenOptions {
            max_bootstrap_bytes: 3,
            ..OpenOptions::default()
        },
        OpenOptions {
            max_metadata_bytes: 3,
            ..OpenOptions::default()
        },
        OpenOptions {
            max_metadata_triples: 1,
            ..OpenOptions::default()
        },
    ] {
        let bytes = fixture(b"major=1\nminor=0", TURTLE.as_bytes());
        assert!(matches!(
            Container::from_reader_with_options(Cursor::new(bytes), options),
            Err(Error::LimitExceeded { .. })
        ));
    }
}

#[test]
fn deduplicates_graph_but_counts_duplicate_input_toward_limits() {
    let triple = format!("<{IMAGE}> a <http://aff4.org/Schema#Image> .\n");
    let bytes = fixture(b"major=1\nminor=0", triple.repeat(2).as_bytes());
    let container = read(bytes.clone()).unwrap();
    assert_eq!(container.metadata().len(), 1);
    let options = OpenOptions {
        max_metadata_triples: 1,
        ..OpenOptions::default()
    };
    assert!(matches!(
        Container::from_reader_with_options(Cursor::new(bytes), options),
        Err(Error::LimitExceeded { .. })
    ));
}

#[test]
fn corrupt_metadata_crc_is_not_accepted() {
    let mut bytes = archive(
        &[
            ("version.txt", b"major=1\nminor=0"),
            ("information.turtle", b"<urn:a> <urn:b> <urn:c> ."),
        ],
        VOLUME,
        CompressionMethod::Stored,
    );
    let offset = bytes.windows(7).position(|w| w == b"<urn:a>").unwrap();
    bytes[offset + 5] = b'z'; // Still valid Turtle, but the ZIP CRC no longer matches.
    assert!(matches!(read(bytes), Err(Error::Io(_))));
}
