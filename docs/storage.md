# MVCC

waterdb 支持 MVCC，提供的隔离级别是快照隔离。在事务眼中的数据库是创建事务的一个 snapshot。事务只能看到 snapshot 的数据，以及自己写入的数据。保证了隔离性、可重复读以及避免了幻读。但是会产生 W-W 冲突。

由于没有实现 GC，所以 waterdb 支持 time-travel query，即给定 txn，可以创建出对应状态的 snapshot。

- 原子性：在事务 commit 的时候，会将他从 active 中删除。这样之后创建的事务都可以看到当前事务的修改，从而实现原子性。

- 可重复读：当创建事务的时候，会创建出对应 txn active-set。所以这些 active-set 事务的修改对该 txn 是不可见的。

