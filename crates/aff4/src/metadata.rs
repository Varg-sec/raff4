use crate::Error;
use rio_api::{model, parser::TriplesParser};
use rio_turtle::TurtleParser;
use std::collections::BTreeSet;

pub const RDF_TYPE: &str = "http://www.w3.org/1999/02/22-rdf-syntax-ns#type";
pub const AFF4_IMAGE: &str = "http://aff4.org/Schema#Image";

/// RDF subject. Blank node identifiers are local to this metadata graph.
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum Subject {
    Iri(String),
    BlankNode(String),
}

/// Owned RDF term. Literal lexical values and datatype IRIs are preserved.
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum Term {
    Iri(String),
    BlankNode(String),
    Literal {
        value: String,
        datatype: String,
        language: Option<String>,
    },
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct Triple {
    pub subject: Subject,
    pub predicate: String,
    pub object: Term,
}

/// Primary metadata graph; imports are retained as triples, not fetched.
#[derive(Debug)]
pub struct Metadata {
    triples: BTreeSet<Triple>,
}

impl Metadata {
    pub(crate) fn parse(bytes: &[u8], volume: &str, limit: usize) -> Result<Self, Error> {
        let base = volume
            .parse()
            .map_err(|_| Error::InvalidIdentity("invalid base IRI".into()))?;
        let mut triples = BTreeSet::new();
        let mut count = 0usize;
        TurtleParser::new(bytes, Some(base)).parse_all(&mut |triple| -> Result<(), Error> {
            if count >= limit {
                return Err(Error::LimitExceeded {
                    resource: "metadata triples",
                    limit: limit as u64,
                });
            }
            count += 1;
            let subject = match triple.subject {
                model::Subject::NamedNode(n) => Subject::Iri(n.iri.into()),
                model::Subject::BlankNode(n) => Subject::BlankNode(n.id.into()),
                model::Subject::Triple(_) => {
                    return Err(Error::InvalidMetadata(
                        "RDF-star subjects are unsupported".into(),
                    ));
                }
            };
            let object = match triple.object {
                model::Term::NamedNode(n) => Term::Iri(n.iri.into()),
                model::Term::BlankNode(n) => Term::BlankNode(n.id.into()),
                model::Term::Literal(literal) => match literal {
                    model::Literal::Simple { value } => Term::Literal {
                        value: value.into(),
                        datatype: "http://www.w3.org/2001/XMLSchema#string".into(),
                        language: None,
                    },
                    model::Literal::Typed { value, datatype } => Term::Literal {
                        value: value.into(),
                        datatype: datatype.iri.into(),
                        language: None,
                    },
                    model::Literal::LanguageTaggedString { value, language } => Term::Literal {
                        value: value.into(),
                        datatype: "http://www.w3.org/1999/02/22-rdf-syntax-ns#langString".into(),
                        language: Some(language.to_ascii_lowercase()),
                    },
                },
                model::Term::Triple(_) => {
                    return Err(Error::InvalidMetadata(
                        "RDF-star objects are unsupported".into(),
                    ));
                }
            };
            triples.insert(Triple {
                subject,
                predicate: triple.predicate.iri.into(),
                object,
            });
            Ok(())
        })?;
        Ok(Self { triples })
    }

    pub fn triples(&self) -> impl Iterator<Item = &Triple> {
        self.triples.iter()
    }
    pub fn len(&self) -> usize {
        self.triples.len()
    }
    pub fn is_empty(&self) -> bool {
        self.triples.is_empty()
    }

    /// Query exact IRIs; namespace aliases are never silently substituted.
    pub fn values<'a>(
        &'a self,
        subject: &'a str,
        predicate: &'a str,
    ) -> impl Iterator<Item = &'a Term> {
        self.triples.iter().filter_map(move |t| {
            if matches!(&t.subject, Subject::Iri(iri) if iri == subject) && t.predicate == predicate
            {
                Some(&t.object)
            } else {
                None
            }
        })
    }

    /// Enumerate named subjects explicitly assigned a type, without inference.
    pub fn subjects_of_type<'a>(&'a self, class: &'a str) -> impl Iterator<Item = &'a str> {
        self.triples.iter().filter_map(move |t| {
            if t.predicate == RDF_TYPE && matches!(&t.object, Term::Iri(iri) if iri == class) {
                if let Subject::Iri(iri) = &t.subject {
                    return Some(iri.as_str());
                }
            }
            None
        })
    }
}
