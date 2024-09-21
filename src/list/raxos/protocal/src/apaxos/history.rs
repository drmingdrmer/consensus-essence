use crate::Types;

pub struct TimeEvent<T: Types> {
    time: T::Time,
    event: T::Event,
}

pub trait History<T: Types>: Default {
    fn append(&mut self, time_event: TimeEvent<T>);
    fn get(&self, time: T::Time) -> Option<&T::Event>;


    /// Return a sub set of the history that is visible at `time`.
    ///
    /// In other words, a sub set of TimeEvent that is less than or equal to `time`.
    fn visible(&self, time: T::Time) -> Self;

    /// Return the maximal [`TimeEvent`] in the history.
    ///
    /// `maximal` is defined as:
    /// g in P is a maximal element:
    /// if there is no element a in P such that a > g
    fn maximals(&self) -> Vec<&TimeEvent<T>>;

    fn merge(&mut self, other: Self);
}