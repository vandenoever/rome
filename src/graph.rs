pub trait Graph<'g> {
    type SubjectPtr: SubjectPtr<'g>;
    type PredicatePtr: PredicatePtr<'g>;
    type ObjectPtr: ObjectPtr<'g>;
    type SPOTriple: Triple<'g, SubjectPtr = Self::SubjectPtr,
           PredicatePtr = Self::PredicatePtr,
           ObjectPtr = Self::ObjectPtr>;
    type SPOIter: Iterator<Item = Self::SPOTriple>;
    type SPORangeIter: Iterator<Item = Self::SPOTriple>;
    type OPSTriple: Triple<'g, SubjectPtr = Self::SubjectPtr,
           PredicatePtr = Self::PredicatePtr,
           ObjectPtr = Self::ObjectPtr>;
    type OPSRangeIter: Iterator<Item = Self::OPSTriple>;
    fn iter(&'g self) -> Self::SPOIter;

    fn subject_ptr<'a, S>(&'g self, subject: S) -> Option<Self::SubjectPtr> where S: IntoSubject<'a>;
    fn predicate_ptr<'a>(&'g self, predicate: &'a str) -> Option<Self::PredicatePtr>;
    fn object_ptr<'a, O>(&'g self, object: O) -> Option<Self::ObjectPtr> where O: IntoObject<'a>;
    fn object_to_subject(&'g self, object: Self::ObjectPtr) -> Option<Self::SubjectPtr>;
    fn object_to_predicate(&'g self, object: Self::ObjectPtr) -> Option<Self::PredicatePtr>;
    fn subject_to_object(&'g self, subject: Self::SubjectPtr) -> Self::ObjectPtr;
    fn predicate_to_object(&'g self, predicate: Self::PredicatePtr) -> Self::ObjectPtr;

    fn iter_s_p(&'g self,
                subject: Self::SubjectPtr,
                predicate: Self::PredicatePtr)
                -> Self::SPORangeIter;
    fn iter_o_p(&'g self,
                object: Self::ObjectPtr,
                predicate: Self::PredicatePtr)
                -> Self::OPSRangeIter;

    /// iterator over all triples with the same subject and predicate
    fn iter_subject_predicate(&'g self, subject: &Subject, predicate: &str) -> Self::SPORangeIter;
    /// iterator that returns no results
    fn empty_spo_range(&'g self) -> Self::SPORangeIter;
    /// iterator that returns no results
    fn empty_ops_range(&'g self) -> Self::OPSRangeIter;

    /// return the number of triples in the graph
    fn len(&self) -> usize;
}

pub trait GraphCreator<'g> {
    type Graph: Graph<'g>;
    fn create_blank_node(&mut self) -> BlankNode;
    fn add_triple<'t,T>(&mut self, triple: &T) where T: Triple<'t>;
    fn add<'a:'b,'b, S, O>(&'a mut self, subject: S, predicate: &str, object: O)
        where S: IntoSubject<'b>,
              O: IntoObject<'b>;
    fn collect(&mut self) -> Self::Graph;
}

pub type BlankNode = (usize, usize);

pub trait Triple<'g>: Eq {
    type SubjectPtr: SubjectPtr<'g>;
    type PredicatePtr: PredicatePtr<'g>;
    type ObjectPtr: ObjectPtr<'g>;
    fn subject(&self) -> Subject;
    fn subject_ptr(&self) -> Self::SubjectPtr;
    fn predicate(&self) -> &str;
    fn object(&self) -> Object;
    fn object_ptr(&self) -> Self::ObjectPtr;
    fn eq<'a, Rhs>(&self, other: &Rhs) -> bool
        where Rhs: Triple<'a>
    {
        self.subject().eq(&other.subject()) && self.predicate().eq(other.predicate()) &&
        self.object().eq(&other.object())
    }
}

pub trait PredicatePtr<'a>: Eq + Clone {
    fn iri(&'a self) -> &'a str;
}
pub trait SubjectPtr<'g>: Eq + Clone {
    fn iri(&self) -> Option<&'g str>;
}
pub trait ObjectPtr<'g>: SubjectPtr<'g> + Ord + IntoObject<'g> {
    fn literal(&self) -> Option<&str>;
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
