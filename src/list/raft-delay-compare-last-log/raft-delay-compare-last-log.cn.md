## Raft: (Variant): Delay Comparing Last Log to AppendEntries RPC

> In the Raft paper:
> RequestVote RPC:
> 
> Arguments:
>     term         : candidate’s term
>     candidateId  : candidate   requesting vote
>     lastLogIndex : index of candidate’s last log entry (§5.4)
>     lastLogTerm  : term of candidate’s last log entry (§5.4)
>
> Results:
>     term         : currentTerm, for candidate to update itself
>     voteGranted  : true means candidate received vote
>
> Receiver implementation:
>     1. Reply false if term < currentTerm (§5.1)
>     2. If votedFor is null or candidateId, and candidate’s log is at
>        least as up-to-date as receiver’s log, grant vote (§5.2, §5.4)


当 Candidate 向其他成员请求 Vote 时, 需要将自己的 last log id(`lastLogTerm, lastLogIndex`) 也发给这些成员,
这些成员会拒绝 last log id 较小的 Candidate 的 Vote 请求. 以保证 Leader 选出后一定持有最大 last log id, 从而保证了已committed的log一定包含在leader的日志中, 这是因为raft 中一系列日志的commit的条件是last log id 达到最大, 具体体现在: 只有 Leader 当前 term 的 log 复制到 majority 才认为达到了commit的条件.

这里为保持选出的Leader 持有最大 last log id的条件不需要一定在Vote阶段保证, 它也可以转移到AppendEntries 阶段.
这样调整之后, 选出的 Leader 不一定持有所有已committed的log, 但也并不会在后面的AppendEntries阶段让已committed的日志丢失.
当这样一个在RequestVote阶段没有比较last log id的Leader选出后, 它必须要在AppendEntries阶段比较自己的last log id和Follower的last log id, 如果遇到Leader 自己的last log id较小则退出Leader, 这样依旧可以保证正确性.

```
N1  1-1  1-2
N2  1-1       
N3  1-1  2-2
N4  1-1  2-2
N5  1-1  2-2
---------------------------------------------------------------> log index
```

例如 在RequestVote 不比较 last log id的模式下, N1 可以在term=3 从N2 N3 得到Vote 而选为 Leader, 
虽然N3 的last log id 更大.
这时N3 的log (2-2) 是已committed, 但是没有包含在Leader N1 中.

后续N1复制 log时,

```
N1  1-1  1-2  3-3
N2  1-1  1-2     
N3  1-1  2-2
N4  1-1  2-2
N5  1-1  2-2
---------------------------------------------------------------> log index
```

错误: 3-3 现在成了最大 last log id, 这样会导致后续Leader选择N1的日志二覆盖掉2-2
