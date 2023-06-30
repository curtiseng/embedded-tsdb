如何保证消息的顺序？

日志以图 6 展示的方式组织。每一个日志条目存储一条状态机指令和从领导人收到这条指令时的任期号。日志中的任期号用来检查是否出现不一致的情况，
同时也用来保证图 3 中的某些性质。每一条日志条目同时也都有一个整数索引值来表明它在日志中的位置。

### 日志复制

+ 日志复制只能由领导者发起
+ 在正常非故障情况下，Leader的日志顺序和和Flower的日志顺序完全一样，主要靠日志index和term实现
+ 解释上一条，由于系统同时只有一个Leader，而重新选举会增加term号，所以某一个term中某一个index是确定唯一的

### Term

+ term会持久化之后发送，原因是避免重复vote
+ 如果一个Leader或者Candidate，发现自己的任期编号比起他节点小，立即恢复自己成为Flower

### ChangeMember

+ change member must add a member as leaner, but etcd does not need?

### Q&A

+ 如果一个term，只有一个node有，说明这个log未提交，但这是触发了重新选举，那么这个log又应用到了其他node，对于client，没有写入成功，但实际以及写入成功了，这种不一致是正常的吗？