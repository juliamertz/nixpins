mod emit;
mod fetcher;
mod pins;
mod prefetch;
mod url;

use anyhow::Result;
use clap::{Parser, Subcommand};
use fetcher::Source;
use pins::Pins;
use std::path::PathBuf;
use url::Url;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Source to operate on
    #[arg(default_value = "pins.nix")]
    file: PathBuf,

    #[arg(short, long)]
    dry: bool,

    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Create a new pins.nix in the current directory
    Init,

    /// Show pins and their versions
    Show {
        /// Name of pin
        #[arg(short, long)]
        name: Option<String>,
    },

    /// Add a new pin
    Add {
        /// Input url
        url: String,

        #[arg(short, long)]
        flake: bool,

        /// Name to addres this pin by
        #[arg(short, long)]
        name: Option<String>,
    },

    /// Remove a pin
    Remove {
        /// Pin name
        name: String,
    },

    /// Update pin sources
    Update {
        /// Name of pin to update
        name: Option<String>,
        // #[arg(short, long)]
        // recursive: bool,
    },
}

pub fn main() -> Result<()> {
    colog::init();
    let args = Cli::parse();

    match args.command {
        Command::Init => {
            let pins = Pins::default();
            pins.write_to_file(&args.file)?;
        }

        Command::Show { .. } => {
            if !&args.file.exists() {
                anyhow::bail!("Cannot find {:?} in current directory", &args.file);
            }

            let pins = Pins::read_from_file(&args.file)?;

            for (key, _pin) in pins.inputs.iter() {
                match pins.sources.get(key) {
                    Some(source) => log::info!(
                        "{key}:\n  rev = '{rev}'\n  hash = '{hash}'",
                        rev = source.version(),
                        hash = source.hash()
                    ),
                    None => log::warn!(
                        "Missing source for {key} run '{package_name} update' to fix this",
                        package_name = env!("CARGO_PKG_NAME")
                    ),
                };
            }
        }

        Command::Add { url, name, flake } => {
            if !&args.file.exists() {
                anyhow::bail!("Cannot find {:?} in current directory", &args.file);
            }

            let url = Url::try_from(url)?;
            let mut pins = Pins::read_from_file(&args.file)?;
            pins.add(url, name, flake)?;
            if !args.dry {
                pins.write_to_file(&args.file)?;
            }
        }

        Command::Remove { name } => {
            if !&args.file.exists() {
                anyhow::bail!("Cannot find {:?} in current directory", &args.file);
            }

            let mut pins = Pins::read_from_file(&args.file)?;
            pins.remove(&name);
            if !args.dry {
                pins.write_to_file(&args.file)?;
            }
        }

        Command::Update { name } => {
            if !&args.file.exists() {
                anyhow::bail!("Cannot find {:?} in current directory", &args.file);
            }

            let mut pins = Pins::read_from_file(&args.file)?;
            match name {
                Some(ref name) => pins.update(name)?,
                None => pins.update_all()?,
            }
            if !args.dry {
                pins.write_to_file(&args.file)?;
            }
        }
    }

    Ok(())
}
