//! Implement classic-paxos with abstract-paxos
//!
//! - [`Time`] in paxos is ballot-number, a monotonic incremental integer.
//! - [`QuorumSet`] is a simple [`Majority`].
//! - Network [`Transport`] is implemented with direct function call to an
//!   [`Acceptor`].
//! - To rebuild a **maybe committed** value with [`Distribute`], it just use
//!   the one with max `v_ballot`.

use crate::commonly_used::history::linear::LinearHistory;
use crate::commonly_used::history::linear::SINGLE_VALUE;
use crate::commonly_used::quorum_set::majority::Majority;
use crate::commonly_used::transport::DirectCall;
use crate::Types;

/// Implement classic-paxos with abstract-paxos
#[derive(Debug, Clone, Default)]
struct Paxos {}

impl Types for Paxos {
    type Time = u64;
    type Event = String;
    type History = LinearHistory<Self, { SINGLE_VALUE }>;
    type QuorumSet = Majority<Self>;
    type Transport = DirectCall<Self>;
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use crate::apaxos::acceptor::Acceptor;
    use crate::apaxos::proposer::Proposer;
    use crate::commonly_used::quorum_set::majority::Majority;
    use crate::commonly_used::transport::DirectCall;
    use crate::implementations::paxos::Paxos;
    use crate::APaxos;

    #[test]
    fn test_paxos() -> Result<(), Box<dyn std::error::Error>> {
        //

        let acceptor_ids = [1, 2, 3];

        let mut acceptors = BTreeMap::new();
        for id in acceptor_ids {
            acceptors.insert(id, Acceptor::default());
        }

        let quorum_set = Majority::new(acceptor_ids);
        let transport = DirectCall::new(acceptors.clone());

        let mut apaxos = APaxos::<Paxos>::new(acceptor_ids, quorum_set, transport);

        let mut proposer = Proposer::new(&mut apaxos, 5, "hello".to_string());
        let committed = proposer.run()?;

        assert_eq!(committed.latest_time(), Some(5));
        assert_eq!(committed.latest_value(), Some(s("hello")));

        let mut proposer = Proposer::new(&mut apaxos, 6, "world".to_string());
        let committed = proposer.run()?;

        assert_eq!(committed.latest_time(), Some(6));
        assert_eq!(committed.latest_value(), Some(s("hello")));

        Ok(())

        // TODO: rebuild from previous value
    }

    fn s(s: impl ToString) -> String {
        s.to_string()
    }
}
