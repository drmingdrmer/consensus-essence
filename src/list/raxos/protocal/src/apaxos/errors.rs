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
}
