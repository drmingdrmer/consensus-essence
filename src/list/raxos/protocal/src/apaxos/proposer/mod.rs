use phase1::Phase1;
use phase2::Phase2;

use crate::apaxos::proposal::Proposal;
use crate::APaxos;
use crate::Types;

mod phase1;
mod phase2;

/// Proposer proposes a value and tries to commit it by getting it accepted by a
/// quorum of Acceptors. It does so by running Phase1 and Phase2.
pub struct Proposer<'a, T: Types> {
    apaxos: &'a mut APaxos<T>,
    time: T::Time,
    proposal: Proposal<T, T::Event>,
}

impl<'a, T: Types> Proposer<'a, T> {
    /// Create an instance of [`APaxos`] that tries to commit `value` at `time`
    /// to the distributed system.
    pub fn new(apaxos: &'a mut APaxos<T>, time: T::Time, value: T::Event) -> Self {
        Self {
            apaxos,
            time,
            proposal: Proposal::new(time, value),
        }
    }

    pub fn run(&mut self) -> Proposal<T, T::Event> {
        let maybe_committed = self.new_phase1().run();
        let committed = self.new_phase2(maybe_committed).run();

        committed
    }

    // TODO: phase-1-revert
    fn new_phase1(&mut self) -> Phase1<T> {
        Phase1 {
            apaxos: &mut self.apaxos,
            time: self.time,
            granted: Default::default(),
            previously_accepted: Default::default(),
        }
    }

    fn new_phase2(&mut self, maybe_committed: Option<Proposal<T, T::Event>>) -> Phase2<T> {
        Phase2 {
            apaxos: &mut self.apaxos,
            time: self.time,
            decided: maybe_committed.unwrap_or_else(|| self.proposal.clone()),
            granted: Default::default(),
        }
    }
}
