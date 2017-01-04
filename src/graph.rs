pub trait Graph {
    type SPOTriple: Triple;
    type SPOIter: Iterator<Item = Self::SPOTriple>;
    type SPORangeIter: Iterator<Item = Self::SPOTriple>;
    fn iter(&self) -> Self::SPOIter;

    /// iterator over all triples with the same subject and predicate
    fn iter_subject_predicate(&self, subject: &Subject, predicate: &str) -> Self::SPORangeIter;
    /// iterator that returns no results
    fn empty_spo_range(&self) -> Self::SPORangeIter;

    /// return the number of triples in the graph
    fn len(&self) -> usize;
}

pub trait GraphCreator {
    type Graph: Graph;
    fn create_blank_node(&mut self) -> BlankNode;
    fn add_triple<T>(&mut self, triple: &T) where T: Triple;
    fn add<'b, S, O>(&mut self, subject: S, predicate: &str, object: O)
        where S: IntoSubject<'b>,
              O: IntoObject<'b>;
    fn collect(&mut self) -> Self::Graph;
}

pub type BlankNode = (usize, usize);

pub trait Triple: Eq {
    fn subject(&self) -> Subject;
    fn predicate(&self) -> &str;
    fn object(&self) -> Object;
    fn eq<Rhs>(&self, other: &Rhs) -> bool
        where Rhs: Triple
    {
        self.subject().eq(&other.subject()) && self.predicate().eq(other.predicate()) &&
        self.object().eq(&other.object())
    }
}

#[derive(PartialEq,Eq,Hash,Clone,Copy,PartialOrd,Ord,Debug)]
pub enum Subject<'a> {
    IRI(&'a str),
    BlankNode(BlankNode),
}

#[derive(PartialEq,Eq,Hash,Clone,PartialOrd,Ord,Debug)]
pub struct Literal<'a> {
    pub lexical: &'a str,
    pub datatype: &'a str,
    pub language: Option<&'a str>,
}

#[derive(PartialEq,Eq,Hash,Clone,PartialOrd,Ord,Debug)]
pub enum Object<'a> {
    IRI(&'a str),
    BlankNode(BlankNode),
    Literal(Literal<'a>),
}

pub trait IntoSubject<'a> {
    fn subject(self) -> Subject<'a>;
}
pub trait IntoObject<'a> {
    fn object(self) -> Object<'a>;
}

impl<'a> IntoSubject<'a> for Subject<'a> {
    fn subject(self) -> Subject<'a> {
        self
    }
}
impl<'a> IntoSubject<'a> for &'a str {
    fn subject(self) -> Subject<'a> {
        Subject::IRI(self)
    }
}
impl<'a> IntoSubject<'a> for BlankNode {
    fn subject(self) -> Subject<'a> {
        Subject::BlankNode(self)
    }
}
impl<'a> IntoObject<'a> for Object<'a> {
    fn object(self) -> Object<'a> {
        self
    }
}
impl<'a> IntoObject<'a> for &'a str {
    fn object(self) -> Object<'a> {
        Object::IRI(self)
    }
}
impl<'a> IntoObject<'a> for BlankNode {
    fn object(self) -> Object<'a> {
        Object::BlankNode(self)
    }
}
impl<'a> IntoObject<'a> for Literal<'a> {
    fn object(self) -> Object<'a> {
        Object::Literal(self)
    }
}
impl<'a> IntoObject<'a> for Subject<'a> {
    fn object(self) -> Object<'a> {
        Subject::into(self)
    }
}

pub struct SubjectClone {
    iri: String,
    subject: SubjectCloneEnum,
}

enum SubjectCloneEnum {
    IRI,
    BlankNode(BlankNode),
}

impl SubjectClone {
    pub fn new() -> SubjectClone {
        SubjectClone {
            iri: String::new(),
            subject: SubjectCloneEnum::IRI,
        }
    }
    pub fn assign(&mut self, s: &Subject) {
        self.iri.clear();
        match s {
            &Subject::IRI(iri) => {
                self.iri.push_str(iri);
                self.subject = SubjectCloneEnum::IRI
            }
            &Subject::BlankNode(b) => self.subject = SubjectCloneEnum::BlankNode(b),
        };
    }
}

impl<'a> PartialEq<Subject<'a>> for SubjectClone {
    fn eq(&self, s: &Subject) -> bool {
        match (&self.subject, s) {
            (&SubjectCloneEnum::IRI, &Subject::IRI(iri)) => self.iri == iri,
            (&SubjectCloneEnum::BlankNode(b1), &Subject::BlankNode(b2)) => b1 == b2,
            _ => false,
        }
    }
}
impl<'a> From<Subject<'a>> for SubjectClone {
    fn from(s: Subject<'a>) -> SubjectClone {
        match s {
            Subject::IRI(iri) => {
                SubjectClone {
                    iri: String::from(iri),
                    subject: SubjectCloneEnum::IRI,
                }
            }
            Subject::BlankNode(b) => {
                SubjectClone {
                    iri: String::new(),
                    subject: SubjectCloneEnum::BlankNode(b),
                }
            }
        }
    }
}

impl<'a> From<Subject<'a>> for Object<'a> {
    fn from(s: Subject<'a>) -> Object<'a> {
        match s {
            Subject::IRI(iri) => Object::IRI(iri),
            Subject::BlankNode(b) => Object::BlankNode(b),
        }
    }
}
