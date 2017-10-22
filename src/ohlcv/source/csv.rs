extern crate chrono;
extern crate csv;

use std::io::Read;
use std::collections::HashMap;
use self::chrono::prelude::{DateTime, Utc};
use ohlcv::Ohlcv;
use ohlcv::source::{OhlcvSource, OhlcvSourceError};
use util::record_parser::{RecordParser, ParseError};

#[derive(Debug)]
pub struct CsvOhlcvSource<T> {
    csv_reader: csv::Reader<T>,
    record_parser: RecordParser,
    loaded: HashMap<DateTime<Utc>, Ohlcv>
}

impl<T: Read> OhlcvSource for CsvOhlcvSource<T> {
    fn ohlcv(&self, date: &DateTime<Utc>) -> Result<Ohlcv, OhlcvSourceError> { 
        match self.loaded.get(date) {
            Some(ohlcv) => Ok(ohlcv.clone()),
            None => Err(OhlcvSourceError::DateNotFound(date.clone()))
        }
    }
}

impl<T: Read> CsvOhlcvSource<T> {

    pub fn new(csv_reader: csv::Reader<T>, datetime_format: String) -> Result<CsvOhlcvSource<T>, ParseError> {
        let mut source = CsvOhlcvSource {
            csv_reader: csv_reader,
            record_parser: RecordParser::new(datetime_format),
            loaded: HashMap::new()
        };
        let ohlcv = source.record_parser.parse(source.csv_reader.records())?;
        source.loaded = source.load(ohlcv)?;
        println!("{:#?}", source.loaded);
        Ok(source)
    }

    fn load(&self, ohlcv: Vec<Ohlcv>) -> Result<HashMap<DateTime<Utc>, Ohlcv>, ParseError> {
        let mut loaded = HashMap::new();
        for i in ohlcv {
            loaded.insert(i.datetime, i);
        }
        Ok(loaded)
    }

}

#[cfg(test)]
mod tests {
    use super::*;
    use ohlcv::chrono::TimeZone;

    #[test]
    fn new_with_correct_inputs() {
        let data = "date;open;high;low;close;volume
        20160103 170000;1.087010;1.087130;1.087010;1.087130;1
        20160103 170100;1.087120;1.087120;1.087120;1.087120;0
        20160103 170200;1.087080;1.087220;1.087080;1.087220;4";
        let rdr = csv::ReaderBuilder::new().delimiter(b';').from_reader(data.as_bytes());
        let source = CsvOhlcvSource::new(rdr, String::from("%Y%m%d %H%M%S")).unwrap();
        assert_eq!(
            source.ohlcv(&Utc.ymd(2016, 1, 3).and_hms(17, 0, 0)).unwrap(),
            Ohlcv {
                datetime: Utc.ymd(2016, 1, 3).and_hms(17, 0, 0),
                open: 1.087010,
                high: 1.087130,
                low: 1.087010,
                close: 1.087130,
                volume: 1
            }
        )
    }

    #[test]
    fn new_with_incorrect_date_format() {
        let data = "date;open;high;low;close;volume
        20160103 170000;1.087010;1.087130;1.087010;1.087130;1
        20160103 170100;1.087120;1.087120;1.087120;1.087120;0
        20160103 170200;1.087080;1.087220;1.087080;1.087220;4";
        let rdr = csv::ReaderBuilder::new().delimiter(b';').from_reader(data.as_bytes());
        assert!(CsvOhlcvSource::new(rdr, String::from("%m%d%Y")).is_err());
    }

    #[test]
    fn new_with_incorrect_number_of_columns() {
        let data = "date;open;high;low;close;volume
        20160103 170000;1.087010;1.087130;1.087010;1.087130;1
        20160103 170100;1.087120;1.087120;1.087120;1.087120;0
        20160103 170200;1.087080;1.087220;1.087080;1.087220";
        let rdr = csv::ReaderBuilder::new().delimiter(b';').from_reader(data.as_bytes());
        assert!(CsvOhlcvSource::new(rdr, String::from("%Y%m%d %H%M%S")).is_err());
    }
}
