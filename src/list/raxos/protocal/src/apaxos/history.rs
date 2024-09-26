use std::fmt::Debug;

use crate::apaxos::errors::TimeRegression;
use crate::apaxos::history_view::HistoryView;
use crate::Types;

/// A [`History`] contains [`Time`] and [`Event`] in a Partially Ordered Set.
///
/// [`History`] is used by an [`Acceptor`] to store the [`Time`] and [`Event`]
/// pairs.
/// It represents the causal history of events in a distributed system.
pub trait History<T>
where
    T: Types<History = Self>,
    Self: Default + Debug + Clone,
{
    /// Append a new [`Event`] at a specific [`Time`] in the history.
    ///
    /// The [`Time`] must not smaller than any of the
    /// [`maximal_times()`](Self::maximal_times).
    fn append(&mut self, time: T::Time, event: T::Event) -> Result<(), TimeRegression<T>> {
        for max_time in self.maximal_times() {
            if time < max_time {
                return Err(TimeRegression::new(time, max_time));
            }
        }

        self.do_append(time, event);

        Ok(())
    }

    fn do_append(&mut self, time: T::Time, event: T::Event);

    fn get(&self, time: &T::Time) -> Option<&T::Event>;

    /// Returns a view(subset) of the history that is causally prior to or
    /// concurrent with the given `time`.
    fn history_view(&self, time: T::Time) -> HistoryView<T> {
        let lower = self.lower_bounds(time);
        HistoryView::new(time, lower)
    }

    /// Return a new instance in which every [`Time`] in it is causally prior to
    /// the given `time`.
    fn lower_bounds(&self, time: T::Time) -> Self;

    /// Return an iterator over the maximal [`Time`] and [`Event`] pairs in the
    /// history.
    ///
    /// `maximal` is defined as:
    /// g in P is a maximal element:
    /// if there is no element a in P such that a > g
    ///
    /// All `maximal` have no order between them.
    fn maximals(&self) -> impl Iterator<Item = (T::Time, T::Event)>;

    /// Merge two [`History`]
    ///
    /// Note that if there are `maximal` that have an order, the smaller one
    /// will be removed. Because a `reader` only choose the greater branch.
    fn merge(&mut self, other: Self)
    where Self: sealed::Seal {
        let mut res = Self::default();

        for my_maximal in self.maximal_times() {
            if other.greater_equal(my_maximal) {
                continue;
            }
            res.do_merge(self.lower_bounds(my_maximal));
        }

        for other_maximal in other.maximal_times() {
            if self.greater_equal(other_maximal) {
                continue;
            }
            res.do_merge(other.lower_bounds(other_maximal));
        }

        *self = res;
    }

    fn do_merge(&mut self, other: Self);

    fn maximal_times<'a>(&'a self) -> impl Iterator<Item = T::Time> + 'a
    where Self: sealed::Seal {
        self.maximals().map(|(t, _)| t)
    }

    /// Check if a [`History`] is greater or equal to a given time.
    ///
    /// In other word, if there is a [`Time`] in this history that is greater or
    /// equal the given time.
    fn greater_equal(&self, t: T::Time) -> bool {
        for max_time in self.maximal_times() {
            if max_time >= t {
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
