use crate::cli::RuscCli;

mod cli;
mod lines;
pub mod prelude;

/// Initializes tracing and indicatif
pub fn init_from_cli(cli: &RuscCli) {
    cli.init();
}

/// Initializes with default args
pub fn init() {
    RuscCli::default().init();
}
