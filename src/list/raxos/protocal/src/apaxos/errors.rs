use std::collections::BTreeMap;

use crate::Types;

#[derive(thiserror::Error)]
pub enum APError<T: Types> {
    ReadQuorumNotReached {
        accepted: BTreeMap<T::AcceptorId, ()>,
    },
    WriteQuorumNotReached {
        accepted: BTreeMap<T::AcceptorId, ()>,
    },
}
