use std::{
    fs::File,
    io::Write,
    path::{Path, PathBuf},
};

use clap::{Parser, Subcommand};
// use service::error::Result;
use eyre::Result;
use migration::{Migrator, MigratorTrait};
use sea_orm::Database;

#[derive(Debug, Parser)]
#[command(author, version, about)]
struct Opts {
    /// Path to config file
    #[arg(
        short,
        long,
        global=true,
        default_value_os_t = PathBuf::from("config.toml"),
        value_hint = clap::ValueHint::FilePath,
        value_name = "FILE"
    )]
    config: PathBuf,

    #[command(subcommand)]
    op: Operation,
}

#[derive(Subcommand, Debug)]
enum Operation {
    /// Generate a config file
    Config {
        #[command(subcommand)]
        op: ConfigSubCommands,
    },
    /// Run the server
    Serve,
    /// TODO: Control the server
    Ctl,

    /// Migrate commands
    Migrate {
        #[command(subcommand)]
        op: MigrateSubcommands,
    },
}
#[derive(Debug, Subcommand)]
enum ConfigSubCommands {
    /// Generate a default config file
    Gen,

    /// Print the config file
    Print,
}

#[derive(Subcommand, Debug)]
enum MigrateSubcommands {
    /// list recent applied migrations
    List {
        /// list size
        #[arg(long, short, default_value_t = 10)]
        num: usize,
    },
    /// Apply migrations
    Apply {
        /// Number of migrations to apply
        #[arg(long, short)]
        num: Option<u32>,
    },
    /// Revert migrations
    Revert {
        /// Number of migrations to revert
        #[arg(long, short, default_value = "1")]
        num: Option<u32>,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    let opts = Opts::parse();
    match opts.op {
        Operation::Config { op } => run_config_command(op, opts.config)?,
        Operation::Serve => {
            register_config_path(opts.config);
            service::serve().await?
        }
        Operation::Migrate { op } => run_migrate_command(op, opts.config).await?,
        _ => todo!(),
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

fn run_config_command(command: ConfigSubCommands, config_path: PathBuf) -> Result<()> {
    match command {
        ConfigSubCommands::Gen => gen_default_config(config_path)?,
        ConfigSubCommands::Print => {
            register_config_path(config_path);
            let settings = settings::get();
            println!("{}", toml::to_string_pretty(&settings)?);
        }
    }
    Ok(())
}

async fn run_migrate_command(command: MigrateSubcommands, config_path: PathBuf) -> Result<()> {
    register_config_path(config_path);
    let settings = settings::get();

    let conn = Database::connect(&settings.db.database_url).await?;
    match command {
        MigrateSubcommands::List { num } => {
            let ms = Migrator::get_migration_with_status(&conn).await?;
            ms[ms.len().saturating_sub(num)..]
                .iter()
                .for_each(|m| println!("{}\t{}", m.status(), m.name()));
            // dbg!(ms);
        }
        MigrateSubcommands::Apply { num } => {
            Migrator::up(&conn, num).await?;
        }
        MigrateSubcommands::Revert { num } => {
            Migrator::down(&conn, num).await?;
        }
    }
    Ok(())
}
