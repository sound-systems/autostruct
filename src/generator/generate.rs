// use crate::database::TableColumn;

use anyhow::Error;
use cruet::Inflector;

use crate::database::{TableInfo, TableInfoProvider};

pub fn code_from(
    table: &TableInfo,
    info_provider: &impl TableInfoProvider,
) -> Result<String, Error> {
    let mut code = String::new();
    code.push_str("#![allow(dead_code)]\n");
    code.push_str(
        "// Generated with autostruct\n// https://github.com/sound-systems/autostruct\n\n",
    );

    code.push_str(&format!("pub struct {} {{\n", table.name.to_pascal_case()));

    for column in &table.columns {
        let rust_type = info_provider.type_name_from(column);
        let field_name = column.name.to_snake_case();
        let struct_field = format!("    pub {field_name}: {rust_type},\n");
        code.push_str(&struct_field);
    }

    code.push('}');

    Ok(code)
}
