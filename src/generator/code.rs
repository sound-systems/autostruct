use crate::{database, rust::Type};
use anyhow::Error;
use cruet::Inflector;
use std::collections::HashSet;

pub struct Snippet {
    pub id: String,
    pub imports: HashSet<String>,
    pub code: String,
}

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

        Ok(snippets)
    }

    fn code_from_enums(&self, enums: &[database::Enum]) -> Vec<Snippet> {
        enums
            .iter()
            .map(|e| {
                let mut code = String::new();
                let name = e.name.to_pascal_case();
                code.push_str(&format!("pub enum {} {{\n", name));

                for value in &e.values {
                    let field_name = value.name.to_pascal_case();
                    let enum_field = format!("    {field_name},\n");
                    code.push_str(&enum_field);
                }

                code.push('}');

                Snippet {
                    id: name,
                    imports: Default::default(),
                    code,
                }
            })
            .collect()
    }

    fn code_from_composites(&self, composites: &[database::CompositeType]) -> Vec<Snippet> {
        composites
            .iter()
            .map(|composite| {
                let mut code = String::new();
                let table_name = self.format_name(&composite.name);
                code.push_str(&format!("pub struct {} {{\n", table_name.to_pascal_case()));

                for attr in &composite.attributes {
                    let rust_type = self.provider.type_name_from(&attr.data_type);
                    let field_name = attr.name.to_snake_case();
                    let struct_field = format!("    pub {field_name}: {rust_type},\n");
                    code.push_str(&struct_field);
                }

                code.push('}');

                Snippet {
                    id: table_name,
                    imports: Default::default(),
                    code,
                }
            })
            .collect()
    }

    fn code_from_tables(&self, tables: &[database::Table]) -> Vec<Snippet> {
        tables
            .iter()
            .map(|table| {
                let mut code = String::new();
                let table_name = self.format_name(&table.name);
                code.push_str(&format!("pub struct {} {{\n", table_name.to_pascal_case()));

                for column in &table.columns {
                    let mut rust_type = self.provider.type_name_from(&column.udt_name);
                    if column.is_nullable {
                        rust_type = Type::Option(Box::new(rust_type))
                    }
                    let field_name = column.name.to_snake_case();
                    let struct_field = format!("    pub {field_name}: {rust_type},\n");
                    code.push_str(&struct_field);
                }

                code.push('}');

                Snippet {
                    id: table_name,
                    imports: Default::default(),
                    code,
                }
            })
            .collect()
    }

    fn format_name(&self, name: &str) -> String {
        if self.formatting.singular {
            name.to_singular()
        } else {
            name.to_string()
        }
    }
}
