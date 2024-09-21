#![feature(associated_type_defaults)]
#![feature(map_try_insert)]

pub mod apaxos;
pub mod commonly_used;
pub mod implementations;

use std::collections::BTreeMap;
use std::fmt::Debug;

use apaxos::proposal::Proposal;
use apaxos::ptime::Time;

use crate::apaxos::acceptor::Acceptor;
use crate::apaxos::greater_equal_map::Map;
use crate::apaxos::history::History;

pub trait AcceptorId: Debug + Clone + Copy + Ord + 'static {}

pub trait Value: Debug + Clone + 'static {}

/// Defines types that are used in the Abstract-Paxos algorithm.
pub trait Types
where
    Self: Default + Debug + Clone + Sized + 'static,
{
    /// Acceptor ID
    type AcceptorId: AcceptorId = u64;

    /// Pseudo time used in a distributed consensus.
    ///
    /// Every distributed consensus algorithm has its own definition of time.
    /// - In Paxos, it is ballot number, which is `(round, proposer_id)`.
    /// - In Raft, it is `(term, Option<voted_for>)`.
    /// - In 2PC, it is mainly a vector of related data entry name.
    // TODO: explain 2pc time.
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

    /// The distribution algorithm for distributing a value to several acceptors
    /// and for rebuilding the value from accepted value parts.
    type Distribute: Distribute<Self>;
}

pub trait Transport<T: Types> {
    fn send_phase1_request(&mut self, target: T::AcceptorId, t: T::Time);
    fn recv_phase1_reply(&mut self) -> (T::AcceptorId, (T::Time, Acceptor<T>));

    fn send_phase2_request(
        &mut self,
        target: T::AcceptorId,
        t: T::Time,
        proposal: Proposal<T, T::Part>,
    );
    fn recv_phase2_reply(&mut self) -> (T::AcceptorId, bool);
}

/// Defines the distribution policy for storing portions of a value on several
/// Acceptor-s.
///
/// This trait is responsible to split the [`Proposal`] into several `Part`s,
/// each part for every Acceptor, and to rebuild a [`Proposal`] from `Part`s
pub trait Distribute<T: Types> {
    /// Distribute a value to several [`Acceptor`]s;
    fn distribute<'a>(
        &mut self,
        value: T::Event,
        acceptor_ids: impl IntoIterator<Item=&'a T::AcceptorId>,
    ) -> Vec<T::Part>;

    /// `rebuild` is the reverse operation of `distribute`:
    /// It rebuilds a value from parts that are accepted by [`Acceptor`]s.
    fn rebuild<'a>(
        &mut self,
        x: impl IntoIterator<Item=(&'a T::AcceptorId, &'a T::Part)>,
    ) -> Option<T::Event>;
}

pub trait QuorumSet<T: Types> {
    fn is_read_quorum(&self, acceptor_ids: impl IntoIterator<Item=T::AcceptorId>) -> bool;
    fn is_write_quorum(&self, acceptor_ids: impl IntoIterator<Item=T::AcceptorId>) -> bool;
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

    /// Defines how to distribute a parts of a value to several [`Acceptor`]
    distribute: T::Distribute,

    /// Transport for sending and receiving messages.
    transport: T::Transport,
}

impl<T: Types> APaxos<T> {
    pub fn new(
        acceptors: impl IntoIterator<Item=T::AcceptorId>,
        quorum_set: T::QuorumSet,
        distribute: T::Distribute,
        transport: T::Transport,
    ) -> Self {
        let acceptors = acceptors.into_iter().map(|id| (id, ())).collect();

        Self {
            acceptors,
            quorum_set,
            distribute,
            transport,
        }
    }
}
