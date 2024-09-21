use std::fmt::Debug;

use crate::apaxos::greater_equal::GreaterEqual;

/// Pseudo time that is used in a distributed system.
///
/// `Time`s can be compared with `==` or `>=`.
/// But note that the **greater-or-equal** relation of `Time` is **NOT**
/// transitive and must **NOT** form a cycle, i.e., `Time` is **NOT**
/// `PartialOrd`. `Time` is a [DAG]:
///
/// - Reflexivity: `a >= a`
/// - Antisymmetry: `a >= b` and `b >= a` implies `a == b`
/// - Anti-transitivity: `a >= b` and `b >= c` does **NOT** imply `a >= c`
///
/// See: [Partially-Ordered-set]
/// See: [DAG]
/// See: [Topological-order]
///
/// [Partially-Ordered-set]: https://en.wikipedia.org/wiki/Partially_ordered_set
/// [DAG]: https://en.wikipedia.org/wiki/Directed_acyclic_graph
/// [Topological-order]: https://en.wikipedia.org/wiki/Topological_sorting
pub trait Time: Default + Debug + Clone + Copy + PartialEq + Eq + GreaterEqual + 'static {}

impl<T> Time for T where T: Default + Debug + Clone + Copy + PartialEq + Eq + GreaterEqual + 'static {}
