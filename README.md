# Distributed Consensus Essence

[中文版](CN.md)

<!-- DO NOT EDIT README.md directly. It is built from [src/README.md](src/README.md). -->

It's challenging to design, to implement or to detect bugs in the realm of distributed consensus, and even
a small problem could result in data loss.
This repo is a list of distributed consensus protocol's bugs, flaws, deceptive traps, and improvements.

<table>
<tr class="header">
<th>Issue type</th>
<th>description</th>
</tr>
<tr class="odd">
<td><strong>Bug</strong></td>
<td>a bug that will break the consensus.</td>
</tr>
<tr class="even">
<td><strong>Trap</strong></td>
<td>not a bug, but somehow misleading. People may believe it is a bug.</td>
</tr>
<tr class="odd">
<td><strong>Suboptimal</strong></td>
<td>a solution that works, but not in the best way.</td>
</tr>
<tr class="even">
<td><strong>Optimize</strong></td>
<td>Improvement to a current design.</td>
</tr>
</table>

## Issues

<!-- START doctoc generated TOC please keep comment here to allow auto update -->
<!-- DON'T EDIT THIS SECTION, INSTEAD RE-RUN doctoc TO UPDATE -->

- [Paxos: (Trap): The Bug in Paxos Made Simple](#paxos-trap-the-bug-in-paxos-made-simple)
- [Paxos: (Optimize): Asymmetric Acceptors](#paxos-optimize-asymmetric-acceptors)
- [Raft: (Suboptimal): Leader Step Down](#raft-suboptimal-leader-step-down)
- [Raft: (Optimize): ReadIndex: Less Wait](#raft-optimize-readindex-less-wait)

<!-- END doctoc generated TOC please keep comment here to allow auto update -->

<!-- #### List -->

---

## Paxos: (Trap): The Bug in Paxos Made Simple

This is not a bug but people tend to interpret it in the wrong way.

#### The issue:

```
1. P1 sends 'prepare 1' to AB
2.  Both AB respond P1 with a promise to not to accept any request numbered smaller than 1.\
    Now the status is: A(-:-,1) B(-:-,1) C(-:-,-)
3.  P1 receives the responses, then gets stuck and runs very slowly
4.  P2 sends 'prepare 100' to AB
5.  Both AB respond P2 with a promise to not to accept any request numbered smaller than 100.
    Now the status is: A(-:-,100) B(-:-,100) C(-:-,-)
6.  P2 receives the responses, chooses a value b and sends 'accept 100:b' to BC
7.  BC receive and accept the accept request, the status is: A(-:-,100) B(100:b,100) C(100:b,-).
    Note that proposal 100:b has been chosen.
8.  P1 resumes, chooses value a and sends 'accept 1:a' to BC
9.  B doesn't accept it, but C accepts it because C has never promise anything.
    Status is: A(-:-,100) B(100:b,100) C(1:a,-). The chosen proposal is abandon, Paxos fails.
```

#### Explanation:

Missed something in step 7.
When C processes `accept 100:b` it sets its state to `C(100:b,100)`.
**By accepting a value the node is also promising to not accept earlier values.**

Sadly:

> What's more I looked through several proprietary and open-source paxos
> implementations and they **all had the bug submitted by the OP**!


**References**:

-   [Marc Brooker's blog](https://brooker.co.za/blog/2021/11/16/paxos.html)
-   [On stackoverflow](https://stackoverflow.com/questions/29880949/contradiction-in-lamports-paxos-made-simple-paper)

## Paxos: (Optimize): Asymmetric Acceptors

An [erasure-code](https://en.wikipedia.org/wiki/Erasure_code) like algorithm can be applied to the storage layer of
paxos to reduce data redundancy.

In [classic Paxos](http://lamport.azurewebsites.net/pubs/pubs.html#paxos-simple),
acceptors are **symmetric**:

![classic](https://cdn.jsdelivr.net/gh/drmingdrmer/consensus-essence@ci-asset/README/8f2689f1e7dba5f9-asymmetric-paxos-classic.jpeg)

A proposer(quorum: $q_i$) stores value $x$ on acceptors(at least 2 acceptors) to commit $x$.
To rebuild(read) $x$ from acceptors, another proposer(quorum: $q_j$) has to visit one of the acceptor that holds the committed value.
Thus two quorums must have at least 1 common acceptors: $|q_i \cap q_j| \ge 1$.
I.e., a quorum for a cluster of 3 is any 2 acceptors: $|q_i| \ge 2$.

Redundancy is **300%**; Tolerates **1** failure; Availability is about ${ 3 \choose 2  } p^2$, where $p$ is acceptor failure rate.

**Asymmetric Paxos**:
Because we can rebuild $x, y$ from a linear equation system $ax+by=d_1, cx+dy=d_2$,
acceptor states can be **asymmetric** so that more data can be stored:

![ec](https://cdn.jsdelivr.net/gh/drmingdrmer/consensus-essence@ci-asset/README/0bb845a2df1a5134-asymmetric-paxos-ec.jpeg)

A proposer(quorum: $q_i$) stores $x, y, x+y, x-y$ on acceptor 1 to 4(at least 3 of them) to commit $x, y$.
To rebuild(read) $x, y$ from acceptors, another proposer(quorum: $q_j$) has to visit at least two of the **4 values**.
Thus two quorums must have at least 2 common acceptors: $|q_i \cap q_j| \ge 2$.
A quorum for a cluster of 4 is any 3 acceptors: $|q_i| \ge 3$.

With such a policy: Redundancy is **200%**; Tolerates **1** failure; Availability is about ${ 4 \choose 2  } p^2$, where $p$ is acceptor failure rate.

Another example is **asymmetric Paxos 5-4**: 5 asymmetric acceptors can store 3 independent values
$x, y, z$:

![ec53](https://cdn.jsdelivr.net/gh/drmingdrmer/consensus-essence@ci-asset/README/586e20c6dfc9460f-asymmetric-paxos-ec-53.jpeg)

A proposer stores $x, y, z, x+y+z, x+2y+4z$ on acceptor 1 to 5.
To rebuild these 3 values, this must hold: $|q_i \cap q_j| \ge 3$.
Thus quorum size is at least 4: $|q_i| \ge 4$.

Redundancy is **140%**; Tolerates **1** failure; Availability is about ${ 5 \choose 2  } p^2$.

**Summary**: with asymmetric paxos, the avaiability decreases slightly while the data redundancy is reduced in [asymmetric Paxos](https://github.com/drmingdrmer/consensus-bugs#paxos-optimize-asymmetric-acceptors).
This algorithm applies to paxos and its variants but not to [raft](https://raft.github.io/).
Because it requires more than one nodes to rebuild a committed value.

![chart](https://cdn.jsdelivr.net/gh/drmingdrmer/consensus-essence@ci-asset/README/781c336bed9bc848-asymmetric-paxos-chart.jpeg)

## Raft: (Suboptimal): Leader Step Down

> In the raft paper:
> 6. Cluster membership changes
> 
> The second issue is that the cluster leader may not be part of the new configuration.
> In this case, the leader steps down (returns to follower state) once it has committed the $C_{new}$ log entry.
> 
> ![](https://cdn.jsdelivr.net/gh/drmingdrmer/consensus-essence@ci-asset/README/b29339428b745edd-raft-leader-step-down-std.jpeg)


But the leader does **NOT** have to give up leadership:

Despite it **should not** cast a ballot(vote) for other candidates, a learner(AKA
non-voter, a node removed from cluster config) can nevertheless be a leader(or
become a candidate) as long as it wants. This non-voting leader:

-   handles write operations in the same way as a normal leader, except the local log store does not count in majority.
-   handles read operations in the same way as a normal leader.

**NOTE**: A learner(non-voter) does not have to reject vote requests.
Because raft ensures that a candidate using the second-to-last committed config
would never become the leader. Thanks to [Gao Xinge](https://www.zhihu.com/people/gao-xinge).

![](https://cdn.jsdelivr.net/gh/drmingdrmer/consensus-essence@ci-asset/README/cb9ebf5135722aaa-raft-leader-step-down-optimize.jpeg)

#### Improvement

When a leader commits $C_{new}$, it does **NOT** give up leadership, but just
keep serving as leader.

This way, membership config log does not need to be dealt with specially by an
implementation. The (non-voting) leader will be removed only if it is required:
by just shutting down the non-voting leader or informing it to transfer its
leadership to another node.

**References**:

-   [Raft consensus algorithm](https://raft.github.io/)

## Raft: (Optimize): ReadIndex: Less Wait

### ReadIndex Process in Raft

Raft's ReadIndex process ensures consistent reads across the cluster through these steps:

1.  **Step-1: Initial Commit Check**: The leader checks if its latest term's entry is committed. If not, it defers the read.

1.  **Step-2: Setting the ReadIndex**: Once a log is committed for the current term, the leader sets the ReadIndex to its current CommitIndex.

1.  **Step-3: Leader Confirmation**: A heartbeat is sent to the quorum to ensure no other higher term exists at that moment.

1.  **Step-4: Index Synchronization**: The leader waits for the StateMachine to apply entries up to the ReadIndex before reading, ensuring data consistency.

The steps in Raft's ReadIndex process are designed to ensure linearizable reads, guaranteeing that a read operation will reflect all preceding updates in the system's timeline.

Such an implementation is popular, e.g., in [etcd-raft](https://github.com/etcd-io/raft/blob/4fcf99f38c20868477e01f5f5c68ef1e4377a8b1/raft.go#L2063-L2098).

#### Proof of Linearizable Consistency

![](https://cdn.jsdelivr.net/gh/drmingdrmer/consensus-essence@ci-asset/README/0e3c320ca5817065-raft-read.excalidraw.png)

When a Leader node(at term `Term_1`) receives a read request labeled `read_1` at time `time_1`, it must ensure that `read_1` sees all data observed by any prior read `read_0` at `Term_0`, which took place at `time_0`.

Assume `read_0` saw the StateMachine's data up to log entry `(Term_0, index_0)`. We then consider three potential scenarios regarding `Term_0`:

-   **case-gt: Higher Term** (`Term_0 > Term_1`): A heartbeat sent at time `t`(`t > time_1`) prevents the confusion of having a higher term exists in time range `(0, t)`(Step-3).

-   **case-lt: Lower Term** (`Term_0 < Term_1`): Raft guarantees that the current Leader has all logs that are previously committed. Consequently, `index_0` must be lower than the no-op log index (`NoopIndex`) that is written upon the Leader's election. For consistency, the StateMachine must include logs up to at least this no-op entry(`NoopIndex`) when `read_1` is processed.

-   **case-eq: Equal Term** (`Term_0 == Term_1`): In this case, the `read_0` is definitely conducted by the current node; Since Raft ensures only committed logs are visible to reads, `read_1` must be able to observe all data up to the current CommitIndex, which `read_0` would have seen.

By carefully managing these scenarios, the Raft protocol ensures that reads are linearizable:

-   **case-gt: Higher Term** is preemptively resolved with the Leader's heartbeat confirmation (Step-3).
-   **case-lt: Lower Term** is handled once the no-op log is committed (Step-1), leaving only the **case-eq: Equal Term** to be addressed(Step-2).
-   For the **case-eq: Equal Term**, the Leader simply ensures the StateMachine has applied all entries up to the CommitIndex before processing the read (Step-4), guaranteeing that `read_1` observes all data seen by `read_0`.

### Optimization

In the **Step-1**, the leader does not have to defer the read. Instead, it
just sets ReadIndex to be `max(CommitIndex, NoopIndex)`. The **Step-3** and
**Step-4** does not change.

It is obviously correct because it simply combines requirements of **case-lt** and **case-eq**.

[Openraft](https://github.com/datafuselabs/openraft) implements this optimization: A brief proof can be found in: [Openraft linearizable read](https://github.com/datafuselabs/openraft/blob/79372b4dff4312f5eb344db76d5ed1dffe69fac7/openraft/src/docs/protocol/read.md).

For Openraft application, it simply calls and blocks until [`Raft::ensure_linearizable()`][] returns, and the proceed reading process.
For example, [kv-store](https://github.com/datafuselabs/openraft/blob/79372b4dff4312f5eb344db76d5ed1dffe69fac7/examples/raft-kv-memstore/src/network/api.rs#L42) implemnts linearizable read with the following snippet:

```rust
async fn read (app: Data<App>, req: Json<String>) -> Result<impl Responder> {
    app.raft.ensure_linearizable().await;

    let state_machine = app.store.state_machine. read ().await;
    let value = state_machine.data.get(&req.key).cloned();
    Ok(value)
}
```

Internally, a simplified implementation of `ensure_linearizable()` would look
like the following:

```rust
async fn ensure_linearizable(&mut self, tx: ClientReadTx<C>) {

    let read_log_id = {
        let leader_first = self.log_ids.by_last_leader().first();
        let committed = self.committed();

        std::cmp::max(leader_first, committed)
    };

    do_send_heartbeat_to_quorum().await;

    self.wait(None)
        .applied_index_at_least(read_log_id.index())
        .await;
}
```

### Benefits of the Optimization

This optimization not only reduces the potential **effective** waiting period but also logically decreases the likelihood of **ineffective** waiting.

For example:

-   When a read request arrives and the Leader has not yet committed its term's noop log, the CommitIndex is at `c1`, and the NoopIndex at `n1`, clearly `c1 < n1`;

    -   Without this optimization, the caller would be suspended and wait;
    -   With this optimization, the caller waits only until `n1` is applied to the StateMachine.

-   Suppose there is a new election and the current node becomes the Leader again (naturally, in a greater Term), and assume the Leader’s noop log is still not committed; now the CommitIndex is at `c2`, and the NoopIndex at `n2`, `c2 < n2`.

In this scenario, we can see the difference between with and without this optimization:

-   Without this optimization, there might be a need to wait again for a larger log index to be committed: `max(c2, n2)`;
-   With it, it avoids this kind of **livelock** and only requires waiting for the original `max(c1, n1)` to be applied;

Therefore, in standard Raft, it is **possible** that a read request may never be executed, while in Openraft, this situation does not occur.

Of course, this is an unlikely issue, but in standard Raft, we still need to spend time considering and proving that it **will not cause a problem**. In contrast, Openraft's simplified logic completely avoids the occurrence of **livelocks**, making correctness verification easier.

The key to optimization is to simplify logic, not necessarily to reduce code (in terms of form). Simple logic sometimes requires more code to describe, and less code can also mean more complex underlying logic.

---

**Contribution**

Thank you for sharing an distributed consensus bug/issue.
Even a small problem could result in data loss.

-   Update or add a snippet in the [src/list](src/list).

-   Update the link entries in [src/README.md](src/README.md).

-   `README.md` will be built in the next push to main branch.



Reference:

- Openraft : [https://github.com/datafuselabs/openraft](https://github.com/datafuselabs/openraft)

- etcd-raft-read-index : [https://github.com/etcd-io/raft/blob/4fcf99f38c20868477e01f5f5c68ef1e4377a8b1/raft.go#L2063-L2098](https://github.com/etcd-io/raft/blob/4fcf99f38c20868477e01f5f5c68ef1e4377a8b1/raft.go#L2063-L2098)

- kv-store : [https://github.com/datafuselabs/openraft/blob/79372b4dff4312f5eb344db76d5ed1dffe69fac7/examples/raft-kv-memstore/src/network/api.rs#L42](https://github.com/datafuselabs/openraft/blob/79372b4dff4312f5eb344db76d5ed1dffe69fac7/examples/raft-kv-memstore/src/network/api.rs#L42)

- read : [https://github.com/datafuselabs/openraft/blob/79372b4dff4312f5eb344db76d5ed1dffe69fac7/openraft/src/docs/protocol/read.md](https://github.com/datafuselabs/openraft/blob/79372b4dff4312f5eb344db76d5ed1dffe69fac7/openraft/src/docs/protocol/read.md)


[Openraft]: https://github.com/datafuselabs/openraft
[etcd-raft-read-index]: https://github.com/etcd-io/raft/blob/4fcf99f38c20868477e01f5f5c68ef1e4377a8b1/raft.go#L2063-L2098
[kv-store]: https://github.com/datafuselabs/openraft/blob/79372b4dff4312f5eb344db76d5ed1dffe69fac7/examples/raft-kv-memstore/src/network/api.rs#L42
[read]: https://github.com/datafuselabs/openraft/blob/79372b4dff4312f5eb344db76d5ed1dffe69fac7/openraft/src/docs/protocol/read.md