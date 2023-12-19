#[derive(Debug)]
pub struct Record {}

#[derive(Debug)]
pub struct RecordPageIterator {}

/*
  @brief 负责处理单个页面的各种操作
  @note 定长记录如下：
        |   PageHeader | bitmap             |
        | record1 | record2 | ... | recordN |
*/
#[derive(Debug)]
pub struct RecordPageHandler {}

// 管理整个文件中记录的增删改查
#[derive(Debug)]
pub struct RecordFileHandler {}

impl RecordFileHandler {
    pub fn new() -> RecordFileHandler {
        RecordFileHandler {}
    }
}

// 遍历某个文件中的所有记录
#[derive(Debug)]
pub struct RecordFileScanner {}
