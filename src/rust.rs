// An enum to represent Rust types
#[derive(Debug)]
pub enum Type {
    Bool,
    I8,
    I16,
    I32,
    I64,
    F32,
    F64,
    String,
    Vec(Box<Type>),
    Option(Box<Type>),
    DateTime,
    Json,
    Uuid,
    Custom(String),
}

// Implement display for a rust:Type to visualize the mapping
impl std::fmt::Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Type::Bool => write!(f, "bool"),
            Type::I8 => write!(f, "i8"),
            Type::I16 => write!(f, "i16"),
            Type::I32 => write!(f, "i32"),
            Type::I64 => write!(f, "i64"),
            Type::F32 => write!(f, "f32"),
            Type::F64 => write!(f, "f64"),
            Type::String => write!(f, "String"),
            Type::Vec(inner) => write!(f, "Vec<{}>", inner),
            Type::Option(inner) => write!(f, "Option<{}>", inner),
            Type::DateTime => write!(f, "DateTime<Utc>"),
            Type::Json => write!(f, "serde_json::Value"),
            Type::Uuid => write!(f, "uuid::Uuid"),
            Type::Custom(ty) => write!(f, "{}", ty),
        }
    }
}
