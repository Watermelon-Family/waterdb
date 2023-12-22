use std::fs::File;

use std::io::Read;

use crate::common::types::PageNum;

const BP_PAGE_SIZE: usize = 1 << 13;

const BP_CAPACITY: u8 = 16;

const BP_BITMAP_SIZE: usize = 16;

pub const BP_PAGE_DATA_SIZE: usize = BP_PAGE_SIZE - 32 - 8 - 16;

/// 表示一个页面，可能存在内存中也可能存在磁盘上面，为了简化 Page 上面最多可以存放 16 个 record
/// | page_id 32bits | capacity 8bits | bitmap 16bits |
/// | record | record | record | record |
#[derive(Debug, PartialEq, Eq)]
pub struct Page {
    pub page_id: PageNum,
    pub page_capacity: u8,
    pub bitmap: [u8; BP_BITMAP_SIZE / 8],
    pub data: [u8; BP_PAGE_DATA_SIZE / 8],
}

impl Page {
    pub fn new() -> Page {
        Page {
            page_id: 0,
            page_capacity: BP_CAPACITY,
            bitmap: [0 as u8; BP_BITMAP_SIZE / 8],
            data: [0 as u8; BP_PAGE_DATA_SIZE / 8],
        }
    }

    pub fn load_from_file(file_path: String) -> crate::Result<Page> {
        let mut file = File::open(file_path)?;
        let mut buffer = vec![];
        file.read_to_end(&mut buffer)?;

        let page_id_bytes = &buffer[..4];
        let page_capacity_bytes = &buffer[4..5];
        let bitmap_bytes = buffer[5..7].try_into().unwrap();
        let data_bytes = buffer[7..BP_PAGE_SIZE / 8].try_into().unwrap();

        Ok(Page {
            page_id: u32::from_le_bytes(page_id_bytes.try_into().unwrap()),
            page_capacity: u8::from_le_bytes(page_capacity_bytes.try_into().unwrap()),
            bitmap: bitmap_bytes,
            data: data_bytes,
        })
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        let mut bytes = [0 as u8; BP_PAGE_SIZE / 8];
        bytes[..4].copy_from_slice(&self.page_id.to_le_bytes());
        bytes[4..5].copy_from_slice(&self.page_capacity.to_le_bytes());
        bytes[5..7].copy_from_slice(&self.bitmap);
        bytes[7..BP_PAGE_SIZE / 8].copy_from_slice(&self.data);
        bytes.to_vec()
    }

    pub fn is_free(&self) -> bool {
        for bit in self.bitmap {
            if bit != u8::MAX {
                return true;
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use std::{
        fs::{self, File},
        io::Write,
    };

    use super::Page;

    #[test]
    fn test_page() -> crate::Result<()> {
        let mut page = Page::new();
        page.page_id = 1;

        let file_name = "tests/resource/test_page";

        let mut file = File::create(file_name)?;
        file.write_all(&page.as_bytes())?;
        drop(file);

        let page = Page::load_from_file(file_name.to_string())?;

        assert_eq!(page.page_id, 1);

        fs::remove_file(file_name)?;

        Ok(())
    }
}
