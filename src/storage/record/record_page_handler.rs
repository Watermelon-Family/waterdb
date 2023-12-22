use std::{collections::HashSet, rc::Rc, cell::RefCell};

use crate::{storage::buffer::frame_manager::FrameManager, common::types::PageNum};

/// 用于处理一个页面中各种操作，比如插入记录、删除记录或者查找记录
/// | page_id 32bits | capacity 8bits | bitmap 16bits |
/// | record | record | record | record |
struct RecordPageHandler {
    disk_buffer_pool: Rc<RefCell<FrameManager>>,
    free_pages: HashSet<PageNum>,
    table_id: i32,
}

impl RecordPageHandler {
    pub fn new(buffer_pool: Rc<RefCell<FrameManager>>, tid: i32) -> crate::Result<RecordPageHandler> {
        let mut record_page_handler = RecordPageHandler {
            disk_buffer_pool: buffer_pool,
            free_pages: HashSet::new(),
            table_id: tid
        };

        let page_nums = record_page_handler
                                    .disk_buffer_pool
                                    .borrow_mut()
                                    .find_free_pages_by_table_id(record_page_handler.table_id)?;
        for page_num in page_nums {
            record_page_handler.free_pages.insert(page_num);
        }

        Ok(record_page_handler)
    }
    
    pub fn insert_record() {}

    pub fn free_pages(&self) -> &HashSet<PageNum> {
        &self.free_pages
    }
}

#[cfg(test)]
mod tests {
    use std::{rc::Rc, cell::RefCell};

    use crate::storage::buffer::frame_manager::FrameManager;

    use super::RecordPageHandler;


    #[test]
    fn test_new() -> crate::Result<()> {
        let (table_id, page_num) = (2, 0);
        let frame_buffer = Rc::new(RefCell::new(FrameManager::new()));
        let _ = frame_buffer.borrow_mut().new_frame(table_id, page_num);

        let rph = RecordPageHandler::new(frame_buffer.clone(), 2)?;
        assert_eq!(rph.free_pages().len(), 1);

        Ok(())
    }
}