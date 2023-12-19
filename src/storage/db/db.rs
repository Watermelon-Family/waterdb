use std::collections::HashMap;


#[derive(Debug)]
pub struct Db {
    name: String,
    db_path: String,
    opened_tables: HashMap<String, Table>,
    // unique_ptr<CLogManager> clog_manager ?
    next_table_id: i32,
}

impl Db {
  fn new() -> Db {
    Db {
      name: String::new(),
      db_path: String::new(),
      opened_tables: HashMap::new(),
      new_table_id: 0,
    }
  }

  pub fn init(&mut self, name: &str, dp_path: &str) {

  }

  pub fn create_table(&mut self, table_name: &str, attribute_count: i32, attributes: &[Attribute]) {

  }

  pub fn find_table(&self, table_name: &str) -> Option<Table> {
  
  }

  pub fn find_table(&self, table_id: i32) -> Option<Table> {

  }

  fn open_all_tables() {

  }

  pub fn sync() {

  }

  pub recover() {

  }

 }
