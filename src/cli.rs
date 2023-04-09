use std::fmt::Display;
use std::path::PathBuf;

use clap::{Parser, ValueEnum};

/// Convert Kindle for PC data to JSON or YAML
#[derive(Parser, Debug)]
#[command(author, version, about, long_about)]
pub struct Cli {
    /// Input KindleSyncMetadataCache.xml file path
    ///
    /// If not specified, the default path is used.
    ///
    /// The default path is:
    ///
    /// - Windows: %LOCALAPPDATA%\Amazon\Kindle\Cache\KindleSyncMetadataCache.xml
    ///
    /// - macOS: ~/Library/Containers/com.amazon.Kindle/Data/Library/Application Support/Amazon/Kindle/Cache/KindleSyncMetadataCache.xml
    ///
    /// - Other OS: Not supported
    #[arg(short = 'i', long = "input-xml")]
    pub input_xml: Option<PathBuf>,

    /// Output file path. If not specified, the output is written to stdout.
    #[arg(short = 'o', long = "output")]
    pub output: Option<PathBuf>,

    #[arg(short = 'f', long = "format", default_value_t)]
    pub format: OutputFormat,
}

#[derive(Debug, Clone, ValueEnum)]
pub enum OutputFormat {
    /// JSON format
    Json,
    /// YAML format
    Yaml,
}

impl Default for OutputFormat {
    fn default() -> Self {
        Self::Json
    }
}

impl Display for OutputFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Json => write!(f, "json"),
            Self::Yaml => write!(f, "yaml"),
        }
    }
}
