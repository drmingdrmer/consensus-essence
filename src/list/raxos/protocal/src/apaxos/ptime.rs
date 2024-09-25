use std::fmt::Debug;
use std::hash::Hash;

/// Pseudo time that is used in a distributed system.
///
/// `Time` is [Partially-Ordered-set]:
///
/// - Reflexivity: `a >= a`
/// - Antisymmetry: `a >= b` and `b >= a` implies `a == b`
/// - Transitivity: `a >= b` and `b >= c` implies `a >= c`
///
/// See: [Partially-Ordered-set]
/// See: [DAG]
/// See: [Topological-order]
///
/// [Partially-Ordered-set]: https://en.wikipedia.org/wiki/Partially_ordered_set
/// [DAG]: https://en.wikipedia.org/wiki/Directed_acyclic_graph
/// [Topological-order]: https://en.wikipedia.org/wiki/Topological_sorting
pub trait Time:
    Default + Debug + Clone + Copy + PartialEq + Eq + Hash + PartialOrd + 'static
{
}

impl<T> Time for T where T: Default + Debug + Clone + Copy + PartialEq + Eq + Hash + PartialOrd + 'static
{}
