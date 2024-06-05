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
    ByteArray,
    Unit,
    PgInterval,
    PgRange(Box<Type>),
    PgMoney,
    PgLTree,
    PgLQuery,
    PgCiText,
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
            Type::ByteArray => write!(f, "Vec<u8>"),
            Type::Unit => write!(f, "()"),
            Type::PgInterval => write!(f, "PgInterval"),
            Type::PgRange(inner) => write!(f, "PgRange<{}>", inner),
            Type::PgMoney => write!(f, "PgMoney"),
            Type::PgLTree => write!(f, "PgLTree"),
            Type::PgLQuery => write!(f, "PgLQuery"),
            Type::PgCiText => write!(f, "PgCiText"),
            Type::Custom(ty) => write!(f, "{}", ty),
        }
    }
}
