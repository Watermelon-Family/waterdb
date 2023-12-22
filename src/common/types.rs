// 磁盘文件按照 Page 来组织，每一页都有对应的页号
pub type PageNum = u32;

// Page 中每一行都有对应的 SlotNum
pub type SlotNum = u32;