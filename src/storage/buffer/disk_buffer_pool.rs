use std::{num::NonZeroUsize, sync::Mutex};

use lru::LruCache;
use tracing::error;
// use slab::Slab;

use crate::common::types::PageNum;

use super::{
    frame::{Frame, FrameId, self},
    page::BP_PAGE_DATA_SIZE,
};

const MAX_PAGE_NUM: usize = (BP_PAGE_DATA_SIZE - std::mem::size_of::<u32>() * 2) * 8;

const LRU_MAX_NUM: usize = 5000;

/// Buffer Pool 的第一个页面中存放一些元数据
pub struct BPFilerHeader {
    pub page_count: u32,
    pub allocated_pages: u32,

    /// 页面分配位图, 第0个页面(就是当前页面)，总是1
    pub bitmap: [char; 1],
}

impl BPFilerHeader {
    pub fn to_string(&self) -> String {
        format!(
            "page count: {}, allocated count: {}",
            self.page_count, self.allocated_pages
        )
    }
}

/// 管理 Frame
/// 当内存中的页帧不够用时，需要从内存中淘汰一些页帧，以便为新的页帧腾出空间。
/// 这个管理器负责为所有的BufferPool提供页帧管理服务，也就是所有的BufferPool磁盘文件
/// 在访问时都使用这个管理器映射到内存。
pub struct BPFrameManager {
    frames: LruCache<FrameId, Frame>,
    // allocator: Slab<Frame>,
    lock: Mutex<i32>,
}

impl BPFrameManager {
    pub fn new() -> BPFrameManager {
        BPFrameManager {
            frames: LruCache::new(NonZeroUsize::new(LRU_MAX_NUM).unwrap()),
            // allocator: Slab::with_capacity(LRU_MAX_NUM),
            lock: Mutex::new(0),
        }
    }

    /// 获取指定的页面
    /// locked
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
        let frame_id = FrameId::new(file_desc, page_num);
        self.frames.get_or_insert_mut(frame_id, || {
            let mut frame = Frame::new();
            frame.set_file_desc(file_desc);
            frame.set_page_num(page_num);
            frame.pin();
            frame
        })
    }

    /// 从 lru 中淘汰一个 pin count 为 0 的 frame
    pub fn purge(&mut self) -> usize {
        if self.frames.len() < LRU_MAX_NUM {
            0
        } else {
            while let Some((frame_id, mut frame)) = self.frames.pop_lru() {
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
}

// struct DiskBufferPool {
//     frame_manager: BPFrameManager,

//     lock: Mutex<i32>,
// }
