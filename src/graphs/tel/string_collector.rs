use std::cmp::Ordering;
use std::ops::Index;

#[derive(Clone)]
struct StringRef {
    start: u32,
    end: u32,
    index: u32,
}

// Within a StringCollection the strings are sorted, so sorting by StringId
// is just like sorting by the underlying strings.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct StringId {
    pub id: u32,
}

pub struct StringCollector {
    buffer: String,
    refs: Vec<StringRef>,
}
fn slice<'a>(buffer: &'a str, r: &StringRef) -> &'a str {
    &buffer[r.start as usize..r.end as usize]
}
impl StringCollector {
    /// Creates a new empty StringBuffer with a particular capacity.
    pub fn with_capacity(capacity: usize) -> StringCollector {
        StringCollector {
            buffer: String::with_capacity(capacity),
            refs: Vec::new(),
        }
    }
    /// Add a string to the StringBuffer and return an identifier for it.
    pub fn add_string(&mut self, string: &str) -> StringId {
        let start = self.buffer.len() as u32;
        self.buffer.push_str(string);
        let index = self.refs.len() as u32;
        self.refs.push(StringRef {
            start,
            end: self.buffer.len() as u32,
            index,
        });
        StringId { id: index }
    }
    /// Return the number of bytes available for writing.
    /// It is possible to write more, but that would give a potentially costly
    /// reallocation.
    #[cfg(unstable)]
    pub fn space(&self) -> usize {
        self.buffer.capacity() - self.buffer.len()
    }
    /// Sort the references by the strings that they point to.
    fn sort(&mut self) {
        let buffer = &self.buffer;
        self.refs.sort_by_key(|s| slice(buffer, s));
    }
    /// Remove duplicate strings from the refs array and create an array that
    /// translates from the old order to the new order.
    fn deduplicate_and_translate(&mut self) -> Vec<StringId> {
        let buffer = &self.buffer;
        let refs = &mut self.refs;
        if refs.is_empty() {
            return Vec::new();
        }
        let mut translation = vec![StringId { id: 0 }; refs.len()];
        translation[refs[0].index as usize] = StringId { id: 0 };
        let mut to = 0;
        let mut prev_str = slice(buffer, &refs[0]);
        let end = refs.len();
        for i in 1..end {
            let r = refs[i].clone();
            let str = slice(buffer, &r);
            if str != prev_str {
                to += 1;
                refs[to].start = r.start;
                refs[to].end = r.end;
                prev_str = str;
            }
            translation[r.index as usize] = StringId { id: to as u32 };
        }
        refs.truncate(to + 1);
        translation
    }
    /// Create a new string buffer that contains each string only once
    /// in sorted order.
    fn create_new_buffer(&mut self) -> String {
        let buffer = &self.buffer;
        let refs = &mut self.refs;
        let mut new_buf = String::new();
        for r in refs.iter_mut() {
            let start = new_buf.len() as u32;
            new_buf.push_str(slice(buffer, r));
            r.start = start;
        }
        new_buf
    }
    /// Collect the strings in a new StringCollection.
    /// Also return an array that translates from the old order to the new order.
    pub fn collect(&mut self) -> (Vec<StringId>, StringCollection) {
        self.sort();
        let translation = self.deduplicate_and_translate();
        let new_buffer = self.create_new_buffer();
        let mut starts: Vec<u32> = self.refs.iter().map(|r| r.start).collect();
        starts.push(new_buffer.len() as u32);
        let collection = StringCollection {
            buffer: new_buffer,
            starts,
        };
        self.buffer.clear();
        self.refs.clear();
        (translation, collection)
    }
}

#[derive(Clone, Debug)]
pub struct StringCollection {
    buffer: String,
    starts: Vec<u32>,
}

impl StringCollection {
    pub fn get(&self, i: StringId) -> &str {
        let start = self.starts[i.id as usize] as usize;
        let end = self.starts[(i.id + 1) as usize] as usize;
        &self.buffer[start..end]
    }
    pub fn find(&self, s: &str) -> Option<StringId> {
        match binary_search_by_index(self.starts.len() - 1, |i| {
            self.get(StringId { id: i as u32 }).cmp(s)
        }) {
            Ok(pos) => Some(StringId { id: pos as u32 }),
            Err(_) => None,
        }
    }
}

fn binary_search_by_index<F>(len: usize, mut f: F) -> Result<usize, usize>
where
    F: FnMut(usize) -> Ordering,
{
    if len == 0 {
        return Err(0);
    }
    let mut l = 0;
    let mut r = len - 1;
    loop {
        if l > r {
            return Err(l);
        }
        let m = (l + r) >> 1;
        match f(m) {
            Ordering::Less => {
                l = m + 1;
            }
            Ordering::Greater => {
                r = m - 1;
            }
            Ordering::Equal => return Ok(m),
        }
    }
}

impl Index<StringId> for Vec<StringId> {
    type Output = StringId;

    fn index(&self, id: StringId) -> &StringId {
        &self[id.id as usize]
    }
}

#[test]
fn test_string_collector() {
    let mut c = StringCollector::with_capacity(1000);
    let refs = [
        c.add_string("xy"),
        c.add_string("1234"),
        c.add_string("xy"),
        c.add_string("abc"),
    ];
    assert_eq!(
        refs,
        [
            StringId { id: 0 },
            StringId { id: 1 },
            StringId { id: 2 },
            StringId { id: 3 },
        ]
    );
    let (translation, c) = c.collect();
    assert_eq!(
        translation,
        [
            StringId { id: 2 },
            StringId { id: 0 },
            StringId { id: 2 },
            StringId { id: 1 },
        ]
    );
    assert_eq!(c.get(StringId { id: 0 }), "1234");
    assert_eq!(c.get(StringId { id: 1 }), "abc");
    assert_eq!(c.get(StringId { id: 2 }), "xy");
}
