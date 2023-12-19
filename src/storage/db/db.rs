use std::collections::HashMap;

use crate::sql::parser::value::AttrInfoSqlNode;
use crate::storage::table::table::Table;

#[derive(Debug)]
pub struct Db {
    name: String,
    db_path: String,
    tables: HashMap<String, Table>,
    // unique_ptr<CLogManager> clog_manager ?
    next_table_id: i32,
}

impl Db {
    fn new() -> Db {
        Db {
            name: String::new(),
            db_path: String::new(),
            tables: HashMap::new(),
            next_table_id: 0,
        }
    }

    pub fn init(&mut self, name: &str, dp_path: &str) -> Result<(), ()> {
        Ok(())
    }

    pub fn create_table(
        &mut self,
        table_name: &str,
        attribute_count: i32,
        attributes: &[AttrInfoSqlNode],
    ) -> Result<(), ()> {
        Ok(())
    }

    pub fn find_table_by_name(&self, table_name: &str) -> Option<&Table> {
        self.tables.get(table_name)
    }

    pub fn find_table_by_id(&self, table_id: i32) {}

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn open_all_tables(&self) {
        // self.tables.keys().cloned().collect()
    }

    pub fn sync(&self) -> Result<(), ()> {
        Ok(())
    }

    pub fn recover() -> Result<(), ()> {
        Ok(())
    }
}
