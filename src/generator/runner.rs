use std::path::Path;

use anyhow::{Context, Error};
use cruet::Inflector;
use tokio::{
    fs::{self, File},
    io::AsyncWriteExt,
};

use super::{
    code::{self, Options},
    utils,
};

pub struct Arguments {
    pub target_dir: String,
    pub exclude_tables: Vec<String>,
    pub connection_string: String,
    pub singular_table_names: bool,
    pub use_statements: Vec<String>,
    pub derive_statements: Vec<String>,
}

impl Arguments {
    pub fn from(connection_string: &str) -> Self {
        Self {
            connection_string: connection_string.to_string(),
            ..Default::default()
        }
    }
}

impl Default for Arguments {
    fn default() -> Self {
        Self {
            target_dir: "./autostructs".to_string(),
            exclude_tables: Default::default(),
            connection_string: Default::default(),
            singular_table_names: false,
            use_statements: Default::default(),
            derive_statements: Default::default(),
        }
    }
}

/// Executes the code generation process using the provided arguments
///
/// # Arguments
///
/// * `args` - Configuration options for code generation including:
///   - `exclude_tables`: Tables to exclude from generation
///   - `connection_string`: Database connection string
///   - `target_dir`: Output directory for generated files
///   - `singular_table_names`: Whether to use singular form of table names
///
/// # Returns
///
/// Returns `Ok(())` if code generation succeeds, otherwise returns an `Error`
///
/// # Errors
///
/// This function will return an error if:
/// - Database connection fails
/// - Output directory creation fails
/// - File operations fail during code writing
pub async fn run(args: Arguments) -> Result<(), Error> {
    let Arguments {
        exclude_tables,
        connection_string,
        target_dir,
        singular_table_names,
        use_statements,
        derive_statements,
    } = args;

    let provider = utils::setup(&connection_string, exclude_tables).await?;
    let generator = code::Generator::new(
        Options {
            singular: singular_table_names,
        },
        Box::new(provider),
    );

    let code_snippets = generator.generate_code().await?;

    let output_dir = Path::new(&target_dir);
    if !output_dir.exists() {
        fs::create_dir_all(output_dir)
            .await
            .context("failed to create directory that generated source code will be written to")?;
    }

    for snippet in code_snippets {
        let file_name = snippet.id.to_snake_case();
        let source_file = output_dir.join(format!("{file_name}.rs"));
        let mut code = String::new();
        code.push_str("#![allow(dead_code)]\n");
        code.push_str(
            "// Generated with autostruct\n// https://github.com/sound-systems/autostruct\n\n",
        );
        for (i, use_statement) in use_statements.iter().enumerate() {
            code.push_str(&format!("{use_statement}\n"));
            if i == use_statements.len() - 1 {
                code.push_str("\n");
            }
        }
        for (i, derive_statement) in derive_statements.iter().enumerate() {
            if i == 0 {
                code.push_str("#[derive(");
            }
            code.push_str(&derive_statement);
            if i == derive_statements.len() - 1 {
                code.push_str(")]\n");
            }
        }
        code.push_str(&snippet.code);
        let mut file = File::create(source_file)
            .await
            .context("failed to create source code file")?;
        file.write_all(code.as_bytes())
            .await
            .context("failed to write generated source code to file")?;
    }

    Ok(())
}
