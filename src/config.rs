use clap::Args;
use tracing::Level;
use tracing_indicatif::IndicatifLayer;
use tracing_subscriber::prelude::*;
use tracing_subscriber::EnvFilter;

pub trait HasRuscConfig {
    fn rusc_config<'a>(&'a self) -> &'a RuscConfig;
}

#[derive(Debug, Args, Default)]
pub struct RuscConfig {
    #[arg(short, long, action = clap::ArgAction::Count, global = true)]
    pub verbose: u8,
}

impl RuscConfig {
    pub fn init(&self) {
        let indicatif_layer = IndicatifLayer::new();
        let max_level = match self.verbose {
            0 => Level::WARN,
            1 => Level::INFO,
            2 => Level::DEBUG,
            _ => Level::TRACE,
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
            .with(EnvFilter::from_default_env())
            .init();
    }
}
