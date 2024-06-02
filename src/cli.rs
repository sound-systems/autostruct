use clap::{Parser, Subcommand, ValueEnum};

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

    /// If provided, the struct will have derive macros for the respective database client added
    #[arg(long, value_enum)]
    pub client: Option<Client>,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
pub enum Client {
    SQLX,
}
