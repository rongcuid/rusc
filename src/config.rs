use clap::Args;
use tracing::{debug, Level};
use tracing_indicatif::IndicatifLayer;
use tracing_subscriber::{prelude::*, EnvFilter};

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

        let env_filter = EnvFilter::builder()
            .from_env_lossy()
            .add_directive(max_level.into());
        tracing_subscriber::registry()
            .with(env_filter)
            .with(
                tracing_subscriber::fmt::layer()
                    .without_time()
                    .compact()
                    .with_writer(
                        indicatif_layer
                            .get_stderr_writer()
                            .with_max_level(Level::TRACE),
                    ),
            )
            .with(indicatif_layer)
            .init();
        debug!("Enabling log level: {}", max_level);
    }
}
