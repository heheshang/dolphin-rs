use std::fs::DirEntry;

use crate::prelude::*;
use format as f;

impl TryFrom<W<&DirEntry>> for String {
    type Error = Error;

    fn try_from(value: W<&DirEntry>) -> Result<String> {
        let entry = value.0;
        let entry = entry
            .path()
            .to_str()
            .map(String::from)
            .ok_or_else(|| Error::Generic(f!("Invalid path {:?}", value.0)))?;
        Ok(entry)
    }
}
