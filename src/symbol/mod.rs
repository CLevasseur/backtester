use ohlcv::source::OhlcvSource;

pub struct Symbol {
    pub name: String,
    pub source: Box<OhlcvSource>
}
