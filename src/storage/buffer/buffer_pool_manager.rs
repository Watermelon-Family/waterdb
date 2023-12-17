use std::{sync::Mutex, collections::HashMap};

use super::{frame_manager::FrameManager, disk_buffer_pool::DiskBufferPool};

pub struct BufferPoolManager {
    frame_manager: FrameManager,
    lock: Mutex<i32>,

    buffer_pools: HashMap<String, DiskBufferPool>,
    fd_buffer_pools: HashMap<i32, DiskBufferPool>,
}