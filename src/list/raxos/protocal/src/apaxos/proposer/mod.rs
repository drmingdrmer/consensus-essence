use phase1::Phase1;
use phase2::Phase2;

use crate::apaxos::errors::APError;
use crate::apaxos::focal_history::FocalHistory;
use crate::APaxos;
use crate::Types;

mod phase1;
mod phase2;

/// Proposer proposes a value and tries to commit it by getting it accepted by a
/// quorum of Acceptors. It does so by running Phase1 and Phase2.
pub struct Proposer<'a, T: Types> {
    apaxos: &'a mut APaxos<T>,
    time: T::Time,
    event: T::Event,
}

impl<'a, T: Types> Proposer<'a, T> {
    /// Create an instance of [`APaxos`] that tries to commit `value` at `time`
    /// to the distributed system.
    pub fn new(apaxos: &'a mut APaxos<T>, time: T::Time, event: T::Event) -> Self {
        Self {
            apaxos,
            time,
            event,
        }
    }

    pub fn run(&mut self) -> Result<T::History, APError<T>> {
        let maybe_committed = self.new_phase1().run()?;
        let committed = self.new_phase2(maybe_committed).run()?;

        Ok(committed)
    }

    fn new_phase1(&mut self) -> Phase1<T> {
        Phase1 {
            apaxos: &mut self.apaxos,
            time: self.time,
            granted: Default::default(),
            previously_accepted: Default::default(),
        }
    }

    fn new_phase2(&mut self, maybe_committed: FocalHistory<T>) -> Phase2<T> {
        // If the current time already has an event, no new event is added.

        let decided = match maybe_committed.add_event(self.event.clone()) {
            Ok(decided) => {
                // new event is added
                decided
            }
            Err(old) => {
                // new event is not added.
                old
            }
        };

        Phase2 {
            apaxos: &mut self.apaxos,
            time: self.time,
            decided,
            accepted: Default::default(),
        }
    }
}
