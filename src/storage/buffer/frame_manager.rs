use std::{cell::RefCell, num::NonZeroUsize, rc::Rc, sync::Mutex};

use lru::LruCache;
use tracing::{error, warn};

use crate::common::types::PageNum;

use super::frame::{Frame, FrameId};

/// Frame Manager frame 最大容纳量
const LRU_MAX_NUM: usize = 5000;

/// 管理 Frame
/// 当内存中的页帧不够用时，需要从内存中淘汰一些页帧，以便为新的页帧腾出空间。
/// 这个管理器负责为所有的BufferPool提供页帧管理服务，也就是所有的BufferPool磁盘文件
/// 在访问时都使用这个管理器映射到内存。
pub struct FrameManager {
    frames: Mutex<LruCache<FrameId, Rc<RefCell<Frame>>>>,
}

impl FrameManager {
    pub fn new() -> FrameManager {
        FrameManager {
            frames: Mutex::new(LruCache::new(NonZeroUsize::new(LRU_MAX_NUM).unwrap())),
        }
    }

    pub fn get_frame(
        &mut self,
        table_id: i32,
        page_num: PageNum,
    ) -> crate::Result<Rc<RefCell<Frame>>> {
        let frame_id = FrameId::new(table_id, page_num);
        let mut frames = self.frames.lock().unwrap();
        let frame = frames.get(&frame_id);
        match frame {
            Some(frame) => Ok(Rc::clone(frame)),
            None => {
                drop(frames);
                self.load_from_file(frame_id)
            }
        }
    }

    pub fn find_all_frames(&mut self, table_id: i32) -> crate::Result<Vec<Rc<RefCell<Frame>>>> {
        let mut frames = vec![];
        let mut page_num = 0;
        loop {
            let frame = self.get_frame(table_id, page_num);
            if frame.is_err() {
                break;
            }
            let frame = frame.unwrap();
            frames.push(frame);
            page_num = page_num + 1;
        }

        Ok(frames)
    }

    pub fn find_free_pages_by_table_id(&mut self, table_id: i32) -> crate::Result<Vec<PageNum>> {
        let mut frame_ids = vec![];

        let mut page_num = 0;
        loop {
            let frame = self.get_frame(table_id, page_num);
            if frame.is_err() {
                break;
            }
            let frame = frame.unwrap();
            if frame.borrow().is_free() {
                let frame_id = frame.borrow().get_frame_id();
                frame_ids.push(frame_id.page_num);
            }
            page_num = page_num + 1;
        }

        Ok(frame_ids)
    }

    pub fn new_frame(
        &mut self,
        table_id: i32,
        page_num: PageNum,
    ) -> crate::Result<Rc<RefCell<Frame>>> {
        let frame_id = FrameId::new(table_id, page_num);
        let mut frame = Frame::new();
        frame.set_table_id(table_id);
        frame.set_page_id(page_num);

        let mut frames = self.frames.lock().unwrap();
        let _ = frames.put(frame_id, Rc::new(RefCell::new(frame)));
        drop(frames);

        self.get_frame(table_id, page_num)
    }

    pub fn flush_all(&self) -> crate::Result<()> {
        let frames = self.frames.lock().unwrap();
        for (_, frame) in frames.iter() {
            frame.borrow().flush()?;
        }
        Ok(())
    }

    fn load_from_file(&mut self, frame_id: FrameId) -> crate::Result<Rc<RefCell<Frame>>> {
        self.purge()?;

        let frame = Frame::load_from_file(frame_id.table_id, frame_id.page_num)?;
        let mut frames = self.frames.lock().unwrap();

        let _ = frames.put(frame_id.clone(), Rc::new(RefCell::new(frame)));

        drop(frames);
        self.get_frame(frame_id.table_id, frame_id.page_num)
    }

    fn purge(&mut self) -> crate::Result<()> {
        if self.is_full() {
            let mut try_count = 0;
            let mut frames = self.frames.lock().unwrap();
            while let Some((frame_id, frame)) = frames.pop_lru() {
                if try_count == self.frame_num() {
                    return Err("can not purge frame".into());
                }
                try_count = try_count + 1;
                match frame.clone().borrow().pin_count() {
                    0 => {
                        drop(frames);
                        if let Err(e) = self.free_internal(&frame_id) {
                            error!("can not free frame: {:?}, {}", frame_id, e);
                        }
                        return Ok(());
                    }
                    _ => {
                        frames.put(frame_id, frame);
                        continue;
                    }
                }
            }
        }
        Ok(())
    }

    fn frame_num(&self) -> usize {
        let frames = self.frames.lock().unwrap();
        frames.len()
    }

    fn is_full(&self) -> bool {
        self.frame_num() == LRU_MAX_NUM
    }

    fn free_internal(&mut self, frame_id: &FrameId) -> crate::Result<()> {
        let frame_source = self.get_frame(frame_id.table_id, frame_id.page_num)?;
        assert!(
            frame_source.borrow().pin_count() == 0,
            "failed to free frame"
        );
        self.flush(frame_id)?;
        let mut frames = self.frames.lock().unwrap();
        frames.pop(frame_id);
        Ok(())
    }

    fn flush(&self, frame_id: &FrameId) -> crate::Result<()> {
        let mut frames = self.frames.lock().unwrap();
        let frame = frames.get(frame_id);

        match frame {
            Some(frame) => frame.borrow().flush()?,
            None => warn!("flush non existing frame to disk, {:?}", frame_id),
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::fs;

    use crate::storage::buffer::utils::SYS_DIR;

    use super::FrameManager;

    #[test]
    fn test_lru() -> crate::Result<()> {
        let mut manager = FrameManager::new();
        let frames = manager.find_all_frames(0)?;
        assert_eq!(frames.len(), 0);
        let _ = manager.new_frame(0, 0);
        let frames = manager.find_all_frames(0)?;
        assert_eq!(frames.len(), 1);
        let _ = manager.flush_all()?;

        for entry in fs::read_dir(format!("{}/{}", SYS_DIR, 0))? {
            let entry = entry?;
            let path = entry.path();
            if path.is_file() {
                fs::remove_file(path)?;
            }
        }

        Ok(())
    }
}
