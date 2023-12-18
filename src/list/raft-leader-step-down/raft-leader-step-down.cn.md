## Raft: (Suboptimal): Leader Step Down

> 在raft论文中：
> 6. Cluster membership changes
>
> 第二个问题是集群 Leader 可能不在新配置中。
> 在这种情况下，一旦 Leader 提交了 $C_{new}$ 日志条目，它就会退位（返回到追随者状态）。
>
> ![](raft-leader-step-down-std.jpeg)

但是 Leader  **不必** 立刻放弃领导权：

尽管它 **不应该**为其他 Candidate 投票，一个 Learner （也称为 Non-voter，从集群配置中移除的节点）仍然可以保持 Leader 地位（或成为 Candidate ），只要它愿意。这种不投票的 Leader ：

- 与正常 Leader 一样处理写操作，除了本地日志存储不计入 quorum。
- 与正常 Leader 一样处理读操作。

**注意**： Learner （Non-voter）也不必拒绝来自其他节点的投票请求。
因为raft确保使用倒数第二个已提交配置的 Candidate 永远不会成为 Leader 。Thanks to [Gao Xinge](https://www.zhihu.com/people/gao-xinge).

![](raft-leader-step-down-optimize.jpeg)

### 改进

当 Leader 提交 $C_{new}$ 时，它 **不会** 放弃 leadership，但会继续作为 Leader 服务。

通过这种方式，membership config 日志不需要被实现特别处理。只有在需要时才会移除（non-voting） Leader ：
通过关闭非投票 Leader 或通知其将领导权转移给另一个节点。


**参考文献**：

- [Raft consensus algorithm](https://raft.github.io/)
