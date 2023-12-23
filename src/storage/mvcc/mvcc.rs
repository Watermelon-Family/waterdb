/// 将实现 MVCC，MVCC 广泛用于保证 ACID 以及并发控制。
/// 使得多个事务可以同时隔离的并发访问同一个数据集，并且处理冲突，
/// 当事务 commit 的时候，实现原子性写入
use std::sync::{Arc, Mutex};

use serde_derive::{Serialize, Deserialize};

use crate::{storage::engine::Engine, error::Result};

use super::{transaction::{Transaction, TransactionState}, key::{Version, Key, KeyPrefix}};

pub struct MVCC<E: Engine> {
    engine: Arc<Mutex<E>>,
}

/// MVCC engine 状态
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Status {
    /// MVCC 事务总数
    pub versions: u64,
    /// 目前存活事务的数量
    pub active_txns: u64,
    /// engine 的状态
    pub storage: crate::storage::engine::Status,
}

impl<E: Engine> Clone for MVCC<E> {
    fn clone(&self) -> Self {
        MVCC { engine: self.engine.clone() }
    }
}

impl<E: Engine> MVCC<E> {
    pub fn new(engine: E) -> Self {
        Self { engine: Arc::new(Mutex::new(engine)) }
    }

    pub fn begin(&self) -> Result<Transaction<E>> {
        Transaction::begin(self.engine.clone())
    }

    pub fn begin_read_only(&self) -> Result<Transaction<E>> {
        Transaction::begin_read_only(self.engine.clone(), None)
    }

    pub fn begin_as_of(&self, version: Version) -> Result<Transaction<E>> {
        Transaction::begin_read_only(self.engine.clone(), Some(version))
    }

    pub fn resume(&self, state: TransactionState) -> Result<Transaction<E>> {
        Transaction::resume(self.engine.clone(), state)
    }

    pub fn get_unversioned(&self, key: &[u8]) -> Result<Option<Vec<u8>>> {
        self.engine.lock()?.get(&Key::Unversioned(key.into()).encode()?)
    }

    pub fn set_unversioned(&self, key: &[u8], value: Vec<u8>) -> Result<()> {
        self.engine.lock()?.set(&Key::Unversioned(key.into()).encode()?, value)
    }

    pub fn status(&self) -> Result<Status> {
        let mut engine = self.engine.lock()?;
        let versions = match engine.get(&Key::NextVersion.encode()?)? {
            Some(ref v) => bincode::deserialize::<u64>(v)? - 1,
            None => 0,
        };
        let active_txns = engine.scan_prefix(&KeyPrefix::TxnActive.encode()?).count() as u64;
        Ok(Status { versions, active_txns, storage: engine.status()? })
    }
}