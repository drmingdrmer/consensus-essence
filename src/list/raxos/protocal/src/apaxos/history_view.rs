use crate::apaxos::decided::Decided;
use crate::Types;

// For doc reference
#[rustfmt::skip]
#[allow(unused_imports)]
use crate::apaxos::{
    history::History,
    proposer::Proposer
};

/// Represents a snapshot view of a [`History`] up to a specific time.
///
/// It represents a subset of the history that includes all time and events
/// **before or equal** a specific "time".
///
/// This trait is used by a [`Proposer`] to represent the system state it sees.
#[derive(Debug, Clone, Default)]
pub struct HistoryView<T>
where T: Types
{
    current_time: T::Time,
    history: T::History,
}

impl<T> HistoryView<T>
where T: Types
{
    pub fn new(current_time: T::Time, history: T::History) -> Self {
        Self {
            current_time,
            history,
        }
    }

    /// Returns the "current" time of this snapshot view.
    ///
    /// This time represents the **greatest** single time.
    /// All events in the snapshot are causally prior to or concurrent with this
    /// time.
    ///
    /// Note: The current time does not necessarily have to be an actual event
    /// time present in this History. It can be any valid time that defines
    /// the causal "cut" for this snapshot view.
    pub fn current_time(&self) -> T::Time {
        self.current_time
    }

    /// Attempts to append an [`Event`] at the current time.
    ///
    /// The updated instance should include all events from this view, plus the
    /// new event at the current view time.
    ///
    /// This method should return an `Err` if there is already an [`Event`] at
    /// the ([`current_time()`](Self::current_time)).
    pub fn append(mut self, event: T::Event) -> Result<Decided<T>, Decided<T>> {
        if let Some(_ev) = self.history.get(&self.current_time) {
            //
        } else {
            self.history.append(self.current_time, event).unwrap();
        }

        Ok(Decided::new(self.current_time, self.into_history()))
    }

    pub fn into_history(self) -> T::History {
        self.history
    }
}
