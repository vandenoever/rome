
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
    fn object_is_iri(&self) -> bool;
    fn object_is_blank_node(&self) -> bool;
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
        val += (subject as u64) << 54;
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
        self.value &= !(0x3ffff << 54);
        self.value += (subject as u64) << 54;
    }
    fn set_predicate(&mut self, predicate: u32) {
        self.value &= !(0x3ffff << 54);
        self.value += (predicate as u64) << 27;
    }
    fn set_object(&mut self, object: u32) {
        self.value &= !(0x3ffff << 54);
        self.value += (object as u64) << 7;
    }
    fn set_datatype_or_lang(&mut self, datatype_or_lang: u32) {
        self.value &= !(0x3ffff << 54);
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
    fn subject(&self) -> u32 {
        ((self.value >> 54) & 0x3ffff) as u32
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
