use crate::apaxos::decided::Decided;
use crate::Types;

// For doc reference
#[rustfmt::skip]
#[allow(unused_imports)]
use crate::apaxos::{
    history::History,
    proposer::Proposer
};

/// Indicate the [`Branch`] includes an event has been set at the head
/// time.
pub const HEAD_SET: bool = true;

/// Indicate the [`Branch`] **may** not include an event at the head
/// time. Note that the head time is already in the internal history.
pub const HEAD_UNSET: bool = false;

pub type HeadEventState = bool;

/// Represents a single branch [`History`] that is reachable upto a designated
/// "head" time.
///
/// It represents a subset of the history that includes all time and events
/// **before or equal** a specific "time".
///
/// Unlike a [`History`], which may contain multiple maximal times due to
/// the partial ordering of events, [`Branch`] has a single, well-defined
/// "head time" that serves as the focal point of this historical view.
///
/// This struct is used by a [`Proposer`] to represent the system state it
/// observes at a given moment,
/// with a clear notion of what it considers to be the head time.
#[derive(Debug, Clone, Default)]
pub struct Branch<T, const HEAD: HeadEventState>
where T: Types
{
    head_time: T::Time,
    history: T::History,
}

impl<T> Branch<T, HEAD_UNSET>
where T: Types
{
    pub fn new(head_time: T::Time, history: T::History) -> Self {
        Self { head_time, history }
    }

    /// Attempts to add an [`Event`] at the **head time**, and create a
    /// [`Decided`] instance.
    ///
    /// The returned [`Decided`] instance include all events from this view,
    /// plus the new event at the **head time**.
    ///
    /// This method should return an `Err` if there is already an [`Event`] at
    /// the ([`Self::head_time()`]).
    pub fn add_event(
        mut self,
        event: T::Event,
    ) -> Result<Branch<T, HEAD_SET>, Branch<T, HEAD_SET>> {
        if let Some(_ev) = self.history.get(&self.head_time) {
            // Nothing to do
        } else {
            self.history.append(self.head_time, event).unwrap();
        }

        Ok(Branch::<T, HEAD_SET> {
            head_time: self.head_time,
            history: self.history,
        })
    }
}

impl<T, const HEAD: HeadEventState> Branch<T, HEAD>
where T: Types
{
    /// Returns the head time of this branch.
    ///
    /// This time represents the **greatest** single time.
    /// All events in the view are causally prior to or equal this time.
    ///
    /// The head time is the **now**, new event can only be added at **now**,
    /// not at a time in the past.
    ///
    /// Note: The head time does not necessarily have to be an actual event
    /// time present in this History. It can be any valid time that defines
    /// the causal "cut" for this branch.
    pub fn head_time(&self) -> T::Time {
        self.head_time
    }

    pub fn into_history(self) -> T::History {
        self.history
    }
}
