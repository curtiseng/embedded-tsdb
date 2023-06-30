# Design

+ 计算基于管道，管道携带scheme，类似于表，但是没有持久化，存在于内存中
+ 管道可以发送的远端，也可以持久化到本地
+ Edge first，Cloud Second
+ Stream first, analyse second

### Lib

提供scheme，提供query