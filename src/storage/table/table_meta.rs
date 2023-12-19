#[derive(Debug)]
pub struct TableMeta {
    table_id: i32,
    name: String,
    fields: Vec<String>,
    indexes: Vec<String>,

    record_size: i32,
}

impl TableMeta {
    pub fn new() -> TableMeta {
        TableMeta {
            table_id: -1,
            name: String::new(),
            fields: Vec::new(),
            indexes: Vec::new(),
            record_size: 0,
        }
    }

    pub fn table_id(&self) -> i32 {
        self.table_id
    }
}
