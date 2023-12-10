use std::{sync::Mutex, mem};

use tracing::debug;

use crate::{common::types::PageNum, Result};

use super::page::{Page, BP_PAGE_DATA_SIZE};

/// 页帧
/// 页帧 frame 是 页面 Page 在存在中的表示。磁盘文件 Page 首先通过 Frame 映射到内存中
/// 
/// 在 frame 中可以使用 dirty 标记数据是否修改过，并且在 frame 淘汰的时候 sync 到磁盘中
/// 
/// 为了防止 frame 在使用的时候被淘汰，可以使用 pin count 表示当前页面被谁使用了
pub struct Frame {
    dirty: bool,
    pin_count: Mutex<u32>,
    // acc_time: u64,
    file_desc: i32,
    page: Page,
}

#[derive(Debug, PartialEq, Eq)]
pub struct FrameId {
    pub file_desc: i32,
    pub page_num: PageNum,
}

impl FrameId {
    pub fn new(file_desc: i32, page_num: PageNum) -> FrameId {
        FrameId {
            file_desc,
            page_num,
        }
    }

    pub fn hash(&self) -> u64 {
        ((self.file_desc as u64) << 32) | self.page_num as u64
    }

    pub fn to_string(&self) -> String {
        format!("fd: {}, page_num: {}", self.file_desc, self.page_num)
    }
}

/// Frame 对象数据的访问控制
impl Frame {
    pub fn file_desc(&self) -> i32 {
        self.file_desc
    }

    pub fn set_file_desc(&mut self, fd: i32) -> () {
        self.file_desc = fd
    }

    pub fn page(&self) -> &Page {
        &self.page
    }

    pub fn page_num(&self) -> PageNum {
        self.page.page_num
    }

    pub fn set_page_num(&mut self, page_num: PageNum) -> () {
        self.page.page_num = page_num
    }

    pub fn get_frame_id(&self) -> FrameId {
        FrameId { file_desc: self.file_desc, page_num: self.page.page_num }
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

    pub fn data(&self) -> &[char; BP_PAGE_DATA_SIZE] {
        &self.page.data
    }

    pub fn pin_count(&self) -> u32 {
        *self.pin_count.lock().unwrap()
    }
}

impl Frame {
    pub fn reinit() -> () {}
    pub fn reset() -> () {}

    pub fn clear_page(&mut self) -> () {
        self.page = unsafe { mem::zeroed() }
    }

    pub fn can_purge(&self) -> bool {
        let pin_count = self.pin_count.lock().unwrap();
        *pin_count == 0
    }

    pub fn pin(&mut self) -> () {
        let mut pin_count = self.pin_count.lock().unwrap();
        *pin_count += 1;
        debug!("after frame pin. pin = {}, file_desc = {}, page num = {}", *pin_count, self.file_desc, self.page.page_num)
    }

    pub fn unpin(&mut self) -> Result<u32> {
        let mut pin_count = self.pin_count.lock().unwrap();

        if *pin_count == 0 {
            return Err("can not unpin this frame, pin count = 0".into())
        }

        *pin_count -= 1;

        Ok(*pin_count)
    }

    pub fn to_string(&self) -> String {
        format!("frame id: {}, dirty: {}, pin count: {}, fd: {}, page num: {}",
            self.get_frame_id().to_string(), self.dirty(), self.pin_count(), self.file_desc(), self.page_num())
    }
}