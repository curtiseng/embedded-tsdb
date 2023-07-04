# Design

+ 计算基于管道，管道携带scheme，类似于表，但是没有持久化，存在于内存中
+ 管道可以发送的远端，也可以持久化到本地
+ Edge first，Cloud Second
+ Stream first, analyse second

### Lib

提供table scheme，提供query

### Items
> DB, Table, Device, Tag(倒排索引), Field, Value

+ Every db is a tenant
+ Every table is a device model, include tag and field, support consistency
+ Every device is belonged table, with value, and device support quorum