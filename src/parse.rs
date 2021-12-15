use core::num;
use std::io;

use anyhow::Result;
use thiserror::Error;

/// An error raised during parsing.
#[derive(Debug, Error)]
pub enum ParseError {
    #[error("failed to parse int")]
    ParseInt(
        #[source]
        #[from]
        num::ParseIntError,
    ),
    #[error("failed to parse float")]
    ParseFloat(
        #[source]
        #[from]
        num::ParseFloatError,
    ),
    #[error("i/o error")]
    Io(
        #[source]
        #[from]
        io::Error,
    ),
    #[error("missing item in line")]
    MissingItem,
    #[error("missing line")]
    MissingLine,
    #[error("failed to parse")]
    Custom(
        #[source]
        #[from]
        anyhow::Error,
    ),
}

/// Parser helper.
#[derive(Debug)]
pub struct Parser<'a> {
    input: &'a str,
}

impl<'a> Parser<'a> {
    /// Parse lines.
    pub fn new(input: &'a str) -> Self {
        Self { input }
    }

    /// Coerce into underlying string.
    pub fn into_str(self) -> &'a str {
        self.input
    }

    /// Parse the next line as input.
    pub fn parse<T: 'a>(&mut self) -> Result<T, ParseError>
    where
        T: Parseable<'a>,
    {
        T::parse(self)
    }

    /// Get the next item as split by whitespace.
    pub fn item(&mut self) -> Result<&'a str, ParseError> {
        self.next_item().ok_or_else(|| ParseError::MissingItem)
    }

    /// Get the next line.
    pub fn line(&mut self) -> Result<Parser<'a>, ParseError> {
        let p = self.next_line().ok_or_else(|| ParseError::MissingLine)?;

        Ok(p)
    }

    /// Get the next line.
    pub fn next_line(&mut self) -> Option<Parser<'a>> {
        if let Some((part, rest)) = self.input.split_once('\n') {
            self.input = rest;
            return Some(Parser { input: part.trim() });
        }

        let input = std::mem::take(&mut self.input);

        if !input.is_empty() {
            return Some(Parser { input });
        }

        None
    }

    /// Parse the next item or raise an error.
    pub fn next_item(&mut self) -> Option<&'a str> {
        if let Some((part, rest)) = self
            .input
            .trim_start_matches(char::is_whitespace)
            .split_once(char::is_whitespace)
        {
            self.input = rest;
            return Some(part);
        }

        let s = std::mem::take(&mut self.input);

        if !s.is_empty() {
            return Some(s);
        }

        None
    }
}

/// Parse a single line of input into the given output.
pub fn parse<'de, T: 'de>(line: &'de str) -> Result<T, ParseError>
where
    T: Parseable<'de>,
{
    let mut p = Parser { input: line };
    T::parse(&mut p)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[non_exhaustive]
pub struct Skip;

/// A trait for things that can be parsed.
pub trait Parseable<'de>: Sized {
    fn parse(p: &mut Parser<'de>) -> Result<Self, ParseError>;
}

impl<'de> Parseable<'de> for Skip {
    fn parse(p: &mut Parser<'de>) -> Result<Self, ParseError> {
        p.item()?;
        Ok(Skip)
    }
}

impl<'de> Parseable<'de> for &'de str {
    fn parse(p: &mut Parser<'de>) -> Result<Self, ParseError> {
        p.item()
    }
}

macro_rules! parse_int {
    ($ty:ty) => {
        impl Parseable<'_> for $ty {
            fn parse(p: &mut Parser<'_>) -> Result<Self, ParseError> {
                let item = p.item()?;
                Ok(item.parse()?)
            }
        }
    };
}

parse_int!(isize);
parse_int!(usize);

parse_int!(u8);
parse_int!(u16);
parse_int!(u32);
parse_int!(u64);
parse_int!(u128);

parse_int!(i8);
parse_int!(i16);
parse_int!(i32);
parse_int!(i64);
parse_int!(i128);

parse_int!(f32);
parse_int!(f64);

macro_rules! parse_tuple {
    ($first_ty:ident $first_var:ident $(, $ty:ident $var:ident)*) => {
        impl<'de, $first_ty, $($ty,)*> Parseable<'de> for ($first_ty, $($ty,)*) where $first_ty: Parseable<'de>, $($ty: Parseable<'de>),* {
            fn parse(p: &mut Parser<'de>) -> Result<Self, ParseError> {
                let $first_var = $first_ty::parse(p)?;
                $(let $var = $ty::parse(p)?;)*
                Ok(($first_var, $($var,)*))
            }
        }

        parse_tuple!($($ty $var),*);
    };

    () => {};
}

parse_tuple!(A a, B b, C c, D d, E e);

impl<'de, T, const N: usize> Parseable<'de> for [T; N]
where
    T: Copy + Default + Parseable<'de>,
{
    fn parse(p: &mut Parser<'de>) -> Result<Self, ParseError> {
        let mut init = [T::default(); N];

        for out in init.iter_mut() {
            *out = T::parse(p)?;
        }

        Ok(init)
    }
}
