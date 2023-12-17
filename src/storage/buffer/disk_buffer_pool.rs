use std::{collections::HashSet, sync::Mutex, fs::File, io::Write, os::fd::AsRawFd};

use tracing::info;

use crate::common::types::PageNum;

use super::{frame_manager::BPFileHeader, page::Page, frame::{Frame, FrameId}};

#[derive(Debug)]
pub struct DiskBufferPool {
    file_name: String,
    file_desc: i32,
    file_header: BPFileHeader,
    header_frame: Frame,
    disposed_pages: HashSet<PageNum>,

    lock: Mutex<i32>,
}

impl DiskBufferPool {
    // need add to Buffer Pool
    // pub fn create_file(&mut self, file_name: String) -> crate::Result<()> {
    //     info!("creating file: {}", file_name);
    //     let mut file = File::create(file_name.clone())?;

    //     let page = Page::new();

    //     let mut file_header = BPFileHeader::from(page.data);
    //     file_header.allocated_pages = 1;
    //     file_header.page_count = 1;
    //     file_header.bitmap[0] = 0x01;

    //     let page = Page::from(file_header);
    //     file.write_all(&page.as_bytes())?;
        
    //     info!("successfully create file: {}", file_name);
    //     Ok(())
    // }

    pub fn new(file_name: String) -> crate::Result<DiskBufferPool> {
        let file = File::open(file_name.clone())?;
        let fd = file.as_raw_fd();

        let hdr_frame = Frame::new();
        hdr_frame.set_file_desc(fd);
        hdr_frame.set_page_num(0);

        // self.file_desc = file.metadata()
        let disk_buffer_pool = DiskBufferPool {
            file_name,
            file_desc: fd,

            file_header: BPFileHeader::from(Page::new().data),
            disposed_pages: HashSet::new(),

            lock: Mutex::new(0),
        };
        Ok(disk_buffer_pool)
    }

    pub fn close(&mut self) -> crate::Result<()> {

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::DiskBufferPool;


    #[test]
    fn test_open_file() {
        let file_name = "tests/resource/a.txt".to_string();
        let disk_buffer_pool = DiskBufferPool::new(file_name);

        assert!(disk_buffer_pool.is_ok());
    }
}