use crate::apaxos::branch::Branch;
use crate::apaxos::branch::HEAD_SET;

pub type Decided<T> = Branch<T, { HEAD_SET }>;
