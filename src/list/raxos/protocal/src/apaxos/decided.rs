use crate::apaxos::branch::Branch;
use crate::apaxos::branch::HEAD_DECIDED;

pub type Decided<T> = Branch<T, { HEAD_DECIDED }>;
