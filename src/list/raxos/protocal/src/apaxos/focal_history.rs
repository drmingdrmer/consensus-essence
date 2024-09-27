use crate::apaxos::decided::Decided;
use crate::Types;

// For doc reference
#[rustfmt::skip]
#[allow(unused_imports)]
use crate::apaxos::{
    history::History,
    proposer::Proposer
};

/// Indicate the [`FocalHistory`] includes an event has been set at the current
/// time.
pub const WITH_CURRENT_EVENT: bool = true;

/// Indicate the [`FocalHistory`] **may** not include an event at the current
/// time. This is because the current time is already in the history.
pub const WITHOUT_CURRENT_EVENT: bool = false;

pub type CurrentEventState = bool;

/// Represents a focused view of a [`History`] with a designated "current time".
///
/// It represents a subset of the history that includes all time and events
/// **before or equal** a specific "time".
///
/// Unlike a [`History`], which may contain multiple maximal times due to
/// the partial ordering of events, `FocalHistory` has a single, well-defined
/// "current time" that serves as the focal point of this historical view.
///
/// This struct is typically used by a [`Proposer`] to represent the
/// system state it observes at a given moment, with a clear notion
/// of what it considers to be the current time.
#[derive(Debug, Clone, Default)]
pub struct FocalHistory<T, const EVENT: CurrentEventState>
where T: Types
{
    head_time: T::Time,
    history: T::History,
}

impl<T> FocalHistory<T, WITHOUT_CURRENT_EVENT>
where T: Types
{
    pub fn new(current_time: T::Time, history: T::History) -> Self {
        Self {
            head_time: current_time,
            history,
        }
    }

    /// Attempts to add an [`Event`] at the **current time**, and create a
    /// [`Decided`] instance.
    ///
    /// The returned [`Decided`] instance include all events from this view,
    /// plus the new event at the **current time**.
    ///
    /// This method should return an `Err` if there is already an [`Event`] at
    /// the ([`current_time()`](Self::head_time)).
    pub fn add_event(
        mut self,
        event: T::Event,
    ) -> Result<FocalHistory<T, WITH_CURRENT_EVENT>, FocalHistory<T, WITH_CURRENT_EVENT>> {
        if let Some(_ev) = self.history.get(&self.head_time) {
            // Nothing to do
        } else {
            self.history.append(self.head_time, event).unwrap();
        }

        Ok(FocalHistory::<T, WITH_CURRENT_EVENT> {
            head_time: self.head_time,
            history: self.history,
        })
    }
}

impl<T, const EVENT: CurrentEventState> FocalHistory<T, EVENT>
where T: Types
{
    /// Returns the "current" time of this snapshot view.
    ///
    /// This time represents the **greatest** single time.
    /// All events in the view are causally prior to or equal this time.
    ///
    /// Note: The current time does not necessarily have to be an actual event
    /// time present in this History. It can be any valid time that defines
    /// the causal "cut" for this snapshot view.
    pub fn head_time(&self) -> T::Time {
        self.head_time
    }

    pub fn into_history(self) -> T::History {
        self.history
    }
}
