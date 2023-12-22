/// 使用 bincode 包中的函数进行数据的序列话反序列化操作
use bincode::Options;
use lazy_static::lazy_static;

lazy_static! {
    static ref BINCODE: bincode::DefaultOptions = bincode::DefaultOptions::new();
}

pub fn deserialize<'a, T: serde::Deserialize<'a>>(bytes: &'a [u8]) -> crate::Result<T> {
    Ok(BINCODE.deserialize(bytes)?)
}

pub fn serialize<'a, T: serde::Serialize>(value: &T) -> crate::Result<Vec<u8>> {
    Ok(BINCODE.serialize(value)?)
}