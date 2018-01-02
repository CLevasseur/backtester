extern crate chrono;
extern crate csv;

use std::io::Read;
use self::chrono::prelude::{DateTime, Utc};
use ohlcv::Ohlcv;
use ohlcv::source::{OhlcvSource, OhlcvSourceError};
use util::record_parser::{RecordParser, ParseError};

#[derive(Debug)]
pub struct CsvOhlcvSource<T> {
    csv_reader: csv::Reader<T>,
    record_parser: RecordParser,
    loaded: Vec<Ohlcv>
}

impl<T: Read> OhlcvSource for CsvOhlcvSource<T> {
    fn ohlcv(&self, start_date: &DateTime<Utc>, end_date: &DateTime<Utc>) 
        -> Result<Vec<Ohlcv>, OhlcvSourceError> {

        let start_idx = match self.loaded.binary_search_by_key(&start_date, |ohlcv| ohlcv.datetime()) {
            Ok(idx) => idx,
            Err(idx) => idx
        };

        let mut end_idx = match self.loaded.binary_search_by_key(&end_date, |ohlcv| ohlcv.datetime()) {
            Ok(idx) => idx,
            Err(idx) => idx
        };

        if start_idx == end_idx {
            end_idx += 1;
        }
        Ok(self.loaded[start_idx..end_idx].to_vec())
    }
}

impl<T: Read> CsvOhlcvSource<T> {

    pub fn new(csv_reader: csv::Reader<T>, record_parser: RecordParser) -> Result<CsvOhlcvSource<T>, ParseError> {
        let mut source = CsvOhlcvSource {
            csv_reader,
            record_parser,
            loaded: vec![]
        };
        source.loaded = source.record_parser.parse(source.csv_reader.records())?;
        Ok(source)
    }

}

#[cfg(test)]
mod tests {
    use super::*;
    use symbol::SymbolId;

//    use ohlcv::chrono::TimeZone;

    //#[test]
    //fn new_with_correct_inputs() {
        //let data = "date;open;high;low;close;volume
        //20160103 170000;1.087010;1.087130;1.087010;1.087130;1
        //20160103 170100;1.087120;1.087120;1.087120;1.087120;0
        //20160103 170200;1.087080;1.087220;1.087080;1.087220;4";
        //let rdr = csv::ReaderBuilder::new().delimiter(b';').from_reader(data.as_bytes());
        //let source = CsvOhlcvSource::new(rdr, String::from("%Y%m%d %H%M%S")).unwrap();
        //assert_eq!(
            //source.ohlcv(&Utc.ymd(2016, 1, 3).and_hms(17, 0, 0), &Utc.ymd(2016, 1, 3).and_hms(17, 0, 0)).unwrap(),
            //vec![
                //Ohlcv {
                    //datetime: Utc.ymd(2016, 1, 3).and_hms(17, 0, 0),
                    //open: 1.087010,
                    //high: 1.087130,
                    //low: 1.087010,
                    //close: 1.087130,
                    //volume: 1
                //}
            //]
        //)
    //}

    #[test]
    fn new_with_incorrect_date_format() {
        let data = "date;open;high;low;close;volume
        20160103 170000;1.087010;1.087130;1.087010;1.087130;1
        20160103 170100;1.087120;1.087120;1.087120;1.087120;0
        20160103 170200;1.087080;1.087220;1.087080;1.087220;4";
        let rp = RecordParser::new(SymbolId::from("eur/usd"), String::from("%m%d%Y"));
        let rdr = csv::ReaderBuilder::new().delimiter(b';').from_reader(data.as_bytes());
        assert!(CsvOhlcvSource::new(rdr, rp).is_err());
    }

    #[test]
    fn new_with_incorrect_number_of_columns() {
        let data = "date;open;high;low;close;volume
        20160103 170000;1.087010;1.087130;1.087010;1.087130;1
        20160103 170100;1.087120;1.087120;1.087120;1.087120;0
        20160103 170200;1.087080;1.087220;1.087080;1.087220";
        let rp = RecordParser::new(SymbolId::from("eur/usd"), String::from("%m%d%Y"));
        let rdr = csv::ReaderBuilder::new().delimiter(b';').from_reader(data.as_bytes());
        assert!(CsvOhlcvSource::new(rdr, rp).is_err());
    }
}
