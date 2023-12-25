# 🍉db

🍉 is a simple relation database, inspired by miniob-2023, mini-redis, leveldb, simple-db and toydb.

```rust
// run server
cargo run --bin waterdb-server

// run client
cargo run --bin waterdb-cli
```

## 🍉DB SQL Engine

🍉db 为每一个 sokect 都会创建一个 🍉 session。session 是支持事务的（快照隔离）。后续 client 的操作都是直接和 session 交互的。

用户输入的 query 会进过下列阶段，最后得到执行结果：

1. Parser：将用户输入的 query string 转化为 Statement
2. Planner：将 Statement 转化为执行计划（逻辑执行计划）
3. Optimizer: 将执行计划进行优化，例如谓词下推等等。得到优化之后的执行计划。
4. Executor：根据逻辑计划生成物理执行计划(算子)。每一个算子都会有对应的 executor 方法。

在 SQL Engine 中，每一条 SQL 的执行都是通过事务的。Txn 通过 Engine 生成。

🍉DB 的元数据通过 Catalog trait 进行管理，即实现 Catalog 即可执行 DDL。Catalog trait 会被 Txn 实现。从而使得 Txn 可以执行 DDL。

🍉DB 的数据管理则是 Transaction trait 管理的，通过实现 Transaction trait 使得 Txn 可以执行 DML。



参考 miniob并且得益于 rust 中枚举类型的强大，🍉db 中 Value 通过枚举实现。

```rust
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum Value {
    Null,
    Boolean(bool),
    Integer(i64),
    Float(f64),
    String(String),
}
```

对于行数据是通过 `Vec<Value>` 实现的。

## 🍉DB Storage Engine

存储引擎是通过 Bitcask 实现的。Bitcask 是基于日志的。类似于 LSM Tree，所有的操作都是通过 Append Entry 的方式实现的。

![Bitcask](https://pic3.zhimg.com/80/v2-0179c2d04ed19b9e3fad9f842bd16b52_1440w.webp)

一条 Entry 的结构如图。Bitcask 通过在内存中维护一个 Map，key 为存储的 key。value 是 Entry 的 metadata。



### MVCC

>  MVCC 多版本并发控制：指在数据库中维护一条记录的多个副本。达到写不影响读，读不影响写的效果。由于指解决了 R-W 的冲突，并没有解决 W-W 的冲突，所以 MVCC 控制协议并不完整。WaterDB 采用了乐观锁的机制，在出现 W-W 冲突的时候会进行 retry。MV2PL 和 MVOCC 可以解决 W-W 冲突的问题。
>
> time-travel：如果保存之前全部的副本信息，DBMS就可以支持读取任意版本的信息

🍉db 支持 MVCC，提供的隔离级别是快照隔离。在事务眼中的数据库是创建事务的一个 snapshot。事务只能看到 snapshot 的数据，以及自己写入的数据。保证了隔离性、可重复读以及避免了幻读。但是会产生 W-W 冲突。

由于没有实现 GC，所以 🍉db 支持 time-travel query，即给定 txn，可以创建出对应状态的 snapshot。

- 原子性：在事务 commit 的时候，会将他从 active 中删除。这样之后创建的事务都可以看到当前事务的修改，从而实现原子性。

- 可重复读：当创建事务的时候，会创建出对应 txn active-set。所以这些 active-set 事务的修改对该 txn 是不可见的。

由于 rust 可以对复杂类型的 Enum 类型进行序列化操作，所以可以借助 bitcask 轻松实现 MVCC。

