use crate::init::RuscInitBuilder;

mod init;
mod lines;
pub mod prelude;
mod streaming_lines;

pub fn init(verbose: bool) {
    let mut builder = RuscInitBuilder::default();
    if verbose {
        builder = builder.verbose();
    }
    builder.init();
}
