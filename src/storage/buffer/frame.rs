use std::{
    fs::File,
    hash::{Hash, Hasher},
    io::Write,
    mem,
    sync::Mutex,
};

use tracing::debug;

use crate::{common::types::PageNum, Result};

use super::{
    page::{Page, BP_PAGE_DATA_SIZE},
    utils::get_page_path,
};

/// 页帧
/// 页帧 frame 是 页面 Page 在存在中的表示。磁盘文件 Page 首先通过 Frame 映射到内存中
///
/// 在 frame 中可以使用 dirty 标记数据是否修改过，并且在 frame 淘汰的时候 sync 到磁盘中
///
/// 为了防止 frame 在使用的时候被淘汰，可以使用 pin count 表示当前页面被谁使用了
#[derive(Debug)]
pub struct Frame {
    dirty: bool,
    pin_count: Mutex<u32>,
    table_id: i32,
    page: Page,
}

impl PartialEq for Frame {
    fn eq(&self, other: &Self) -> bool {
        self.dirty == other.dirty && self.table_id == other.table_id && self.page == other.page
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct FrameId {
    pub table_id: i32,
    pub page_num: PageNum,
}

impl FrameId {
    pub fn new(table_id: i32, page_num: PageNum) -> FrameId {
        FrameId { table_id, page_num }
    }

    pub fn to_string(&self) -> String {
        format!("table_id: {}, page_num: {}", self.table_id, self.page_num)
    }
}

impl Hash for FrameId {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.table_id.hash(state);
        self.page_num.hash(state);
    }
}

/// Frame 对象数据的访问控制
impl Frame {
    pub fn table_id(&self) -> i32 {
        self.table_id
    }

    pub fn set_table_id(&mut self, table_id: i32) -> () {
        self.table_id = table_id
    }

    pub fn page(&self) -> &Page {
        &self.page
    }

    pub fn page_id(&self) -> PageNum {
        self.page.page_id
    }

    pub fn set_page_id(&mut self, page_id: PageNum) -> () {
        self.page.page_id = page_id
    }

    pub fn get_frame_id(&self) -> FrameId {
        FrameId {
            table_id: self.table_id,
            page_num: self.page.page_id,
        }
    }

    pub fn mark_dirty(&mut self) -> () {
        self.dirty = true
    }

    pub fn clear_dirty(&mut self) -> () {
        self.dirty = false
    }

    pub fn dirty(&self) -> bool {
        self.dirty
    }

    pub fn data(&self) -> &[u8; BP_PAGE_DATA_SIZE / 8] {
        &self.page.data
    }

    pub fn pin_count(&self) -> u32 {
        *self.pin_count.lock().unwrap()
    }
}

impl Frame {
    pub fn reinit() -> () {}
    pub fn reset() -> () {}

    pub fn new() -> Frame {
        Frame {
            dirty: false,
            pin_count: Mutex::new(0),
            table_id: -1,
            page: Page::new(),
        }
    }

    pub fn load_from_file(table_id: i32, page_id: PageNum) -> crate::Result<Frame> {
        let page = Page::load_from_file(get_page_path(table_id, page_id))?;
        Ok(Frame {
            dirty: false,
            pin_count: Mutex::new(0),
            table_id,
            page,
        })
    }

    pub fn is_free(&self) -> bool {
        self.page.is_free()
    }

    pub fn clear_page(&mut self) -> () {
        self.page = unsafe { mem::zeroed() }
    }

    pub fn can_purge(&self) -> bool {
        let pin_count = self.pin_count.lock().unwrap();
        *pin_count == 0
    }

    pub fn pin_frame(&mut self) -> () {
        let mut pin_count = self.pin_count.lock().unwrap();
        *pin_count += 1;
        debug!(
            "after frame pin. pin = {}, file_desc = {}, page num = {}",
            *pin_count, self.table_id, self.page.page_id
        )
    }

    pub fn unpin_frame(&mut self) -> Result<u32> {
        let mut pin_count = self.pin_count.lock().unwrap();

        if *pin_count == 0 {
            return Err("can not unpin this frame, pin count = 0".into());
        }

        *pin_count -= 1;

        Ok(*pin_count)
    }

    pub fn to_string(&self) -> String {
        format!(
            "frame id: {}, dirty: {}, pin count: {}, table id: {}, page id: {}",
            self.get_frame_id().to_string(),
            self.dirty(),
            self.pin_count(),
            self.table_id(),
            self.page_id()
        )
    }

    pub fn flush(&self) -> crate::Result<()> {
        let path = get_page_path(self.table_id(), self.page_id());
        let mut file = File::create(path)?;
        let buf = self.page.as_bytes();
        file.write_all(&buf)?;
        file.flush()?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::{cell::RefCell, fs, rc::Rc};

    use crate::storage::buffer::utils::get_page_path;

    use super::Frame;

    #[test]
    fn test_rc_refcell() {
        let frame = Rc::new(RefCell::new(Frame::new()));
        frame.borrow_mut().pin_frame();
        assert_eq!(frame.borrow().pin_count(), 1);
    }

    #[test]
    fn test_flush() -> crate::Result<()> {
        let mut expected = Frame::new();
        expected.set_page_id(1);
        expected.set_table_id(1);
        expected.mark_dirty();

        let _ = expected.flush()?;
        expected.clear_dirty();

        let frame = Frame::load_from_file(1, 1)?;

        assert_eq!(frame, expected);

        let _ = fs::remove_file(get_page_path(1, 1));
        Ok(())
    }
}
