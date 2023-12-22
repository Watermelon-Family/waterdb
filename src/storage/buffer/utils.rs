use crate::common::types::PageNum;

pub const SYS_DIR: &str = "tests/db";

pub fn get_page_path(table_id: i32, page_id: PageNum) -> String {
    format!("{}/{}/page-{}", SYS_DIR, table_id, page_id)
}