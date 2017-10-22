extern crate chrono;
extern crate csv;
use self::chrono::prelude::{DateTime, Utc, TimeZone};
use std::num::{ParseIntError, ParseFloatError};

use ohlcv::Ohlcv;

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct RecordParser {
    datetime_format: String
}

impl RecordParser {

    pub fn new(datetime_format: String) -> RecordParser {
        RecordParser { datetime_format: datetime_format }
    }

    pub fn parse<I>(&self, records: I) -> Result<Vec<Ohlcv>, ParseError> 
        where I: Iterator<Item=Result<csv::StringRecord, csv::Error>> 
    {
        let mut result = vec![];
        for record in records {
            if let Err(e) = record {
                return Err(ParseError::InvalidRecordStructure(e))
            }
            else {
                let ohlcv = self.parse_one(record.unwrap())?;
                result.push(ohlcv);
            }
        }
        Ok(result)
    }

    pub fn parse_one(&self, record: csv::StringRecord) -> Result<Ohlcv, ParseError> {
        Ok(Ohlcv {
            datetime: self.parse_datetime_field(&record[0])?, 
            open: self.parse_ohlc_field(&record[1])?,
            high: self.parse_ohlc_field(&record[2])?,
            low: self.parse_ohlc_field(&record[3])?,
            close: self.parse_ohlc_field(&record[4])?,            
            volume: self.parse_volume_field(&record[5])?
        })
    }

    pub fn parse_datetime_field(&self, field: &str) -> Result<DateTime<Utc>, ParseError> {
        match Utc.datetime_from_str(field, &self.datetime_format) {
            Ok(d) => Ok(d),
            Err(e) => Err(ParseError::DatetimeError(e))
        }
    }

    pub fn parse_ohlc_field(&self, field: &str) -> Result<f64, ParseError> {
        match field.parse() {
            Ok(value) => Ok(value),
            Err(e) => Err(ParseError::OhlcError(e))
        }
    }

    pub fn parse_volume_field(&self, field: &str) -> Result<u32, ParseError> {
        match field.parse() {
            Ok(value) => Ok(value),
            Err(e) => Err(ParseError::VolumeError(e))
        }
    }

}

#[derive(Debug)]
pub enum ParseError {
    DatetimeError(chrono::ParseError),
    OhlcError(ParseFloatError),
    VolumeError(ParseIntError),
    InvalidRecordStructure(csv::Error)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_correct_record() {
        let parser = RecordParser::new(String::from("%Y%m%d %H:%M:%S"));
        let record = csv::StringRecord::from(vec![
            "20170101 23:59:59", "1.325", "1.330", "1.320", "1.328", "8"
        ]);
        assert_eq!(
            parser.parse(vec![Ok(record)].into_iter()).unwrap(),
            vec![
                Ohlcv {
                    datetime: Utc.ymd(2017, 1, 1).and_hms(23, 59, 59),
                    open: 1.325, high: 1.330, low: 1.320, close: 1.328, volume: 8
                }
            ]
        )
    }

    //#[test]
    //fn parse_incorrect_record() {
        //let parser = RecordParser::new(String::from(""));
        //let err = csv::Error(Box::new(csv::ErrorKind::Seek));
        //assert!(parser.parse(vec![Err(err)].into_iter()).is_err());
    //}

    #[test]
    fn parse_correct_ohlc_field() {
        let parser = RecordParser::new(String::from(""));
        assert_eq!(parser.parse_ohlc_field("1.3052").unwrap(), 1.3052);
    }

    #[test]
    #[should_panic]
    fn parse_incorrect_ohlc_field() {
        let parser = RecordParser::new(String::from(""));
        parser.parse_ohlc_field("erroneous").unwrap();
    }

    #[test]
    fn parse_correct_volume_field() {
        let parser = RecordParser::new(String::from(""));
        assert_eq!(parser.parse_volume_field("123").unwrap(), 123);
    }

    #[test]
    #[should_panic]
    fn parse_incorrect_volume_field() {
        let parser = RecordParser::new(String::from(""));
        parser.parse_volume_field("1.30").unwrap();
    }

    #[test]
    fn parse_correct_datetime_field() {
        let parser = RecordParser::new(String::from("%Y%m%d %H:%M:%S"));
        assert_eq!(
            parser.parse_datetime_field("20170101 23:59:59").unwrap(),
            Utc.ymd(2017, 1, 1).and_hms(23, 59, 59)
        );
    }

    #[test]
    fn parse_incorrect_datetime_field() {
        let parser = RecordParser::new(String::from("%Y%m%d %H:%M:%S"));
        assert!(parser.parse_datetime_field("20170101 23:59").is_err());
    }
}
