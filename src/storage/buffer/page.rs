use crate::common::types::PageNum;

const BP_INVALID_PAGE_NUM: i32 = -1;

const BP_HEADER_PAGE: PageNum = 0;

const BP_PAGE_SIZE: usize = 1 << 13;

pub const BP_PAGE_DATA_SIZE: usize = BP_PAGE_SIZE - std::mem::size_of::<PageNum>();


/// 表示一个页面，可能存在内存中也可能存在磁盘上面
pub struct Page {
    pub page_num: PageNum,
    pub data: [char; BP_PAGE_DATA_SIZE],
}