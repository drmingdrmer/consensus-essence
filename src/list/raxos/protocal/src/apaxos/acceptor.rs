use std::collections::HashSet;
use std::error::Error;
use std::fmt::Debug;

use validit::Validate;

use crate::apaxos::greater_equal::GreaterEqual;
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
    /// Returns the `Time` before handling the request and the updated
    /// [`Acceptor`] itself.
    ///
    /// The returned `Time` will be used to revert the `Time` if the
    /// [`Proposer`] decide to cancel this round of consensus algorithm.
    /// For example, **2PC** will revert the `Time` if the coordinator receives
    /// conflicting votes(otherwise other [`Proposer`] can not proceed). But
    /// **Classic Paxos** does not have to revert the `Time` but it could.
    pub(crate) fn handle_phase1_request(&mut self, commit_time: T::Time) -> (T::Time, T::History) {
        if self.is_committable(&commit_time) {
            return (commit_time, self.history.visible(commit_time));
        }

        self.forbidden_commit_time.insert(commit_time);
        (commit_time, self.history.visible(commit_time))
    }

    pub(crate) fn handle_phase2_request(&mut self, history: T::History) -> bool {
        dbg!("handle_phase2_request", &history);

        {
            let mut maximals = history.maximal_times();
            let new_written_time = maximals.next().unwrap();

            assert!(
                maximals.next().is_none(),
                "A proposer commit a history reachable from only one single time"
            );

            if self.is_committable(&new_written_time) {
                return false;
            }
        }

        self.history.merge(history);

        true
    }

    /// Check it is allowed to commit at the specified time.
    fn is_committable(&self, time: &T::Time) -> bool {
        for t in &self.forbidden_commit_time {
            if t.is_gt(time) {
                return false;
            }
        }

        true
    }
}