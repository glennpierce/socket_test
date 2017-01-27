use std::result;
use std::str::FromStr;
use chrono::{NaiveDateTime, ParseResult, ParseError};
//use chrono::format{ParseErrorKind};

type Result<T> = result::Result<T, ParseError>;

#[derive(Debug, Clone, Copy)]
struct BmosTimeStamp {
    //Err: String,
    val: NaiveDateTime
}

// trait IntoBmosTimeStamp {
//     type OutReader: Reader;
    
//     fn into_reader(self) -> Self::OutReader;
// }

impl BmosTimeStamp {

    //type Err = String;

    /// Constructs a new `BmosTimeStamp`.
    ///
    /// # Examples
    ///
    /// ```
    /// use bmos_timestamp::BmosTimeStamp;
    ///
    /// let five = BmosTimeStamp::new(5);
    /// ```
    pub fn new(dt: NaiveDateTime) -> BmosTimeStamp {
        BmosTimeStamp { val : dt } 
    }

    pub fn from_timestamp(secs: i64, nsecs: u32) -> BmosTimeStamp {
        BmosTimeStamp { val : NaiveDateTime::from_timestamp(secs, nsecs) } 
    }

    pub fn parse_from_str(s: &str, fmt: &str) -> Result<BmosTimeStamp> {
        match NaiveDateTime::parse_from_str(s, fmt) {
            Ok(n) => Ok(BmosTimeStamp { val : n}),
            Err(err) => Err(err),
        }
    }

    ///
    /// A BmosTimeStamp is a string like 1485517650.675674
    /// It is a timestamp with added microsecends seperated by a period. 
    ///
    fn parse_from_bmos_str(s: &str) -> Result<BmosTimeStamp> {
        // count number of chars
        let len = s.chars().count();
        if len != 17 {
            return Err(ParseError(ParseErrorKind::BadFormat));
        }
       
        On();
    }
}


/*
impl str::FromStr for NaiveDateTime {
    type Err = ParseError;

    fn from_str(s: &str) -> ParseResult<NaiveDateTime> {
        const ITEMS: &'static [Item<'static>] = &[
            Item::Space(""), Item::Numeric(Numeric::Year, Pad::Zero),
            Item::Space(""), Item::Literal("-"),
            Item::Space(""), Item::Numeric(Numeric::Month, Pad::Zero),
            Item::Space(""), Item::Literal("-"),
            Item::Space(""), Item::Numeric(Numeric::Day, Pad::Zero),
            Item::Space(""), Item::Literal("T"), // XXX shouldn't this be case-insensitive?
            Item::Space(""), Item::Numeric(Numeric::Hour, Pad::Zero),
            Item::Space(""), Item::Literal(":"),
            Item::Space(""), Item::Numeric(Numeric::Minute, Pad::Zero),
            Item::Space(""), Item::Literal(":"),
            Item::Space(""), Item::Numeric(Numeric::Second, Pad::Zero),
            Item::Fixed(Fixed::Nanosecond), Item::Space(""),
        ];

        let mut parsed = Parsed::new();
        try!(parse(&mut parsed, s, ITEMS.iter().cloned()));
        parsed.to_naive_datetime_with_offset(0)
    }
}

*/