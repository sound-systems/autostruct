use anyhow::bail;
use clap::{Parser, Subcommand, ValueEnum};
use humantime::{Duration, DurationError};

use crate::generator::{self};

#[derive(Clone, Debug, ValueEnum)]
pub enum Framework {
    None,
    Sqlx,
}

impl Default for Framework {
    fn default() -> Self {
        Self::None
    }
}

#[derive(Parser)]
#[command(name = "autostruct")]
#[command(version = env!("CARGO_PKG_VERSION"))]
#[command(about = env!("CARGO_PKG_DESCRIPTION"), long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    #[command(
        about = "Generate Rust structs from SQL schema",
        long_about = "Generate Rust structs from SQL schema.\n\n\
                      This command will connect to the specified database using the provided connection string and \
                      generate Rust structs for each table found in the database schema. The structs will be written \
                      to the specified output directory. Please run this command with --help to see what arguments can be used to configure it"
    )]
    Generate(GenerateArgs),
}

#[derive(Parser, Debug)]
pub struct GenerateArgs {
    /// Sets the directory in which the generated files should be written to
    #[arg(short, long, default_value = "./output")]
    pub output: String,

    /// Sets the connection string to the database. It will use the DATABASE_URL environment variable if set
    #[arg(short, long, env = "DATABASE_URL")]
    pub database_url: Option<String>,

    /// Creates struct names in the singular variant of the table name
    #[arg(long, default_value_t = false)]
    pub singular: bool,

    /// Exclude table names from being generated into structs
    #[arg(long)]
    pub exclude: Vec<String>,

    /// Specifies which framework-specific code to generate
    #[arg(long, value_enum, default_value_t = Framework::None)]
    pub framework: Framework,

    /// Sets the connection timeout duration when connecting to the database
    #[arg(short, long, value_parser = parse_duration, default_value = "3s")]
    pub timeout: Duration,
}

fn parse_duration(arg: &str) -> Result<Duration, DurationError> {
    let timeout: Duration = arg.parse()?;
    Ok(timeout)
}

impl TryInto<generator::Arguments> for GenerateArgs {
    type Error = anyhow::Error;

    fn try_into(self) -> Result<generator::Arguments, Self::Error> {
        let conn_str = match self.database_url {
            Some(url) => url,
            None => bail!("no database url provided - please set it via command line arguments or with the DATABASE_URL environment variable"),
        };

        let args = generator::Arguments {
            target_dir: self.output,
            connection_string: conn_str,
            singular_table_names: self.singular,
            exclude_tables: self.exclude,
            framework: match self.framework {
                Framework::None => generator::Framework::None,
                Framework::Sqlx => generator::Framework::Sqlx,
            },
            timeout: self.timeout.into(),
        };

        Ok(args)
    }
}
