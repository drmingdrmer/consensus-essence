use std::collections::HashSet;
use std::error::Error;
use std::fmt::Debug;

use validit::Validate;

use crate::apaxos::decided::Decided;
use crate::apaxos::focal_history::FocalHistory;
use crate::apaxos::history::History;
use crate::Types;

#[derive(Clone, Default, Debug)]
pub struct Acceptor<T: Types> {
    /// A Time that is smaller than any one of these time
    /// is not allow to commit.
    pub forbidden_commit_time: HashSet<T::Time>,

    pub history: T::History,
}

impl<T: Types> Validate for Acceptor<T> {
    fn validate(&self) -> Result<(), Box<dyn Error>> {
        Ok(())
    }
}

impl<T: Types> Acceptor<T> {
    /// Handle the phase-1 request from a [`Proposer`], i.e., set up a new
    /// [`Time`] point.
    ///
    /// Returns a history view for appending new event if it is allowed to
    /// commit. Otherwise, returns the [`Time`] that disables this proposal.
    ///
    /// The returned `Time` will be used to revert the `Time` if the
    /// [`Proposer`] decide to cancel this round of consensus algorithm.
    /// For example, **2PC** will revert the `Time` if the coordinator receives
    /// conflicting votes(otherwise other [`Proposer`] can not proceed). But
    /// **Classic Paxos** does not have to revert the `Time` but it could.
    pub(crate) fn handle_phase1_request(
        &mut self,
        commit_time: T::Time,
    ) -> Result<FocalHistory<T>, T::Time> {
        self.check_committable(&commit_time)?;

        self.forbid_smaller_commit_time(commit_time);

        Ok(self.history.history_view(commit_time))
    }

    pub(crate) fn handle_phase2_request(&mut self, decided: Decided<T>) -> Result<(), T::Time> {
        dbg!("handle_phase2_request", &decided);

        let new_written_time = decided.head_time();
        self.check_committable(&new_written_time)?;

        self.history.merge(decided.into_history());

        Ok(())
    }

    fn forbid_smaller_commit_time(&mut self, time: T::Time) {
        self.forbidden_commit_time.insert(time);

        for t in self.forbidden_commit_time.clone().iter() {
            if t < &time {
                self.forbidden_commit_time.remove(t);
            }
        }
    }

    /// Check it is allowed to commit at the specified time.
    fn check_committable(&self, time: &T::Time) -> Result<(), T::Time> {
        for t in &self.forbidden_commit_time {
            if t > time {
                return Err(*t);
            }
        }

        Ok(())
    }
}
