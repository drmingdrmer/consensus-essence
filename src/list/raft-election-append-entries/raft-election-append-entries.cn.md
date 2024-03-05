## Raft: (Optimize): Commit log via RequestVote RPC


这个优化允许 Candidate 在一个 RRT 内完成选举和首次 commit;
标准 Raft 中, 首次 commit 需要在 Election 阶段完成之后再复制一个 Noop log, 需要2个 RRT.


标准 Raft 的 Election 阶段不允许进行 log 复制:

> RequestVote RPC:
> ```
> Arguments:
>     term         : candidate’s term
>     candidateId  : candidate requesting vote
>     lastLogIndex : index of candidate’s last log entry (§5.4)
>     lastLogTerm  : term of candidate’s last log entry (§5.4)
> ```

我们可以优化协议, 允许 Raft 在 Election 阶段直接复制 log:

在 RequestVote RPC 中增加 AppendEntries RPC 的字段:

```
prevLogIndex : index of log entry immediately preceding new ones
prevLogTerm  : term of prevLogIndex entry
entries[]    : log entries to store
```

例如下图中,
- N₁: Term=3, last-log 是(Term=1, index=3),
- N₂: Term=4, last-log 是(Term=3, index=4),
- N₃: Term=2, last-log 是(Term=2, index=4),

N₂ 作为 Term=4 的 Candidate 发起election时,
将 prevLogTerm=1, prevLogIndex=2, entries=[(Term=1, index=3), (Term=3, index=4)]
的 AppendEntries 信息也一起发给 N₁ 和 N₃;

此时 N₁ 接受 log, N₂ 接受 log 并删除自己本地的 log: (Term=2, index=4),
它一定不是被 committed.

![](raft-election-append-entries.excalidraw.png)


**协议变化**:

收到 RequestVote 的 node Nᵢ, 在执行 Election 固有的逻辑之前,
执行 AppendEntries 的逻辑:

如果 RequestVote 中的最后一个log 的 term 不小于 Nᵢ 的 term, 即: `RequestVote.entries.last().term >= Nᵢ.term`

则按照 AppendEntries 的逻辑将日志追加到本地(需要`prevLogIndex , prevLogTerm, entries`),
并返回一个`AppendEntriesOk = true` 的应答; 否则返回`AppendEntriesOk = false`.


**Candidate 变化**:

同样, Candidate 如果从一个 majority 收到了 `AppendEntriesOk = true`,
则认为它复制出去的 log 已经 commit: `commit(RequestVote.entries.last().index)`.


### Proof-1

这个优化仍然保证 Raft 的正确性:

假设 RequestVote 中最后一条 log 的 term 是 `t₀`: `RequestVote.entries.last().term == t₀`;


1, 这个优化不会导致已 committed 的 log 丢失:

∵ `t₀ >= Nᵢ.term >= Nᵢ.entries.last().term`:

-   如果 `t₀ > Nᵢ.entries.last().term`,
    则 Nᵢ 本地的 log 一定不是已经 committed (Raft 已经保证了 term 最大的 log
    才可能是被 commit 的)

    ∴ 覆盖未 committed 的 log 不会导致已 committed 的 log 丢失.

-   如果 `t₀ == Nᵢ.entries.last().term`,
    则 Nᵢ 本地的 log 和 RequestVote 中的 log 一定是 **兼容的**, 即只可能一个是另一个的子串.

    ∴ 这时 AppendEntries 操作只会合并log, 也不会导致已 committed 的 log 丢失.


2, Candidate 复制的 log 完成了 commit

∵ Candidate 收到了 majority 回复的 `AppendEntriesOk = true`,

∴ 所以任何其他大于 t₀ 的 Candidate 都会看到 `RequestVote.entries`;
即后续更大 term 的 Leader, 都包含这些 log,

∴ `RequestVote.entries` 满足了 committed 的条件


### Proof-2

也可以认为 Candidate 接替了 t₀ 的 Leader 继续复制 log(但不 propose 新 log),

∴ 遵循同样的协议, 复制和 commit 都不会破坏 Raft 的正确性.
