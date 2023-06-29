use tracing::Level;
use tracing_indicatif::IndicatifLayer;
use tracing_subscriber::fmt::writer::MakeWriterExt;
use tracing_subscriber::prelude::*;

pub struct RuscInitBuilder {
    verbose: bool,
}

impl RuscInitBuilder {
    pub fn new() -> Self {
        // let indicatif_layer = IndicatifLayer::new();
        // let sub = tracing_subscriber::fmt()
        //     .without_time()
        //     // .with_writer(indicatif_layer.get_stderr_writer())
        //     // .with(indicatif_layer)
        //     .compact()
        //     .with_max_level(LevelFilter::INFO);
        // Self { sub }
        Self::default()
    }

    pub fn verbose(mut self) -> Self {
        self.verbose = true;
        self
    }

    pub fn init(self) {
        let indicatif_layer = IndicatifLayer::new();
        let max_level = if self.verbose {
            Level::DEBUG
        } else {
            Level::INFO
        };
        tracing_subscriber::registry()
            .with(
                tracing_subscriber::fmt::layer()
                    .without_time()
                    .compact()
                    .with_writer(
                        indicatif_layer
                            .get_stderr_writer()
                            .with_max_level(max_level),
                    ),
            )
            .with(indicatif_layer)
            .init();
    }
}

impl Default for RuscInitBuilder {
    fn default() -> Self {
        Self { verbose: false }
    }
}

pub fn init(verbose: bool) {
    let mut builder = RuscInitBuilder::default();
    if verbose {
        builder = builder.verbose();
    }
    builder.init();
}
