use crate::common::types::PageNum;

use super::frame_manager::BPFileHeader;

const BP_INVALID_PAGE_NUM: i32 = -1;

const BP_HEADER_PAGE: PageNum = 0;

const BP_PAGE_SIZE: usize = 1 << 13;

pub const BP_PAGE_DATA_SIZE: usize = BP_PAGE_SIZE - std::mem::size_of::<PageNum>();


/// 表示一个页面，可能存在内存中也可能存在磁盘上面
#[derive(Debug, PartialEq, Eq)]
pub struct Page {
    pub page_num: PageNum,
    pub data: [u8; BP_PAGE_DATA_SIZE],
}

impl Page {
    pub fn new() -> Page {
        Page {
            page_num: 0,
            data: [0 as u8; BP_PAGE_DATA_SIZE]
        }
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        let mut bytes = [0 as u8; BP_PAGE_DATA_SIZE + 4];

        bytes[..4].copy_from_slice(&self.page_num.to_le_bytes());
        bytes[4..BP_PAGE_DATA_SIZE].copy_from_slice(&self.data);

        bytes.to_vec()
    }
}

impl From<BPFileHeader> for Page {
    fn from(value: BPFileHeader) -> Self {
        let a = value.page_count.to_le_bytes();
        let b = value.allocated_pages.to_le_bytes();
        let c = value.bitmap;
        let mut d = [0 as u8; BP_PAGE_DATA_SIZE];
        d[..4].copy_from_slice(a.as_ref());
        d[4..8].copy_from_slice(b.as_ref());
        d[8..BP_PAGE_DATA_SIZE].copy_from_slice(c.as_ref());
        Page {
            page_num: 0,
            data: d,
        }
    }
}