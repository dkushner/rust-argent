#![crate_type = "lib"]
#![crate_name = "argent"]

use std::fmt::{Display, Formatter, Result};
use std::ops::{Add, Sub};

#[derive(Debug, Clone, PartialEq)]
pub struct Currency {
    pub iso_code: Option<String>,
    pub iso_numeric: Option<i16>,
    pub name: Option<String>,
    pub symbol: String,
    pub minor_ratio: i32,
    pub symbol_first: bool,
    pub decimal_mark: char,
    pub group_mark: char
}

impl Currency {
    pub fn simple(symbol: &'static str) -> Currency {
        Currency { symbol: symbol.into(), ..Default::default() }
    }
}

#[derive(Debug, Clone)]
pub struct Amount<'a> {
    pub currency: &'a Currency, 
    pub quantity: i64
}

impl <'a> Amount<'a> {
    pub fn of(currency: &'a Currency, quantity: i64) -> Amount<'a> {
        Amount{ currency: currency, quantity: quantity }
    }
}

impl Default for Currency {
    fn default() -> Currency {
        Currency {
            iso_code: None,
            iso_numeric: None,
            name: None,
            symbol: "".to_string(),
            minor_ratio: 100,
            symbol_first: true,
            decimal_mark: '.',
            group_mark: ','
        }
    }
}

impl <'a> Display for Amount<'a> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        let currency: &Currency = &self.currency;

        let decimal_count = (currency.minor_ratio as f32).log(10.0) as usize;
        let raw = format!("{:.*}", decimal_count, (self.quantity  as f32 / currency.minor_ratio as f32));
        let parts: Vec<&str> = raw.as_str().split(currency.decimal_mark).collect();
        
        if let (Some(significand), Some(mantissa)) = (parts.get(0), parts.get(1)) {
            let mut groups = (RevChunks { target: significand, size: 3 }).collect::<Vec<_>>();

            groups.reverse();

            let whole = groups.join(&currency.group_mark.to_string());
            write!(f, "{}{}{}{}", currency.symbol, whole, currency.decimal_mark, mantissa)
        } else { 
            write!(f, "{}NaN", currency.symbol)
        }

    }
}

impl <'a> Add for Amount<'a> {
    type Output = Amount<'a>;

    fn add(self, rhs: Amount) -> Amount<'a> {
        if rhs.currency == self.currency {
            Amount { currency: self.currency, quantity: self.quantity + rhs.quantity }
        } else {
            self
        }
    }
}

impl <'a> Sub for Amount<'a> {
    type Output = Amount<'a>;

    fn sub(self, rhs: Amount) -> Amount<'a> {
        if rhs.currency == self.currency {
            Amount { currency: self.currency, quantity: self.quantity - rhs.quantity }
        } else {
            self
        }
    }
}


struct RevChunks<'a> {
    target: &'a str,
    size: usize
}

impl<'a> Iterator for RevChunks<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<&'a str> {
        if self.target.is_empty() {
            return None;
        }

        let mut end = 0;

        for (n, (i, _)) in self.target.char_indices().rev().enumerate() {
            if n == self.size {
                break;
            }
            end = i;
        }

        let (a, b) = self.target.split_at(end);
        self.target = a;

        Some(b)
    }
}
