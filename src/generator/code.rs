use crate::{database, rust::Type};
use anyhow::Error;
use cruet::Inflector;
use std::collections::HashSet;

/**
Contains fields that indicate formatting options that should be applied to the generated code

# Fields
- `singular`: specifies with the generated Rust structs name should be the singular form the provided tables
*/
pub struct Options {
    pub singular: bool,
}

pub struct Generator {
    formatting: Options,
    provider: Box<dyn database::InfoProvider>,
}

impl Generator {
    pub fn new(formatting: Options, provider: Box<dyn database::InfoProvider>) -> Self {
        Generator {
            formatting,
            provider,
        }
    }

    pub async fn generate_code(&self) -> Result<Vec<Snippet>, Error> {
        let schema = self.provider.get_schema().await?;
        let mut snippets: Vec<Snippet> = vec![];
        snippets.append(&mut self.code_from_enums(&schema.enumerations));
        snippets.append(&mut self.code_from_composites(&schema.composite_types));
        snippets.append(&mut self.code_from_tables(&schema.tables));

        // Finalize all snippets
        for snippet in &mut snippets {
            snippet.finalize();
        }

        Ok(snippets)
    }

    fn code_from_enums(&self, enums: &[database::Enum]) -> Vec<Snippet> {
        enums
            .iter()
            .map(|e| {
                let name = e.name.to_pascal_case();
                let mut snippet = Snippet::new(name.clone());
                
                snippet.code.push_str("#[derive(Debug, Clone, PartialEq, Eq)]\n");
                snippet.code.push_str(&format!("pub enum {} {{\n", name));

                for value in &e.values {
                    let field_name = value.name.to_pascal_case();
                    let enum_field = format!("    {field_name},\n");
                    snippet.code.push_str(&enum_field);
                }

                snippet.code.push('}');
                snippet
            })
            .collect()
    }

    fn code_from_composites(&self, composites: &[database::CompositeType]) -> Vec<Snippet> {
        composites
            .iter()
            .map(|composite| {
                let table_name = self.format_name(&composite.name);
                let mut snippet = Snippet::new(table_name.clone());
                
                snippet.code.push_str("#[derive(Debug, Clone)]\n");
                snippet.code.push_str(&format!("pub struct {} {{\n", table_name.to_pascal_case()));

                for attr in &composite.attributes {
                    let rust_type = self.provider.type_name_from(&attr.data_type);
                    self.add_type_imports(&mut snippet, &rust_type);
                    
                    let field_name = attr.name.to_snake_case();
                    let struct_field = format!("    pub {field_name}: {rust_type},\n");
                    snippet.code.push_str(&struct_field);
                }

                snippet.code.push('}');
                snippet
            })
            .collect()
    }

    fn code_from_tables(&self, tables: &[database::Table]) -> Vec<Snippet> {
        tables
            .iter()
            .map(|table| {
                let table_name = self.format_name(&table.name);
                let mut snippet = Snippet::new(table_name.clone());
                
                snippet.code.push_str("#[derive(Debug, Clone)]\n");
                snippet.code.push_str(&format!("pub struct {} {{\n", table_name.to_pascal_case()));

                for column in &table.columns {
                    let mut rust_type = self.provider.type_name_from(&column.udt_name);
                    
                    // Handle foreign key references
                    if let Some(fk_table) = &column.foreign_key_table {
                        let fk_type = self.format_name(fk_table).to_pascal_case();
                        snippet.add_dependency(&fk_type);
                    }
                    
                    if column.is_nullable {
                        rust_type = Type::Option(Box::new(rust_type));
                    }
                    
                    self.add_type_imports(&mut snippet, &rust_type);
                    
                    let field_name = column.name.to_snake_case();
                    let struct_field = format!("    pub {field_name}: {rust_type},\n");
                    snippet.code.push_str(&struct_field);
                }

                snippet.code.push('}');
                snippet
            })
            .collect()
    }

    fn add_type_imports(&self, snippet: &mut Snippet, rust_type: &Type) {
        match rust_type {
            Type::Uuid(_) => snippet.add_import("uuid::Uuid"),
            Type::Date(_) => snippet.add_import("chrono::NaiveDate"),
            Type::Time(_) => snippet.add_import("chrono::NaiveTime"),
            Type::Timestamp(_) => snippet.add_import("chrono::NaiveDateTime"),
            Type::TimestampWithTz(_) => {
                snippet.add_import("chrono::{DateTime, Utc}");
            },
            Type::Decimal(_) => snippet.add_import("rust_decimal::Decimal"),
            Type::IpNetwork(_) => snippet.add_import("ipnetwork::IpNetwork"),
            Type::Json(_) => snippet.add_import("serde_json::Value"),
            Type::Tree(_) => snippet.add_import("postgres_types::LTree"),
            Type::Query(_) => snippet.add_import("postgres_types::TSQuery"),
            Type::Option(inner) => self.add_type_imports(snippet, inner),
            Type::Vector(inner) => self.add_type_imports(snippet, inner),
            Type::Range(inner) => {
                snippet.add_import("std::ops::Range");
                self.add_type_imports(snippet, inner);
            },
            Type::Custom(name) => {
                // If it's a custom type from our schema, add it as a dependency
                if !name.contains("::") {
                    snippet.add_dependency(name);
                } else if name.starts_with("postgres_types::") {
                    // Extract the type name after postgres_types::
                    let type_name = name.strip_prefix("postgres_types::").unwrap();
                    snippet.add_import(&format!("postgres_types::{}", type_name));
                }
            },
            _ => {}
        }
    }

    fn format_name(&self, name: &str) -> String {
        if self.formatting.singular {
            name.to_singular()
        } else {
            name.to_string()
        }
    }
}

pub struct Snippet {
    pub id: String,
    pub imports: HashSet<String>,
    pub code: String,
    pub dependencies: HashSet<String>,  // Track other structs this one depends on
}

impl Snippet {
    fn new(id: String) -> Self {
        Self {
            id,
            imports: HashSet::new(),
            code: String::new(),
            dependencies: HashSet::new(),
        }
    }

    fn add_import(&mut self, import: &str) {
        self.imports.insert(import.to_string());
    }

    fn add_dependency(&mut self, dependency: &str) {
        self.dependencies.insert(dependency.to_string());
    }

    fn finalize(&mut self) {
        // Add imports at the top of the code
        let mut final_code = String::new();
        
        // Add imports
        for import in &self.imports {
            final_code.push_str(&format!("use {};\n", import));
        }
        
        // Add dependencies as relative imports
        for dep in &self.dependencies {
            final_code.push_str(&format!("use super::{};\n", dep.to_pascal_case()));
        }
        
        if !self.imports.is_empty() || !self.dependencies.is_empty() {
            final_code.push('\n');
        }
        
        final_code.push_str(&self.code);
        self.code = final_code;
    }
}
