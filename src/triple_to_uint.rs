
pub enum TripleObjectType {
    BlankNode,
    IRI,
    Literal,
    LiteralLang,
}

pub trait CompactTriple<T>
    where T: Ord + Copy
{
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
    fn object_type(&self) -> TripleObjectType;
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

/// characteristic features of one triple type:
/// - number of bits for blanks, iri, literals (bil)
/// - number of bits for datatypes and languages (dl)
/// - total number of bits: 3 + 3*bil + dl
///     e.g. 3 + 3*18 + 7 = 64
/// - order of fields, e.g. subject, predicate, object, language
///     or object, predicate, subject

/// 1 bit to determine blank or iri
/// 18 bits for subject id
/// 18 bits for predicate id
/// 1 bit to determine non-literal or literal
/// 1 bit to determine blank or iri (for non-literal)
///          or has lang (0) or has no lang (1)
/// 18 bits for object id
///  7 bits for for datatype or language id
const BIL: u32 = 18;
const DL: u32 = 7;

macro_rules! triple64{(
    $name:ident,
    $subject_offset:expr,
    $predicate_offset:expr,
    $object_offset:expr) => {
        triple64_helper!($name, $subject_offset, $predicate_offset,
            $object_offset, ($subject_offset + BIL), ($object_offset + BIL));
    };
}

macro_rules! triple64_helper{(
    $name:ident,
    $subject_offset:expr,
    $predicate_offset:expr,
    $object_offset:expr,
    $subject_type_offset:expr,
    $object_type_offset:expr) => {

#[derive(PartialEq,Eq,Hash,PartialOrd,Ord,Clone,Copy)]
pub struct $name {
    value: u64,
}

impl CompactTriple<u32> for $name {
    fn max_subject_value() -> u32 {
        2 ^ BIL as u32
    }
    fn max_datatype_value() -> u32 {
        2 ^ DL as u32
    }
    fn triple(subject_is_iri: bool,
              subject: u32,
              predicate: u32,
              object_type: TripleObjectType,
              object: u32,
              datatype_or_lang: u32)
              -> $name {
        let mut val = (subject_is_iri as u64) << $subject_type_offset;
        val += (subject as u64) << $subject_offset;
        val += (predicate as u64) << $predicate_offset;
        val += (object as u64) << $object_offset;
        match object_type {
            TripleObjectType::BlankNode => {}
            TripleObjectType::IRI => {
                val += 1 << $object_type_offset;
            }
            TripleObjectType::Literal => {
                val += 3 << $object_type_offset;
                val += datatype_or_lang as u64;
            }
            TripleObjectType::LiteralLang => {
                val += 2 << $object_type_offset;
                val += datatype_or_lang as u64
            }
        }
        $name { value: val }
    }
    fn set_subject(&mut self, subject: u32) {
        self.value &= !(0x3ffff << $subject_offset);
        self.value += (subject as u64) << $subject_offset;
    }
    fn set_predicate(&mut self, predicate: u32) {
        self.value &= !(0x3ffff << $predicate_offset);
        self.value += (predicate as u64) << $predicate_offset;
    }
    fn set_object(&mut self, object: u32) {
        self.value &= !(0x3ffff << $object_offset);
        self.value += (object as u64) << $object_offset;
    }
    fn set_datatype_or_lang(&mut self, datatype_or_lang: u32) {
        self.value &= !0x3ffff;
        self.value += datatype_or_lang as u64;
    }
    fn subject_is_iri(&self) -> bool {
        (self.value >> $subject_type_offset) == 1
    }
    fn object_is_iri(&self) -> bool {
        (self.value >> $object_type_offset) & 3 == 1
    }
    fn object_is_blank_node(&self) -> bool {
        (self.value >> $object_type_offset) & 3 == 0
    }
    fn object_type(&self) -> TripleObjectType {
        match (self.value >> $object_type_offset) & 3 {
            0 => TripleObjectType::BlankNode,
            1 => TripleObjectType::IRI,
            2 => TripleObjectType::Literal,
            _ => TripleObjectType::LiteralLang,
        }
    }
    fn has_language(&self) -> bool {
        (self.value >> $object_type_offset) & 3 == 2
    }
    fn subject(&self) -> u32 {
        ((self.value >> $subject_offset) & 0x3ffff) as u32
    }
    fn predicate(&self) -> u32 {
        ((self.value >> $predicate_offset) & 0x3ffff) as u32
    }
    fn object(&self) -> u32 {
        ((self.value >> $object_offset) & 0x3ffff) as u32
    }
    fn datatype_or_lang(&self) -> u32 {
        (self.value & 0x7f) as u32
    }
}

    };
}

triple64!(Triple64SPO, 45, 27, 7);
triple64!(Triple64OPS, 7, 27, 45);

#[test]
fn test_triple1() {
    let t = Triple64SPO::triple(false, 1, 2, TripleObjectType::BlankNode, 3, 4);
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
    let t = Triple64SPO::triple(true, 1, 2, TripleObjectType::IRI, 3, 4);
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
    let t = Triple64SPO::triple(false, 1, 2, TripleObjectType::Literal, 3, 4);
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
    let t = Triple64SPO::triple(false, 1, 2, TripleObjectType::LiteralLang, 3, 4);
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
    let mut t = Triple64SPO::triple(false, 1, 2, TripleObjectType::BlankNode, 3, 4);
    t.set_subject(2);
    assert_eq!(t.subject(), 2);
}
#[test]
fn test_triple_set_predicate() {
    let mut t = Triple64SPO::triple(false, 1, 2, TripleObjectType::BlankNode, 3, 4);
    t.set_predicate(3);
    assert_eq!(t.predicate(), 3);
}
#[test]
fn test_triple_set_object() {
    let mut t = Triple64SPO::triple(false, 1, 2, TripleObjectType::BlankNode, 3, 4);
    t.set_object(4);
    assert_eq!(t.object(), 4);
}
