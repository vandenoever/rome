use super::compact_triple::*;

/// characteristic features of one triple type:
/// - number of bits for blanks, iri, literals (bil)
/// - number of bits for datatypes and languages (dl)
/// - total number of bits: 3 + 3*bil + dl
///     e.g. 3 + 3*32 + 29 = 128
/// - order of fields, e.g. subject, predicate, object, language
///     or object, predicate, subject

/// 1 bit to determine blank or iri
/// 32 bits for subject id
/// 32 bits for predicate id
/// 1 bit to determine non-literal or literal
/// 1 bit to determine blank or iri (for non-literal)
///          or has lang (0) or has no lang (1)
/// 32 bits for object id
/// 29 bits for for datatype or language id
const BIL: u32 = 32;
const DL: u32 = 29;
const BILMASK: u64 = 0xffff_ffff;
const DLMASK: u64 = 0x1fff_ffff;

macro_rules! triple128 {
    (
        $name:ident,
        $subject_var:ident,
        $subject_offset:expr,
        $predicate_offset:expr,
        $object_var:ident,
        $object_offset:expr
    ) => {
        triple128_helper!(
            $name,
            $subject_var,
            $subject_offset,
            $predicate_offset,
            (64 - $predicate_offset),
            $object_var,
            $object_offset,
            ($subject_offset + BIL),
            ($object_offset + BIL)
        );
    };
}

macro_rules! triple128_helper {
    (
        $name:ident,
        $subject_var:ident,
        $subject_offset:expr,
        $predicate_offset_1:expr,
        $predicate_offset_2:expr,
        $object_var:ident,
        $object_offset:expr,
        $subject_type_offset:expr,
        $object_type_offset:expr
    ) => {
        /// Compact triple used in tel::Graph.
        #[derive(PartialEq, Eq, Hash, PartialOrd, Ord, Clone, Copy, Debug)]
        pub struct $name {
            v1: u64,
            v2: u64,
        }

        impl CompactTriple<u32> for $name {
            fn max_subject_value() -> u32 {
                2 ^ BIL as u32
            }
            fn max_datatype_value() -> u32 {
                2 ^ DL as u32
            }
            fn triple(
                subject_is_iri: bool,
                subject: u32,
                predicate: u32,
                object_type: TripleObjectType,
                object: u32,
                datatype_or_lang: u32,
            ) -> $name {
                let subject = u64::from(subject);
                let predicate = u64::from(predicate);
                let object = u64::from(object);
                let datatype_or_lang = u64::from(datatype_or_lang);
                // check that the values are within the allowed range
                assert!(subject & !BILMASK == 0, "subject out of range {}", subject);
                assert!(
                    predicate & !BILMASK == 0,
                    "predicate out of range {}",
                    predicate
                );
                assert!(object & !BILMASK == 0, "object out of range {}", object);
                assert!(
                    datatype_or_lang & !DLMASK == 0,
                    "datatype_or_lang out of range {}",
                    datatype_or_lang
                );

                let mut s = (subject_is_iri as u64) << $subject_type_offset;
                s += subject << $subject_offset;
                let mut o = object << $object_offset;
                let mut d = 0;
                match object_type {
                    TripleObjectType::BlankNode => {}
                    TripleObjectType::IRI => {
                        o += 1 << $object_type_offset;
                    }
                    TripleObjectType::Literal => {
                        o += 3 << $object_type_offset;
                        d += datatype_or_lang;
                    }
                    TripleObjectType::LiteralLang => {
                        o += 2 << $object_type_offset;
                        d += datatype_or_lang;
                    }
                }
                if $subject_offset > $object_offset {
                    // subject is in v1
                    s += predicate >> $predicate_offset_1;
                    o += predicate << $predicate_offset_2;
                    o += d;
                } else {
                    o += predicate >> $predicate_offset_1;
                    s += predicate << $predicate_offset_2;
                    s += d;
                }
                $name {
                    $subject_var: s,
                    $object_var: o,
                }
            }
            fn set_subject(&mut self, subject: u32) {
                let subject = u64::from(subject);
                assert!(subject & !BILMASK == 0, "subject out of range {}", subject);
                self.$subject_var &= !(BILMASK << $subject_offset);
                self.$subject_var += subject << $subject_offset;
            }
            fn set_predicate(&mut self, predicate: u32) {
                let predicate = u64::from(predicate);
                assert!(
                    predicate & !BILMASK == 0,
                    "predicate out of range {}",
                    predicate
                );
                self.v1 &= !(BILMASK >> $predicate_offset_1);
                self.v1 += predicate >> $predicate_offset_1;
                self.v2 &= !(BILMASK << $predicate_offset_2);
                self.v2 += predicate << $predicate_offset_2;
            }
            fn set_object(&mut self, object: u32) {
                let object = u64::from(object);
                assert!(object & !BILMASK == 0, "object out of range {}", object);
                self.$object_var &= !(BILMASK << $object_offset);
                self.$object_var += object << $object_offset;
            }
            fn set_datatype_or_lang(&mut self, datatype_or_lang: u32) {
                let datatype_or_lang = u64::from(datatype_or_lang);
                assert!(
                    datatype_or_lang & !DLMASK == 0,
                    "datatype_or_lang out of range {}",
                    datatype_or_lang
                );
                self.v2 &= !DLMASK;
                self.v2 += datatype_or_lang;
            }
            fn subject_is_iri(&self) -> bool {
                self.$subject_var & (1 << $subject_type_offset) == 1 << $subject_type_offset
            }
            fn object_is_iri(&self) -> bool {
                self.$object_var & (3 << $object_type_offset) == 1 << $object_type_offset
            }
            fn object_is_blank_node(&self) -> bool {
                self.$object_var & (3 << $object_type_offset) == 0
            }
            fn object_type(&self) -> TripleObjectType {
                match (self.$object_var >> $object_type_offset) & 3 {
                    0 => TripleObjectType::BlankNode,
                    1 => TripleObjectType::IRI,
                    2 => TripleObjectType::Literal,
                    _ => TripleObjectType::LiteralLang,
                }
            }
            fn has_language(&self) -> bool {
                self.$object_var & (3 << $object_type_offset) == 2 << $object_type_offset
            }
            fn subject(&self) -> u32 {
                ((self.$subject_var >> $subject_offset) & BILMASK) as u32
            }
            fn predicate(&self) -> u32 {
                (((self.v1 << $predicate_offset_1) & BILMASK) as u32)
                    + (((self.v2 >> $predicate_offset_2) & BILMASK) as u32)
            }
            fn object(&self) -> u32 {
                ((self.$object_var >> $object_offset) & BILMASK) as u32
            }
            fn datatype_or_lang(&self) -> u32 {
                (self.v2 & DLMASK) as u32
            }
        }
    };
}

triple128!(Triple128SPO, v1, 31, 1, v2, 29);
triple128!(Triple128OPS, v2, 29, 2, v1, 30);

#[test]
fn test_triple1() {
    let t = Triple128SPO::triple(false, 1, 2, TripleObjectType::BlankNode, 3, 4);
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
    let t = Triple128SPO::triple(true, 1, 2, TripleObjectType::IRI, 3, 4);
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
    let t = Triple128SPO::triple(false, 1, 2, TripleObjectType::Literal, 3, 4);
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
    let t = Triple128SPO::triple(false, 1, 2, TripleObjectType::LiteralLang, 3, 4);
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
    let mut t = Triple128SPO::triple(false, 1, 2, TripleObjectType::BlankNode, 3, 4);
    t.set_subject(2);
    assert_eq!(t.subject(), 2);
}
#[test]
fn test_triple_set_predicate() {
    let mut t = Triple128SPO::triple(false, 1, 2, TripleObjectType::BlankNode, 3, 4);
    t.set_predicate(3);
    assert_eq!(t.predicate(), 3);
}
#[test]
fn test_triple_set_object() {
    let mut t = Triple128SPO::triple(false, 1, 2, TripleObjectType::BlankNode, 3, 4);
    t.set_object(4);
    assert_eq!(t.object(), 4);
}

#[test]
fn test_triple1_ops() {
    let t = Triple128OPS::triple(false, 1, 2, TripleObjectType::BlankNode, 3, 4);
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
fn test_triple2_ops() {
    let t = Triple128OPS::triple(true, 1, 2, TripleObjectType::IRI, 3, 4);
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
fn test_triple3_ops() {
    let t = Triple128OPS::triple(false, 1, 2, TripleObjectType::Literal, 3, 4);
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
fn test_triple4_ops() {
    let t = Triple128OPS::triple(false, 1, 2, TripleObjectType::LiteralLang, 3, 4);
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
fn test_triple_set_subject_ops() {
    let mut t = Triple128OPS::triple(false, 1, 2, TripleObjectType::BlankNode, 3, 4);
    t.set_subject(2);
    assert_eq!(t.subject(), 2);
}
#[test]
fn test_triple_set_predicate_ops() {
    let mut t = Triple128OPS::triple(false, 1, 2, TripleObjectType::BlankNode, 3, 4);
    t.set_predicate(3);
    assert_eq!(t.predicate(), 3);
}
#[test]
fn test_triple_set_object_ops() {
    let mut t = Triple128OPS::triple(false, 1, 2, TripleObjectType::BlankNode, 3, 4);
    t.set_object(4);
    assert_eq!(t.object(), 4);
}
