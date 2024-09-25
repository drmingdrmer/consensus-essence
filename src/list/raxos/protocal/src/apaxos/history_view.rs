use crate::apaxos::history::History;
use crate::Types;

/// Represents a snapshot view of a [`History`] up to a specific time.
///
/// This trait extends the [`History`] trait to represent a subset of the
/// history that includes all time and events **before or equal** a specific
/// "view time".
///
/// This trait is used by a [`Proposer`] to represent the system state it sees.
pub trait HistoryView<T: Types, H>
where H: History<T>
{
    /// Returns the "current" time of this snapshot view.
    ///
    /// This time represents the **greatest** single time.
    /// All events in the snapshot are causally prior to or concurrent with this
    /// time.
    ///
    /// Note: The current time does not necessarily have to be an actual event
    /// time present in this History. It can be any valid time that defines
    /// the causal "cut" for this snapshot view.
    fn current_time(&self) -> T::Time;

    /// Attempts to append an [`Event`] at the current time to create a new
    /// [`History`].
    ///
    /// The created new [`History`] instance that includes all
    /// events from this view, plus the new event at the current view time.
    ///
    /// This method should return an `Err` if there is already an [`Event`] at
    /// the ([`current_time()`](Self::current_time)).
    fn append(self, event: T::Event) -> Result<H, H>;
}
