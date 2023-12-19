pub enum AttrType {
    UNDEFINED,
    CHARS,
    INTS,
    FLOATS,
    BOOLEANS,
}

pub struct AttrInfoSqlNode {
    field_type: AttrType,
    name: String,
    length: i32,
}
