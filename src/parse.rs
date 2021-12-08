use core::num;
use std::str::SplitWhitespace;

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
    #[error("missing item in line")]
    MissingItem,
    #[error("failed to parse")]
    Custom(
        #[source]
        #[from]
        anyhow::Error,
    ),
}

/// Parser helper.
pub struct Parser<'a> {
    it: SplitWhitespace<'a>,
}

impl<'a> Parser<'a> {
    /// Parse the next item or raise an error.
    pub fn next(&mut self) -> Result<&'a str, ParseError> {
        self.it.next().ok_or_else(|| ParseError::MissingItem)
    }
}

/// Parse a single line of input into the given output.
pub fn parse<T>(line: impl AsRef<str>) -> Result<T, ParseError>
where
    T: Parseable,
{
    let line = line.as_ref();

    let mut p = Parser {
        it: line.split_whitespace(),
    };

    T::parse(&mut p)
}

/// A trait for things that can be parsed.
pub trait Parseable: Sized {
    fn parse(p: &mut Parser<'_>) -> Result<Self, ParseError>;
}

macro_rules! parse_int {
    ($ty:ty) => {
        impl Parseable for $ty {
            fn parse(p: &mut Parser<'_>) -> Result<Self, ParseError> {
                Ok(p.next()?.parse()?)
            }
        }
    };
}

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
        impl<$first_ty, $($ty,)*> Parseable for ($first_ty, $($ty,)*) where $first_ty: Parseable, $($ty: Parseable),* {
            fn parse(p: &mut Parser<'_>) -> Result<Self, ParseError> {
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
