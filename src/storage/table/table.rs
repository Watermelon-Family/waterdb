use super::table_meta::TableMeta;
use crate::sql::parser::value::AttrInfoSqlNode;
use crate::storage::buffer::disk_buffer_pool::DiskBufferPool;
use crate::storage::record::record::RecordFileHandler;

#[derive(Debug)]
pub struct Table {
    base_dir: String,
    table_meta: TableMeta,
    // data_buffer_pool: DiskBufferPool,
    record_handler: RecordFileHandler,
    indexes: Vec<String>,
}

impl Table {
    fn new() -> crate::Result<Table> {
        let table = Table {
            base_dir: String::new(),
            table_meta: TableMeta::new(),
            // data_buffer_pool: DiskBufferPool::new("".to_string())?,
            record_handler: RecordFileHandler::new(),
            indexes: Vec::new(),
        };
        Ok(table)
    }

    /*
     @func 创建表

     @param path 元数据保存文件完整路径
     @param name 表名
     @base_dir 表数据存放路径
     @attribute_count 字段个数
     @attributes 字段信息
    */
    fn create(
        &self,
        table_id: i32,
        path: &str,
        name: &str,
        base_dir: &str,
        attribute_count: i32,
        attributes: &[AttrInfoSqlNode],
    ) {
        if table_id < 0 {
            ()
        }
    }

    fn open(&self, meta_file: &str, base_dir: &str) -> Result<(), ()> {
        Ok(())
    }

    fn make_record() {}

    fn table_id(&self) -> i32 {
        self.table_meta.table_id()
    }

    fn name(&self) -> &str {
        ""
    }

    fn table_meta() {}

    fn sync(&self) -> Result<(), ()> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::Table;

    #[test]
    fn test_table() {
        let table = Table::new();

        assert!(table.is_ok());
    }
}
