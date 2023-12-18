## Raft: (Suboptimal): Leader Step Down

> In the raft paper:
> 6. Cluster membership changes
>
> The second issue is that the cluster leader may not be part of the new configuration.
> In this case, the leader steps down (returns to follower state) once it has committed the $C_{new}$ log entry.
>
> ![](raft-leader-step-down-std.jpeg)

But the leader does **NOT** have to give up leadership:

Despite it **should not** cast a ballot(vote) for other candidates, a learner(AKA
non-voter, a node removed from cluster config) can nevertheless be a leader(or
become a candidate) as long as it wants. This non-voting leader:

- handles write operations in the same way as a normal leader, except the local log store does not count in majority.
- handles read operations in the same way as a normal leader.

**NOTE**: A learner(non-voter) does not have to reject vote requests.
Because raft ensures that a candidate using the second-to-last committed config
would never become the leader. Thanks to [Gao Xinge](https://www.zhihu.com/people/gao-xinge).

![](raft-leader-step-down-optimize.jpeg)

#### Improvement

When a leader commits $C_{new}$, it does **NOT** give up leadership, but just
keep serving as leader.

This way, membership config log does not need to be dealt with specially by an
implementation. The (non-voting) leader will be removed only if it is required:
by just shutting down the non-voting leader or informing it to transfer its
leadership to another node.


**References**:

- [Raft consensus algorithm](https://raft.github.io/)
