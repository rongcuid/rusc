use anyhow::*;
use std::{
    fs::File,
    io::{stdout, Write},
    path::Path,
};

use crate::config::{HasRuscConfig, RuscConfig};

mod config;
mod lines;
pub mod prelude;

/// Initializes by parsing command line
pub fn init_with_cli<T>() -> T
where
    T: HasRuscConfig + clap::Parser,
{
    let cli = T::parse();
    cli.rusc_config().init();
    cli
}

/// Initializes with default args
pub fn init() {
    RuscConfig::default().init();
}

pub fn create_or_stdout(path: Option<&Path>) -> Result<Box<dyn Write>> {
    match path {
        Some(p) => Ok(Box::new(File::create(p)?)),
        None => Ok(Box::new(stdout())),
    }
}
