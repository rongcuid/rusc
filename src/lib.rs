mod init;
mod lines;
mod streaming_lines;

pub use init::RuscInitBuilder;
pub use lines::*;
pub use streaming_lines::*;

pub fn init(verbose: bool) {
    let mut builder = RuscInitBuilder::default();
    if verbose {
        builder = builder.verbose();
    }
    builder.init();
}
