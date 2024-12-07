// An enum to represent Rust types
#[derive(Debug)]
pub enum Type {
    Bit(&'static str),
    Bool(&'static str),
    I8(&'static str),
    I16(&'static str),
    I32(&'static str),
    I64(&'static str),
    U32(&'static str),
    F32(&'static str),
    F64(&'static str),
    Uuid(&'static str),
    Date(&'static str),
    Time(&'static str),
    Timestamp(&'static str),
    TimestampWithTz(&'static str),
    Decimal(&'static str),
    IpNetwork(&'static str),
    String(&'static str),
    Json(&'static str),
    Xml(&'static str),
    ByteArray(&'static str),
    Unit(&'static str),
    Interval(&'static str),
    Range(Box<Type>),
    Money(&'static str),
    Tree(&'static str),
    Query(&'static str),
    Void(&'static str),
    Option(Box<Type>),
    Vector(Box<Type>),
    Custom(String),
}

// Implement display for a rust:Type to visualize the mapping
impl std::fmt::Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            // Basic types with static str names
            Type::Bit(name) |
            Type::Bool(name) |
            Type::I8(name) |
            Type::I16(name) |
            Type::I32(name) |
            Type::I64(name) |
            Type::U32(name) |
            Type::F32(name) |
            Type::F64(name) |
            Type::Uuid(name) |
            Type::Date(name) |
            Type::Time(name) |
            Type::Timestamp(name) |
            Type::TimestampWithTz(name) |
            Type::Decimal(name) |
            Type::IpNetwork(name) |
            Type::String(name) |
            Type::Json(name) |
            Type::Xml(name) |
            Type::ByteArray(name) |
            Type::Unit(name) |
            Type::Interval(name) |
            Type::Money(name) |
            Type::Tree(name) |
            Type::Query(name) |
            Type::Void(name) => write!(f, "{name}"),

            // Container types that wrap other types
            Type::Vector(inner) => write!(f, "Vec<{inner}>"),
            Type::Option(inner) => write!(f, "Option<{inner}>"),
            Type::Range(inner) => write!(f, "Range<{inner}>"),

            // Custom type that owns a String
            Type::Custom(name) => write!(f, "{name}"),
        }
    }
}
