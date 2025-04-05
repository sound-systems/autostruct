use std::{path::Path, time::Duration};

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

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Framework {
    None,
    Sqlx,
}

impl Default for Framework {
    fn default() -> Self {
        Self::None
    }
}

pub struct Arguments {
    pub target_dir: String,
    pub exclude_tables: Vec<String>,
    pub connection_string: String,
    pub singular_table_names: bool,
    pub framework: Framework,
    pub timeout: Duration,
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
            framework: Framework::None,
            timeout: Duration::from_secs(5),
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
        framework,
        timeout,
    } = args;

    let provider = utils::setup(&connection_string, exclude_tables, timeout).await?;
    let generator = code::Generator::new(
        Options {
            singular: singular_table_names,
            framework,
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

    // Collect module names and write individual files
    let mut module_names: Vec<(String, String)> = Vec::new();
    for snippet in code_snippets {
        let file_name = snippet.id.to_snake_case();
        module_names.push((file_name.clone(), snippet.id.to_pascal_case()));

        let source_file = output_dir.join(format!("{file_name}.rs"));
        let mut code = String::new();
        code.push_str(
            "// Generated with autostruct\n// https://github.com/sound-systems/autostruct\n\n",
        );
        code.push_str(&snippet.code);
        let mut file = File::create(source_file)
            .await
            .context("failed to create source code file")?;
        file.write_all(code.as_bytes())
            .await
            .context("failed to write generated source code to file")?;
    }

    // Generate mod.rs with public module declarations
    let mod_file_path = output_dir.join("mod.rs");
    let mut mod_contents = String::from(
        "// Generated with autostruct\n// https://github.com/sound-systems/autostruct\n\n",
    );

    for module_name in module_names {
        mod_contents.push_str(&format!("mod {};\n", module_name.0));
        mod_contents.push_str(&format!("pub use {}::{};\n", module_name.0, module_name.1));
    }

    let mut mod_file = File::create(mod_file_path)
        .await
        .context("failed to create mod.rs file")?;

    mod_file
        .write_all(mod_contents.as_bytes())
        .await
        .context("failed to write mod.rs contents")?;

    Ok(())
}
