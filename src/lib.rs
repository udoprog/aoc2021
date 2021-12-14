use std::env;
use std::fs;
use std::path::PathBuf;

use anyhow::{anyhow, Result};

mod parse;
pub use self::parse::{parse, LineParser, ParseError, Parseable, Parser};

pub fn load(name: &str) -> Result<String> {
    let dir =
        env::var_os("CARGO_MANIFEST_DIR").ok_or_else(|| anyhow!("missing CARGO_MANIFEST_DIR"))?;
    let p = PathBuf::from(&dir).join("input").join(name);
    let string = String::from_utf8(fs::read(&p)?)?;
    Ok(string)
}

/// Convert the given bufreader into input lines using the given [Parseable] as
/// a template.
pub fn lines<T>(buf: String) -> Result<Vec<T>>
where
    T: Parseable,
{
    let mut out = Vec::new();

    for line in buf.lines() {
        out.push(parse(line)?);
    }

    Ok(out)
}
