use std::fmt;
use ohlcv::source::OhlcvSource;

pub type SymbolId = String;

pub struct Symbol<'a> {
    pub id: SymbolId,
    pub source: &'a OhlcvSource
}

impl<'a> Eq for Symbol<'a> {}
impl<'a> PartialEq for Symbol<'a> {
    fn eq(&self, other: &Symbol) -> bool {
        self.id == other.id
    }
}
impl<'a> fmt::Debug for Symbol<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.id)
    }
}

impl<'a> Symbol<'a> {
    
    pub fn new(id: SymbolId, source: &'a OhlcvSource) -> Symbol<'a> {
        Symbol {
            id: id,
            source: source
        }
    }

}
