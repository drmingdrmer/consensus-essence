# Distributed Consensus Essence

It's challenging to design, to implement or to detect bugs in the realm of distributed consensus, and even
a small problem could result in data loss.
This repo is a list of distributed consensus protocol's bugs, flaws, deceptive traps, and improvements.

|  Type          | description                                                        |
|  ---           | ---                                                                |
| **Bug**        | a bug that will break the consensus.                               |
| **Trap**       | not a bug, but somehow misleading. People may believe it is a bug. |
| **Suboptimal** | a solution that works, but not in the best way.                    |
| **Optimize**   | Improvement to a current design.                                   |
| **Variant**    | Another way to implement a consensus.                              |

## Ideas

- [Paxos: (Trap): The Bug in Paxos Made Simple](src/list/classic-paxos-forget-decided-value/classic-paxos-forget-decided-value.md)
  | 🌎 [中文版](src/list/classic-paxos-forget-decided-value/classic-paxos-forget-decided-value.cn.md)
- [Paxos: (Optimize): Asymmetric Acceptors](src/list/asymmetric-paxos/asymmetric-paxos.md)
  | 🌎 [中文版](src/list/asymmetric-paxos/asymmetric-paxos.cn.md)
- [Paxos/Raft: (Generalize): Round Number Reversion](src/list/paxos-revert-rnd/paxos-revert-rnd.md)
  | 🌎 [中文版](src/list/paxos-revert-rnd/paxos-revert-rnd.cn.md)
- [Paxos: (Generalize): Partial Order Round Number = Paxos + 2PC](src/list/paxos-partial-order-rnd/paxos-partial-order-rnd.md)
  | 🌎 [中文版](src/list/paxos-partial-order-rnd/paxos-partial-order-rnd.cn.md)
- [Raft: (Suboptimal): Leader Step Down](src/list/raft-leader-step-down/raft-leader-step-down.md)
  | 🌎 [中文版](src/list/raft-leader-step-down/raft-leader-step-down.cn.md)
- [Raft: (Optimize): Commit log via RequestVote RPC](src/list/raft-election-append-entries/raft-election-append-entries.md)
  | 🌎 [中文版](src/list/raft-election-append-entries/raft-election-append-entries.cn.md)
- [Raft: (Optimize): ReadIndex: Less Wait](src/list/raft-read-index/raft-read-index.md)
  | 🌎 [中文版](src/list/raft-read-index/raft-read-index.cn.md)


---

**Contribution**

Thank you for sharing a distributed consensus bug/issue.
Even a small problem could result in data loss.

- Update or add a snippet in the [src/list](src/list).

- Update the link entries in [README.md](README.md).
