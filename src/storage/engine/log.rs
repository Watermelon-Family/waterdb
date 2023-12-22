use std::{path::PathBuf, fs::{File, self}, io::{BufReader, Seek, SeekFrom, Read, BufWriter, Write}};

use super::KeyDir;

pub(crate) struct Log {
    pub(crate) path: PathBuf,
    pub(crate) file: File,
}

impl Log {
    pub(crate) fn new(path: PathBuf) -> crate::Result<Log> {
        if let Some(path) = path.parent() {
            fs::create_dir_all(path)?;
        }
        let file = std::fs::OpenOptions::new().read(true).write(true).create(true).open(&path)?;
        Ok(Log { path, file })
    }

    pub(crate) fn build_keydir(&mut self) -> crate::Result<KeyDir> {
        let file_len = self.file.metadata()?.len();

        let mut len_buf = [0 as u8; 4];
        let mut keydir = KeyDir::new();

        let mut reader = BufReader::new(&mut self.file);
        let pos = reader.seek(SeekFrom::Start(0))?;

        while pos < file_len {
            reader.read_exact(&mut len_buf)?;
            let key_len = u32::from_be_bytes(len_buf);

            reader.read_exact(&mut len_buf)?;
            let value_len_or_tombstone = match i32::from_be_bytes(len_buf) {
                -1 => None,
                value_len => Some(value_len as u32), 
            };

            // | key len 4bytes | value len 4bytes | key | value |
            //                                           ^
            //                                        value_pos
            let value_pos = pos + 4 + 4 + key_len as u64;

            let mut key = vec![0; key_len as usize];
            reader.read_exact(&mut key)?;

            match value_len_or_tombstone {
                Some(value_len) => {
                    if value_len as u64 + value_pos > file_len {
                        log::error!("Found incomplete entry at offset {}, truncating file", pos);
                        return Err(
                            "value extends beyond end of file".into()
                        );
                    } else {
                        reader.seek_relative(value_len as i64)?;
                        keydir.insert(key, (value_pos, value_len));
                    }
                },
                None => {
                    keydir.remove(&key);
                }
            }
        }

        Ok(keydir)
    }

    pub(crate) fn read_value(&mut self, value_pos: u64, value_len: u32) -> crate::Result<Vec<u8>> {
        let mut value_buf = vec![0; value_len as usize];
        self.file.seek(SeekFrom::Start(value_pos))?;
        self.file.read_exact(&mut value_buf)?;

        Ok(value_buf)
    }

    /// 将 key value 写入 log 文件中，返回记录起始位置 pos 和记录的长度 len
    pub(crate) fn write_entry(&mut self, key: &[u8], value: Option<&[u8]>) -> crate::Result<(u64, u32)> {
        let key_len = key.len() as u32;
        let value_len = value.map_or(0, |v| v.len() as u32);
        let value_len_or_tombstone = value.map_or(-1, |v| v.len() as i32);
        let len = 4 + 4 + key_len + value_len;


        let pos = self.file.seek(SeekFrom::End(0))?;
        let mut writer = BufWriter::with_capacity(len as usize, &mut self.file);

        writer.write_all(&key_len.to_be_bytes())?;
        writer.write_all(&value_len_or_tombstone.to_be_bytes())?;
        writer.write_all(key)?;

        if let Some(value) = value {
            writer.write_all(value)?;
        }
        writer.flush()?;
        Ok((pos, len))
    }

    pub(crate) fn total_size(&self) -> crate::Result<u64> {
        Ok(self.file.metadata()?.len())
    }

    pub(crate) fn flush(&self) -> crate::Result<()> {
        self.file.sync_all()?;
        Ok(())
    }
}