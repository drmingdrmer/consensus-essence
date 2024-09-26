use std::collections::BTreeMap;

use crate::Types;

#[derive(Debug)]
#[derive(thiserror::Error)]
pub enum APError<T: Types> {
    #[error("Read quorum not reached: {accepted:?}")]
    ReadQuorumNotReached {
        accepted: BTreeMap<T::AcceptorId, ()>,
    },

    #[error("Write quorum not reached: {accepted:?}")]
    WriteQuorumNotReached {
        accepted: BTreeMap<T::AcceptorId, ()>,
    },

    #[error(transparent)]
    TimeRegression(TimeRegression<T>),
}

#[derive(Debug)]
#[derive(thiserror::Error)]
#[error("Cannot update to an earlier time. Attempted: {attempted:?}, Existing: {existing:?}")]
pub struct TimeRegression<T: Types> {
    /// The time that was attempted to be set
    attempted: T::Time,
    /// The existing time that is later than the attempted time
    existing: T::Time,
}

impl<T: Types> TimeRegression<T> {
    pub fn new(attempted: T::Time, existing: T::Time) -> Self {
        Self {
            attempted,
            existing,
        }
    }
}
