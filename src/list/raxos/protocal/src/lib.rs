#![feature(associated_type_defaults)]
#![feature(map_try_insert)]

pub mod apaxos;
pub mod commonly_used;
pub mod implementations;

use std::collections::BTreeMap;
use std::fmt::Debug;

use apaxos::ptime::Time;

use crate::apaxos::branch::Branch;
use crate::apaxos::branch::HEAD_UNDECIDED;
use crate::apaxos::decided::Decided;
use crate::apaxos::history::History;

pub trait AcceptorId: Debug + Clone + Copy + Ord + Send + 'static {}

pub trait Value: Debug + Clone + Send + 'static {}

/// Defines types that are used in the Abstract-Paxos algorithm.
pub trait Types
where Self: Default + Debug + Clone + Sized + 'static
{
    /// Acceptor ID
    type AcceptorId: AcceptorId = u64;

    /// Pseudo time used in a distributed consensus.
    ///
    /// Every distributed consensus algorithm has its own definition of time.
    /// - In Paxos, it is ballot number, which is `(round, proposer_id)`.
    /// - In Raft, it is `(term, Option<voted_for>)`.
    /// - In 2PC, it is mainly a vector of related data entry name.
    type Time: Time;

    /// The value to propose and to commit
    type Event: Value;

    type History: History<Self>;

    /// Quorum set defines quorums for read and write.
    ///
    /// Read-quorum is used by phase-1, write-quorum is used by phase-2.
    /// In most cases, read-quorum and write-quorum are the same.
    ///
    /// A quorum set defines the cluster structure.
    // TODO: explain cluster structure
    type QuorumSet: QuorumSet<Self>;

    /// The network transport for sending and receiving messages.
    type Transport: Transport<Self>;
}

#[async_trait::async_trait]
pub trait Transport<T: Types> {
    fn send_phase1_request(&mut self, target: T::AcceptorId, t: T::Time);

    async fn recv_phase1_reply(
        &mut self,
    ) -> (
        T::AcceptorId,
        Result<Branch<T, { HEAD_UNDECIDED }>, T::Time>,
    );

    fn send_phase2_request(&mut self, target: T::AcceptorId, decided: Decided<T>);

    async fn recv_phase2_reply(&mut self) -> (T::AcceptorId, Result<(), T::Time>);
}

pub trait QuorumSet<T: Types> {
    fn is_read_quorum(&self, acceptor_ids: impl IntoIterator<Item = T::AcceptorId>) -> bool;
    fn is_write_quorum(&self, acceptor_ids: impl IntoIterator<Item = T::AcceptorId>) -> bool;
}

/// Abstract Paxos
pub struct APaxos<T: Types> {
    /// Acceptors stores value or part of the value that is proposed by a
    /// [`Proposer`].
    ///
    /// [`Proposer`]: crate::apaxos::proposer::Proposer
    acceptors: BTreeMap<T::AcceptorId, ()>,

    /// Quorum set defines quorums for read and write.
    ///
    /// A value that is accepted by a quorum is considered committed.
    quorum_set: T::QuorumSet,

    /// Transport for sending and receiving messages.
    transport: T::Transport,
}

impl<T: Types> APaxos<T> {
    pub fn new(
        acceptors: impl IntoIterator<Item = T::AcceptorId>,
        quorum_set: T::QuorumSet,
        transport: T::Transport,
    ) -> Self {
        let acceptors = acceptors.into_iter().map(|id| (id, ())).collect();

        Self {
            acceptors,
            quorum_set,
            transport,
        }
    }
}
