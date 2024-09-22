use std::collections::BTreeMap;

use crate::apaxos::errors::APError;
use crate::apaxos::history::History;
use crate::APaxos;
use crate::QuorumSet;
use crate::Transport;
use crate::Types;

pub struct Phase1<'a, T: Types> {
    pub apaxos: &'a mut APaxos<T>,

    /// The time of the [`Proposer`] that running [`Phase1`].
    pub time: T::Time,

    /// The set of acceptors that granted the [`Proposer`]'s [`Phase1`] request.
    ///
    /// The value is the `previous` [`Time`] before an acceptor grants the
    /// [`Proposer`]'s phase-1 request.
    ///
    /// These `previous` [`Time`]s are used to revert the [`Acceptor`]'s time.
    pub granted: BTreeMap<T::AcceptorId, ()>,

    /// The value part that the acceptor has accepted.
    ///
    /// These value parts are proposed by smaller [`Proposer`]s.
    pub previously_accepted: T::History,
}

impl<'a, T: Types> Phase1<'a, T> {
    pub fn run(mut self) -> Result<T::History, APError<T>> {
        let apaxos = &mut self.apaxos;

        let mut sent = 0;

        for id in apaxos.acceptors.keys() {
            apaxos.transport.send_phase1_request(*id, self.time);
            sent += 1;
        }

        for _ in 0..sent {
            let (target, (greater_equal_time, history)) = self.apaxos.transport.recv_phase1_reply();
            dbg!("received phase-1 reply", &target, &history);
            if greater_equal_time != self.time {
                // Phase-1 request is rejected.
                continue;
            }

            self.granted.insert(target, ());
            self.previously_accepted.merge(history);

            let is_read_quorum =
                self.apaxos.quorum_set.is_read_quorum(self.granted.keys().copied());

            if is_read_quorum {
                return Ok(self.previously_accepted);
            }
        }

        Err(APError::ReadQuorumNotReached {
            accepted: self.granted,
        })
    }
}