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
