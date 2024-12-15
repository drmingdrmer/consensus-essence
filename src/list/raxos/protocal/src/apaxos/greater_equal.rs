/// A type supporting `>=` comparison without implementing `PartialOrd`.
///
/// `GreaterEqual` does not enforce `transitivity`(`a >= b && b >= c implies a
/// >= c`) or `anti-symmetry`(`a >= b && b >= a implies a == b`) properties.
///
/// In `AbstractPaxos`, `GreaterEqual` is used for comparing pseudo time values,
/// ensuring monotonicity and valid time state transitions, e.g.:
/// A transition from `t_1` to `t_2` iff `t_2 >= t_1`.
///
/// Unlike `PartialOrd`, `GreaterEqual` allows cycles and non-transitive
/// relationships:
/// - Cycle: `a >= b && b >= c && c >= a`, cyclic is used by 2pc time.
/// - A relationship where `a >= b >= c`, but `!(a >= c)`. In this case, if `b`
///   exists, transitioning from `c` to `a` is allowed; however, if `b` is
///   absent, transitioning from `c` to `a` is not allowed.
// TODO: consider add RHS as parameter to GreaterEqual
pub trait GreaterEqual {
    /// if greater than `other`
    fn is_gt(&self, other: &Self) -> bool
    where Self: PartialEq {
        self.greater_equal(other) && self != other
    }

    fn greater_equal(&self, other: &Self) -> bool;
}

impl<T: PartialOrd> GreaterEqual for T {
    fn greater_equal(&self, other: &Self) -> bool {
        self >= other
    }
}
