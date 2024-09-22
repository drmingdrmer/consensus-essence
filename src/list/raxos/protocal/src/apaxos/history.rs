use std::fmt::Debug;

use crate::apaxos::greater_equal::GreaterEqual;
use crate::Types;

pub struct TimeEvent<T: Types> {
    time: T::Time,
    event: T::Event,
}

impl<T: Types> TimeEvent<T> {
    pub fn new(time: T::Time, event: T::Event) -> Self {
        Self { time, event }
    }
}

pub trait History<T: Types>
where Self: Default + Debug + Clone
{
    fn append(&mut self, time: T::Time, event: T::Event);

    fn get(&self, time: &T::Time) -> Option<&T::Event>;

    /// Return a sub set of the history that is visible at `time`.
    ///
    /// In other words, a sub set of TimeEvent that is less than or equal to
    /// `time`.
    fn visible(&self, time: T::Time) -> Self;

    /// Return the maximal [`Time`] and [`Event`] in the history.
    ///
    /// `maximal` is defined as:
    /// g in P is a maximal element:
    /// if there is no element a in P such that a > g
    ///
    /// All `maximal` have no order between them.
    fn maximals(&self) -> impl Iterator<Item = (T::Time, T::Event)>;

    fn maximal_times<'a>(&'a self) -> impl Iterator<Item = T::Time> + 'a {
        self.maximals().map(|(t, _)| t)
    }

    /// Merge two [`History`]
    ///
    /// Note that if there are `maximal` that have an order, the smaller one
    /// will be removed. Because a `reader` only choose the greater branch.
    fn merge(&mut self, other: Self)
    where Self: sealed::Seal {
        let mut res = Self::default();

        for my_maximal in self.maximal_times() {
            if !other.greater_equal(&my_maximal) {
                res.do_merge(self.visible(my_maximal));
            }
        }

        for other_maximal in other.maximal_times() {
            if !self.greater_equal(&other_maximal) {
                res.do_merge(other.visible(other_maximal));
            }
        }

        *self = res;
    }

    /// Check if a [`History`] is greater or equal to a given time.
    ///
    /// In other word, if there is a [`Time`] in this history that is greater or
    /// equal the given time.
    fn greater_equal(&self, t: &T::Time) -> bool {
        for max_time in self.maximal_times() {
            if max_time.greater_equal(t) {
                return true;
            }
        }
        false
    }

    fn do_merge(&mut self, other: Self);
}

mod sealed {
    pub trait Seal {}
    impl<T> Seal for T {}
}
