
pub enum TripleObjectType {
    BlankNode,
    IRI,
    Literal,
    LiteralLang,
}

pub trait CompactTriple<T> {
    fn triple(subject_is_iri: bool,
              subject: T,
              predicate: T,
              object_type: TripleObjectType,
              object: T,
              datatype_or_lang: T)
              -> Self;
    fn max_subject_value() -> T;
    fn max_datatype_value() -> T;
    fn subject_is_iri(&self) -> bool;
    fn object_is_blank_node(&self) -> bool;
    fn object_is_iri(&self) -> bool;
    fn has_language(&self) -> bool;
    fn subject(&self) -> T;
    fn predicate(&self) -> T;
    fn object(&self) -> T;
    fn datatype_or_lang(&self) -> T;
    fn set_subject(&mut self, subject: T);
    fn set_predicate(&mut self, predicate: T);
    fn set_object(&mut self, object: T);
    fn set_datatype_or_lang(&mut self, datatype_or_lang: T);
}

/// 1 bit to determine blank or iri
/// 18 bits for subject id
/// 18 bits for predicate id
/// 1 bit to determine non-literal or literal
/// 1 bit to determine blank or iri (for non-literal)
///          or has lang (0) or has no lang (1)
/// 18 bits for object id
///  7 bits for for datatype or language id
#[derive(PartialEq,Eq,Hash,PartialOrd,Ord)]
pub struct Triple64 {
    value: u64,
}

impl CompactTriple<u32> for Triple64 {
    fn max_subject_value() -> u32 {
        2 ^ 18
    }
    fn max_datatype_value() -> u32 {
        2 ^ 7
    }
    fn triple(subject_is_iri: bool,
              subject: u32,
              predicate: u32,
              object_type: TripleObjectType,
              object: u32,
              datatype_or_lang: u32)
              -> Triple64 {
        let mut val = (subject_is_iri as u64) << 63;
        val += (subject as u64) << 45;
        val += (predicate as u64) << 27;
        val += (object as u64) << 7;
        match object_type {
            TripleObjectType::BlankNode => {}
            TripleObjectType::IRI => {
                val += 1 << 25;
            }
            TripleObjectType::Literal => {
                val += 3 << 25;
                val += datatype_or_lang as u64;
            }
            TripleObjectType::LiteralLang => {
                val += 2 << 25;
                val += datatype_or_lang as u64
            }
        }
        Triple64 { value: val }
    }
    fn set_subject(&mut self, subject: u32) {
        self.value &= !(0x3ffff << 45);
        self.value += (subject as u64) << 45;
    }
    fn set_predicate(&mut self, predicate: u32) {
        self.value &= !(0x3ffff << 27);
        self.value += (predicate as u64) << 27;
    }
    fn set_object(&mut self, object: u32) {
        self.value &= !(0x3ffff << 7);
        self.value += (object as u64) << 7;
    }
    fn set_datatype_or_lang(&mut self, datatype_or_lang: u32) {
        self.value &= !0x3ffff;
        self.value += datatype_or_lang as u64;
    }
    fn subject_is_iri(&self) -> bool {
        (self.value >> 63) == 1
    }
    fn object_is_iri(&self) -> bool {
        (self.value >> 25) & 3 == 1
    }
    fn object_is_blank_node(&self) -> bool {
        (self.value >> 25) & 3 == 0
    }
    fn has_language(&self) -> bool {
        (self.value >> 25) & 3 == 2
    }
    fn subject(&self) -> u32 {
        ((self.value >> 45) & 0x3ffff) as u32
    }
    fn predicate(&self) -> u32 {
        ((self.value >> 27) & 0x3ffff) as u32
    }
    fn object(&self) -> u32 {
        ((self.value >> 7) & 0x3ffff) as u32
    }
    fn datatype_or_lang(&self) -> u32 {
        (self.value & 0x7f) as u32
    }
}

#[test]
fn test_triple1() {
    let t = Triple64::triple(false, 1, 2, TripleObjectType::BlankNode, 3, 4);
    println!("test1 {}", t.value);
    assert_eq!(t.subject_is_iri(), false);
    assert_eq!(t.subject(), 1);
    assert_eq!(t.predicate(), 2);
    assert_eq!(t.object_is_blank_node(), true);
    assert_eq!(t.object_is_iri(), false);
    assert_eq!(t.has_language(), false);
    assert_eq!(t.object(), 3);
    assert_eq!(t.datatype_or_lang(), 0);
}
#[test]
fn test_triple2() {
    let t = Triple64::triple(true, 1, 2, TripleObjectType::IRI, 3, 4);
    println!("test2 {}", t.value);
    assert_eq!(t.subject_is_iri(), true);
    assert_eq!(t.subject(), 1);
    assert_eq!(t.predicate(), 2);
    assert_eq!(t.object_is_blank_node(), false);
    assert_eq!(t.object_is_iri(), true);
    assert_eq!(t.has_language(), false);
    assert_eq!(t.object(), 3);
    assert_eq!(t.datatype_or_lang(), 0);
}
#[test]
fn test_triple3() {
    let t = Triple64::triple(false, 1, 2, TripleObjectType::Literal, 3, 4);
    println!("test1 {}", t.value);
    assert_eq!(t.subject_is_iri(), false);
    assert_eq!(t.subject(), 1);
    assert_eq!(t.predicate(), 2);
    assert_eq!(t.object_is_blank_node(), false);
    assert_eq!(t.object_is_iri(), false);
    assert_eq!(t.has_language(), false);
    assert_eq!(t.object(), 3);
    assert_eq!(t.datatype_or_lang(), 4);
}
#[test]
fn test_triple4() {
    let t = Triple64::triple(false, 1, 2, TripleObjectType::LiteralLang, 3, 4);
    println!("test1 {}", t.value);
    assert_eq!(t.subject_is_iri(), false);
    assert_eq!(t.subject(), 1);
    assert_eq!(t.predicate(), 2);
    assert_eq!(t.object_is_blank_node(), false);
    assert_eq!(t.object_is_iri(), false);
    assert_eq!(t.has_language(), true);
    assert_eq!(t.object(), 3);
    assert_eq!(t.datatype_or_lang(), 4);
}
#[test]
fn test_triple_set_subject() {
    let mut t = Triple64::triple(false, 1, 2, TripleObjectType::BlankNode, 3, 4);
    t.set_subject(2);
    assert_eq!(t.subject(), 2);
}
#[test]
fn test_triple_set_predicate() {
    let mut t = Triple64::triple(false, 1, 2, TripleObjectType::BlankNode, 3, 4);
    t.set_predicate(3);
    assert_eq!(t.predicate(), 3);
}
#[test]
fn test_triple_set_object() {
    let mut t = Triple64::triple(false, 1, 2, TripleObjectType::BlankNode, 3, 4);
    t.set_object(4);
    assert_eq!(t.object(), 4);
}
