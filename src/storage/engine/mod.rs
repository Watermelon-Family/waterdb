mod bitcask;

mod log;

mod iterator;

type KeyDir = std::collections::BTreeMap<Vec<u8>, (u64, u32)>;