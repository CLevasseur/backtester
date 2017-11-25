use std::fmt;
use ohlcv::source::OhlcvSource;

pub struct Symbol<'a> {
    pub name: String,
    pub source: &'a OhlcvSource
}

impl<'a> Eq for Symbol<'a> {}
impl<'a> PartialEq for Symbol<'a> {
    fn eq(&self, other: &Symbol) -> bool {
        self.name == other.name
    }
}
impl<'a> fmt::Debug for Symbol<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl<'a> Symbol<'a> {
    
    pub fn new(name: String, source: &'a OhlcvSource) -> Symbol<'a> {
        Symbol {
            name: name,
            source: source
        }
    }

}
