// #![allow(unused)]
mod error;
mod prelude;
mod utils;
use crate::prelude::*;

use std::fs::read_dir;
fn main() -> Result<()> {
    // println!("Hello, world!");
    for entry in read_dir("/home/ssk/workspace")?.filter_map(|e| e.ok()) {
        // let entry = entry
        //     .path()
        //     .to_str()
        //     .map(String::from)
        //     .ok_or_else(|| Error::Generic("Invalid path".into()))?;
        let entry: String = W(&entry).try_into()?;
        println!("{entry:?}")
    }

    Ok(())
}
