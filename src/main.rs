use std::{env, io};
use std::fs;
use std::io::{BufWriter, prelude::*};
use std::path::{Path, PathBuf};

use clap::Parser;

use cli::Cli;

use crate::cli::OutputFormat;
use crate::output::Item;

mod xml;
mod output;
mod cli;

fn default_input_xml_path() -> Result<PathBuf, String> {
    match env::consts::OS {
        "windows" => {
            let path = env::var("LOCALAPPDATA").unwrap();
            let path = Path::new(&path).join("Amazon/Kindle/Cache/KindleSyncMetadataCache.xml");
            Ok(path)
        }

        "macos" => {
            let path = env::var("HOME").unwrap();
            let path = Path::new(&path).join("Library/Containers/com.amazon.Kindle/Data/Library/Application Support/Amazon/Kindle/Cache/KindleSyncMetadataCache.xml");
            Ok(path)
        }

        // Not supported Linux and other OS
        _ => Err(format!("Not supported OS: {}", env::consts::OS)),
    }
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    let input_xml_path = if let Some(input_xml_path) = cli.input_xml {
        input_xml_path
    } else {
        match default_input_xml_path() {
            Ok(path) => path,
            Err(e) => {
                anyhow::bail!("{}", e);
            }
        }
    };

    if !input_xml_path.exists() {
        anyhow::bail!("Input file not found: {:?}", input_xml_path);
    }

    let output_writer = if let Some(output_path) = cli.output {
        Box::new(BufWriter::new(fs::File::create(output_path)?)) as Box<dyn Write>
    } else {
        Box::new(io::stdout().lock()) as Box<dyn Write>
    };

    let format = cli.format;

    let mut content: xml::Response = {
        let mut content = String::new();
        // content.push_str(r#"<?xml version="1.0" encoding="UTF-8"?>"#);
        content.push_str(fs::read_to_string(input_xml_path)?.as_str());
        quick_xml::de::from_str(content.as_str())?
    };
    let items: Vec<Item> = content.add_update_list.meta_data.into_iter().map(|x| x.into()).collect();

    match format {
        OutputFormat::Json => serde_json::to_writer_pretty(output_writer, &items)?,
        OutputFormat::Yaml => serde_yaml::to_writer(output_writer, &items)?,
    }

    Ok(())
}
