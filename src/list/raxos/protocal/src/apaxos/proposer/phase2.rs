use std::collections::BTreeMap;

use crate::apaxos::errors::APError;
use crate::APaxos;
use crate::QuorumSet;
use crate::Transport;
use crate::Types;

pub struct Phase2<'a, T: Types> {
    pub apaxos: &'a mut APaxos<T>,

    /// The time of the Proposer that running phase1.
    pub time: T::Time,

    pub decided: T::History,

    pub accepted: BTreeMap<T::AcceptorId, ()>,
}

impl<'a, T: Types> Phase2<'a, T> {
    pub fn run(mut self) -> Result<T::History, APError<T>> {
        let apaxos = &mut self.apaxos;

        let mut sent = 0;

        let acceptor_ids = apaxos.acceptors.keys();

        for id in acceptor_ids {
            apaxos.transport.send_phase2_request(*id, self.decided.clone());
            sent += 1;
        }

        for _ in 0..sent {
            let (target, is_accepted) = apaxos.transport.recv_phase2_reply();
            if is_accepted {
                self.accepted.insert(target, ());
            }

            if apaxos.quorum_set.is_write_quorum(self.accepted.keys().cloned()) {
                return Ok(self.decided);
            }
        }

        Err(APError::WriteQuorumNotReached {
            accepted: self.accepted,
        })
    }
}
