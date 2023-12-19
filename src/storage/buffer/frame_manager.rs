use std::{num::NonZeroUsize, sync::Mutex};

use lru::LruCache;
use tracing::error;

use crate::common::types::PageNum;

use super::{
    frame::{Frame, FrameId},
    page::BP_PAGE_DATA_SIZE,
};

/// 一个页面中 bitmap 最大的 Byte 数量
const MAX_PAGE_NUM: usize = (BP_PAGE_DATA_SIZE - std::mem::size_of::<u32>() * 2) * 8;

/// Frame Manager frame 最大容纳量
const LRU_MAX_NUM: usize = 5000;

/// Buffer Pool 的第一个页面中存放一些元数据
#[derive(Debug)]
pub struct BPFileHeader {
    pub page_count: u32,
    pub allocated_pages: u32,

    /// 页面分配位图, 第0个页面(就是当前页面)，总是1
    pub bitmap: [u8; MAX_PAGE_NUM / 8],
}

impl BPFileHeader {
    pub fn to_string(&self) -> String {
        format!(
            "page count: {}, allocated count: {}",
            self.page_count, self.allocated_pages
        )
    }
}

impl From<[u8; BP_PAGE_DATA_SIZE]> for BPFileHeader {
    fn from(data: [u8; BP_PAGE_DATA_SIZE]) -> BPFileHeader {
        BPFileHeader {
            page_count: u32::from_le_bytes(data[..4].try_into().unwrap()),
            allocated_pages: u32::from_le_bytes(data[4..8].try_into().unwrap()),
            bitmap: data[8..BP_PAGE_DATA_SIZE].try_into().unwrap(),
        }
    }
}

/// 管理 Frame
/// 当内存中的页帧不够用时，需要从内存中淘汰一些页帧，以便为新的页帧腾出空间。
/// 这个管理器负责为所有的BufferPool提供页帧管理服务，也就是所有的BufferPool磁盘文件
/// 在访问时都使用这个管理器映射到内存。
pub struct FrameManager {
    frames: LruCache<FrameId, Frame>,
    // allocator: Slab<Frame>,
    lock: Mutex<i32>,
}

impl FrameManager {
    pub fn new() -> FrameManager {
        FrameManager {
            frames: LruCache::new(NonZeroUsize::new(LRU_MAX_NUM).unwrap()),
            // allocator: Slab::with_capacity(LRU_MAX_NUM),
            lock: Mutex::new(0),
        }
    }

    /// 获取指定的页面
    pub fn get(&mut self, file_desc: i32, page_num: PageNum) -> Option<&mut Frame> {
        let frame_id = FrameId::new(file_desc, page_num);
        self.get_internal(&frame_id)
    }

    /// 列出所有指定文件的页面
    pub fn find_list(&self, file_desc: i32) -> Vec<&Frame> {
        let _locked = self.lock.lock();
        let mut frames = vec![];

        for (frame_id, frame) in self.frames.iter() {
            if frame_id.file_desc == file_desc {
                frames.push(frame)
            }
        }

        frames
    }

    /// 分配一个新的页面
    pub fn alloc(&mut self, file_desc: i32, page_num: PageNum) -> &mut Frame {
        if self.is_full() {
            assert_eq!(self.purge(), 1, "all frame is pined");
        }
        let frame_id = FrameId::new(file_desc, page_num);
        self.frames.get_or_insert_mut(frame_id, || {
            let mut frame = Frame::new();
            frame.set_file_desc(file_desc);
            frame.set_page_num(page_num);
            frame.pin();
            frame
        })
    }

    pub fn free(&mut self, file_desc: i32, page_num: PageNum, frame: &mut Frame) -> crate::Result<()> {
        let frame_id = FrameId::new(file_desc, page_num);
        self.free_internal(&frame_id, frame)
    }

    /// 从 lru 中淘汰一个 pin count 为 0 的 frame
    pub fn purge(&mut self) -> usize {
        if self.frames.len() < LRU_MAX_NUM {
            0
        } else {
            let mut try_count = 0;
            while let Some((frame_id, mut frame)) = self.frames.pop_lru() {
                if try_count == self.frame_num() {
                    return 0;
                }
                try_count = try_count + 1;
                match frame.pin_count() {
                    0 => {
                        // 如果 pin_count 为 0，退出循环并返回 1
                        if let Err(e) = self.free_internal(&frame_id, &mut frame) {
                            error!("can not free framem, {}", e);
                        }
                        return 1;
                    }
                    _ => {
                        // 如果 pin_count 不为 0，继续迭代
                        self.frames.put(frame_id, frame);
                        continue;
                    }
                }
            }
            0
        }
    }

    pub fn frame_num(&self) -> usize {
        self.frames.len()
    }

    fn get_internal(&mut self, frame_id: &FrameId) -> Option<&mut Frame> {
        let _locked = self.lock.lock();
        let frame = self.frames.get_mut(frame_id);
        if let Some(frame) = frame {
            frame.pin();
            Some(frame)
        } else {
            None
        }
    }

    fn free_internal(&mut self, frame_id: &FrameId, frame: &mut Frame) -> crate::Result<()> {
        let frame_source = self.get_internal(frame_id);
        if let Some(frame_source) = frame_source {
            assert!(frame_source.pin_count() == 1 && frame.pin_count() == 1, "failed to free frame")
        } else {
            unreachable!("")
        }
        let _ = frame.unpin();
        self.frames.pop(frame_id);
        Ok(())
    }

    fn is_full(&self) -> bool {
        self.frames.len() == LRU_MAX_NUM
    }
}

#[cfg(test)]
mod tests {
    use super::FrameManager;


    #[test]
    fn test_lru() {
        let mut manger = FrameManager::new();
        let _ = manger.alloc(1, 1);
        let files = manger.find_list(1);
        assert_eq!(files.len(), 1);

        let _ = manger.alloc(1, 2);
        let _ = manger.alloc(1, 3);
        let files = manger.find_list(1);
        assert_eq!(files.len(), 3);
    }
}