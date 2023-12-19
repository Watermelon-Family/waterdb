use std::{num::NonZeroUsize, sync::Mutex};
use std::collections::{HashMap, HasSet, BitSet};
use std::fs::{File, OpenOptions};
use std::io::{self, Write, Read};

use lru::LruCache;
use tracing::error;
// use slab::Slab;

use crate::common::types::PageNum;
use crate::common::consts::StatusCode;

use super::{
    frame::{Frame, FrameId, self},
    page::BP_PAGE_DATA_SIZE,
};

const MAX_PAGE_NUM: usize = (BP_PAGE_DATA_SIZE - std::mem::size_of::<u32>() * 2) * 8;

const LRU_MAX_NUM: usize = 5000;

/// Buffer Pool 的第一个页面中存放一些元数据
pub struct BPFilerHeader {
    // 文件总页面数
    pub page_count: u32,
    // 已分配页面数
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
    allocator: Slab::new(),
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

    pub fn init(&self, pool_num: i32) -> StatusCode {

    }

    pub fn cleanup() {

    }

    /*
      @brief 获取指定的页面

      @param file_desc 文件描述符
      @param page_num 页面号
      locked
    */
    pub fn get(&mut self, file_desc: i32, page_num: PageNum) -> Option<&mut Frame> {
        let frame_id = FrameId::new(file_desc, page_num);
        self.get_internal(&frame_id)
    }

    /*
      @brief 列出所有指定文件的页面

      @param file_desc 文件描述符
    */
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

    pub fn free(&mut self, file_desc: i32, page_num: PageNum, frame:  &mut Frame) -> StatusCode {

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

    pub fn total_frame_num(&self) -> usize {

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

/*
  @brief BufferPool Manager
*/
struct BufferPoolManager {
    frame_manager: BPFrameManager,

    lock: Mutex<i32>,
    // <name, DiskBufferPool>
    buffer_pools: HashMap,
    // <id, DiskBufferPool>
    fd_buffer_pools: HashMap,
}

impl BufferPoolManager {
  pb fn new(memory_size: i32) -> BufferPoolManager {

    if memory_size <= 0 {
      // TODO(lyy): 定义几个常量
      let MEM_POOL_ITEM_NUM = 20
      let DEFAULT_ITEM_NUM_PER_POOL = 20
      let BP_PAGE_SIZE = 20
      memory_size = MEM_POOL_ITEM_NUM * DEFAULT_ITEM_NUM_PER_POOL * BP_PAGE_SIZE;
    }
    // max(memory_size/bp_page_size/default_item_num_per_pool, 1)
    let pool_num = 1;
    let frame_manager = BPFrameManager::new()
    frame_manager.init(pool_num)

    BufferPoolManager {
      frame_manager: frame_manager,

      lock: Mutex::new(0),
      // <name, DiskBufferPool>
      buffer_pools: HashMap::new(),
      // <id, DiskBufferPool>
      fd_buffer_pools: HashMap::new(),
    }
  }

  pb fn create_file(file_name: &str) -> Result<(), std::io::Error> {
    Ok(())
  }

  pb fn open_file(file_name: &str, bp: DiskBufferPool) -> Result<DiskBufferPool, std::io::Error> {
    
  }

  pb fn close_file(file_name: &str) -> Result<(), std::io::Error> {
    Ok(())
  }

  pb fn flush_file(frame: Frame) -> Result<(), std::io::Error> {
    Ok(())
  }
}

/*
  @brief DiskBufferPool

  @description Top级别的大哥
*/
struct DiskBufferPool {
  bp_manager: BufferPoolManager,
  frame_manager: BPFrameManager,

  file_name: String,
  file_desc: i32 = -1,
  hdr_frame: Option<*mut Frame>,
  file_header: Option<*mut BPFilerHeader>,

  disposed_pages: HasSet,

  lock: Mutex<i32>,
}

/*
  @brief 磁盘缓存池，操作文件
*/
impl DiskBufferPool {
  pb fn new(bp_manager: mut BufferPoolManager, frame_manager: mut BPFrameManager) -> DiskBufferPool {
    DiskBufferPool {
      bp_manager: BufferPoolManager,
      frame_manager: BPFrameManager,
    
      hdr_frame: Option<*mut Frame> = null,
      file_header: Option<*mut BPFilerHeader> = null,
    
      disposed_pages: HasSet::new(),
    
      lock: Mutex::new(0),
    }
  }

  pb create_file(file_name: &str) -> Result<(), std::io:Error> {

  }


  fn open_or_create_file(file_name: &str) -> Result<File, io::Error> {
    let mut options = OpenOptions::new();
    options.read(true).write(true).create(true);

    match options.open(file_name) {
        Ok(f) => {
            println!("success");
            Ok(f)
        }
        Err(e) => {
            eprintln!("Failed to open file {}, because {}", file_name, e);
            Err(e)
        }
    }
  }

  pb open_file(&mut self, file_name: &str) -> Result<(), std::io:Error> {
    let mut options = OpenOptions::new();
    options.read(true).write(true).create(true);

    let file = match options.open(&file_name) {
        Ok(f) => {
            println!("success");
            f
        }
        Err(e) => {
            eprintln!("Failed to open file {}, because {}", file_name, e);
            return Err(e);
        }
    };

    self.file_name = file_name;
    self.file_desc = file.as_raw_fd();
    println!("Successfully open buffer pool file {}, File descriptor: {}", file_name, file_desc);

    // 获得 hdr 页帧
    let mut hdr_frame = Frame { /* ... */ };
    if let Err(e) = allocate_frame(BP_HEADER_PAGE, &mut hdr_frame) {
        eprintln!("failed to allocate frame for header. file name {}", file_name);
        Err(e)
    }

    hdr_frame.set_file_desc(file_desc);
    hdr_frame.access();

    // 假设 load_page 已经定义
    if let Err(e) = load_page(BP_HEADER_PAGE, &mut hdr_frame) {
        eprintln!("Failed to load first page of {}, due to {}", file_name, e);
        // 假设 purge_frame 已经定义
        purge_frame(&mut hdr_frame);
        return Ok(RC::IOERR_ACCESS); // 或处理错误 e
    }

    let file_header = hdr_frame.data();

    println!("Successfully open {}. file_desc={}, hdr_frame={:p}, file header={}",
            file_name, file_desc, &hdr_frame, file_header.to_string());
    Ok(())
  }

  fn close_file() -> Result<(), std::io:Error> {
    
  }

  fn allocate_page(&self, frame: &mut Frame) -> Result<(), std::io::Error> {
    let _locked = self.lock.lock()?;
    
    // TODO(lyy): 具体分配

    // 1. file header
    if file

    // 2. allocate frame
    let mut allocated_frame Frame = null;

    // 3. set allocate frame
    allocate_frame.set_file_desc(self.file_desc);
    allocate_frame.access();
    allocate_frame.clear_page();
    // 需要 file_header
    // allocate_frame.set_page_num(file_header_->page_count - 1);
    
    // 4. flush page internal

    frame = allocate_frame
    Ok(())
  }
}

/*
  @brief BufferPool迭代器，操作 DiskBufferPool
*/
struct BufferPoolIterator {
  // TODO(lyy): 手写一个？
  bitmap: [char],
  current_page_num: PageNum,
  disk_bp_manager: DiskBufferPool,
  start_page: PageNum;
}

impl BufferPoolIterator {
  pub fn new() -> BufferPoolIterator {
    BufferPoolIterator{}
  }

  pub fn init(bp: DiskBufferPool, start_page: PageNum) Result<(), std::io::Error> {
    // bitmap.init()
    if start_page <= 0 {
      current_page_num = 0;
    } else {
      current_page_num = start_page;
    }
    Ok(())
  }

  pub fn has_next() -> bool {

  }

  pub fn next() -> PageNum {

  }

  pub fn reset() -> Result<(), std::io::Error> {

  }
}