extern crate chrono;

use self::chrono::prelude::{DateTime, Utc};
use ohlcv::Ohlcv;
use ohlcv::source::{OhlcvSource, OhlcvSourceError};

pub struct OhlcvSourceCollection {
    symbol_sources: Vec<Box<OhlcvSource>>
}

impl OhlcvSource for OhlcvSourceCollection {

    fn ohlcv(&self, start_date: &DateTime<Utc>, end_date: &DateTime<Utc>)
             -> Result<Vec<Ohlcv>, OhlcvSourceError>
    {
        let mut all_ohlcv: Vec<Ohlcv> = vec![];

        for source in &self.symbol_sources {
            all_ohlcv.append(&mut source.ohlcv(start_date, end_date)?)
        }

        all_ohlcv.sort_unstable_by(
            |a, b| (a.datetime(), a.symbol_id()).cmp(&(b.datetime(), b.symbol_id()))
        );
        Ok(all_ohlcv)
    }

}

impl OhlcvSourceCollection {
    pub fn symbol_sources(&self) -> &Vec<Box<OhlcvSource>> {
        &self.symbol_sources
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use self::chrono::prelude::TimeZone;
    use symbol::SymbolId;
    use ohlcv::source::OhlcvSource;

    pub struct FakeOhlcvSource { symbol_id: SymbolId }
    impl OhlcvSource for FakeOhlcvSource {
        fn ohlcv(&self, _start_date: &DateTime<Utc>, _end_date: &DateTime<Utc>)
                 -> Result<Vec<Ohlcv>, OhlcvSourceError>
        {
            Ok(vec![
                Ohlcv::new(self.symbol_id.clone(), Utc.ymd(2017, 12, 29).and_hms(12, 0, 0), 1., 1., 1., 1., 0),
                Ohlcv::new(self.symbol_id.clone(), Utc.ymd(2017, 12, 29).and_hms(12, 0, 5), 1., 1.5, 1., 1.5, 3)
            ])
        }
    }

    #[test]
    fn ohlcv() {
        let source_collection = OhlcvSourceCollection {
            symbol_sources: vec![
                Box::new(FakeOhlcvSource { symbol_id: SymbolId::from("eur/usd") }),
                Box::new(FakeOhlcvSource { symbol_id: SymbolId::from("usd/jpy") })
            ]
        };
        assert_eq!(
            source_collection.ohlcv(
                &Utc.ymd(2017, 12, 29).and_hms(12, 0, 0),
                &Utc.ymd(2017, 12, 29).and_hms(12, 0, 5)
            ).unwrap(),
            vec![
                Ohlcv::new(SymbolId::from("eur/usd"), Utc.ymd(2017, 12, 29).and_hms(12, 0, 0), 1., 1., 1., 1., 0),
                Ohlcv::new(SymbolId::from("usd/jpy"), Utc.ymd(2017, 12, 29).and_hms(12, 0, 0), 1., 1., 1., 1., 0),
                Ohlcv::new(SymbolId::from("eur/usd"), Utc.ymd(2017, 12, 29).and_hms(12, 0, 5), 1., 1.5, 1., 1.5, 3),
                Ohlcv::new(SymbolId::from("usd/jpy"), Utc.ymd(2017, 12, 29).and_hms(12, 0, 5), 1., 1.5, 1., 1.5, 3)
            ]
        );
    }
}