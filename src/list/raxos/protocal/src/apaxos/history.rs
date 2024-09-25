use std::fmt::Debug;

use crate::apaxos::greater_equal::GreaterEqual;
use crate::apaxos::history_view::HistoryView;
use crate::commonly_used::history_view::BasicView;
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

/// A [`History`] contains [`Time`] and [`Event`] in a Partially Ordered Set.
///
/// [`History`] is used by an [`Acceptor`] to store the [`Time`] and [`Event`]
/// pairs.
/// It represents the causal history of events in a distributed system.
pub trait History<T: Types>
where Self: Default + Debug + Clone
{
    /// The type representing a view of this History.
    ///
    /// Defaults to `BasicView<T, Self>` but can be overridden by implementors.
    type View: HistoryView<T, Self> = BasicView<T, Self>;

    fn get(&self, time: &T::Time) -> Option<&T::Event>;

    /// Returns a view(subset) of the history that is causally prior to or
    /// concurrent with the given `time`.
    fn history_view(&self, time: T::Time) -> Self::View;

    // fn lower_bounds(&self, time: T::Time) -> Self;

    /// Return an iterator over the maximal [`Time`] and [`Event`] pairs in the
    /// history.
    ///
    /// `maximal` is defined as:
    /// g in P is a maximal element:
    /// if there is no element a in P such that a > g
    ///
    /// All `maximal` have no order between them.
    fn maximals(&self) -> impl Iterator<Item = (T::Time, T::Event)>;

    fn do_merge(&mut self, other: Self);

    fn maximal_times<'a>(&'a self) -> impl Iterator<Item = T::Time> + 'a
    where Self: sealed::Seal {
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
                res.do_merge(self.history_view(my_maximal));
            }
        }

        for other_maximal in other.maximal_times() {
            if !self.greater_equal(&other_maximal) {
                res.do_merge(other.history_view(other_maximal));
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
}

mod sealed {
    pub trait Seal {}
    impl<T> Seal for T {}
}
