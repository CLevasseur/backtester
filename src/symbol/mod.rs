use std::hash::Hash;
use ohlcv::source::OhlcvSource;

#[derive(Eq, PartialEq, Hash, Debug)]
pub struct Symbol<'a, T: 'a + SymbolOhlcvSource> {
    pub name: String,
    pub source: &'a T
}

pub trait SymbolOhlcvSource: OhlcvSource + Eq + Hash {}

impl<'a, T: 'a + SymbolOhlcvSource> Symbol<'a, T> {
    
    pub fn new(name: String, source: &'a T) -> Symbol<'a, T> {
        Symbol {
            name: name,
            source: source
        }
    }

}
