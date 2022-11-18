# Consensus bugs

[中文版](CN.md)

<!-- DO NOT EDIT README.md directly. It is built from [src/README.md](src/README.md). -->

It's challenging to detect bugs in the realm of distributed consensus, and event
a small problem could result in data loss.
This repo is a list of distributed consensus protocol's bugs, flaws, and deceptive traps.

|  Issue type    | description                                                        |
|  ---           | ---                                                                |
| **Bug**        | a bug that will break the consensus.                               |
| **Trap**       | not a bug, but somehow misleading. People may believe it is a bug. |
| **Suboptimal** | a solution that works, but not in the best way.                    |

## Issues

<!-- START doctoc generated TOC please keep comment here to allow auto update -->
<!-- DON'T EDIT THIS SECTION, INSTEAD RE-RUN doctoc TO UPDATE -->


<!-- END doctoc generated TOC please keep comment here to allow auto update -->

<!-- #### List -->

---

![](list/classic-paxos-forget-decided-value.md)

![](list/asymmetric-paxos.md)

![](list/raft-leader-step-down.md)

---

**Contribution**

Thank you for sharing an distributed consensus bug/issue.
Even a small problem could result in data loss.

- Update or add a snippet in the [src/list](src/list).

- Update the link entries in [src/README.md](src/README.md).

- `README.md` will be built in the next push to main branch.
