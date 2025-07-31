use std::{
    fs::File,
    io::Write,
    path::{Path, PathBuf},
};

use clap::{Parser, Subcommand};
// use service::error::Result;
use eyre::Result;

#[derive(Debug, Parser)]
#[command(author, version, about)]
struct Opts {
    #[command(subcommand)]
    op: Operation,
}

#[derive(Subcommand, Debug)]
enum Operation {
    /// Generate a config file
    GenConfig {
        /// The path to the output file
        #[arg(short, long, default_value_os_t = PathBuf::from("config.toml"), value_hint = clap::ValueHint::FilePath, value_name = "FILE" )]
        output: PathBuf,
    },
    /// Run the server
    Serve {
        #[arg(short, long, default_value_os_t = PathBuf::from("config.toml"), value_hint = clap::ValueHint::FilePath, value_name = "FILE" )]
        config: PathBuf,
    },
    /// TODO: Control the server
    Ctl,
}

#[tokio::main]
async fn main() -> Result<()> {
    let opts = Opts::parse();
    match opts.op {
        Operation::GenConfig { output } => gen_default_config(output)?,
        Operation::Serve { config } => {
            register_config_path(config);
            service::serve().await?
        }
        Operation::Ctl => todo!(),
    };
    Ok(())
}

fn gen_default_config<T: AsRef<Path>>(path: T) -> Result<()> {
    let settings = settings::Settings::default();
    // dbg!(&settings);
    let mut file = File::create_new(path)?;
    let s = toml::to_string(&settings)?;
    file.write_all(s.as_bytes())?;
    Ok(())
}

fn register_config_path<T: AsRef<Path>>(path: T) {
    settings::set_default_config_path(path);
}
