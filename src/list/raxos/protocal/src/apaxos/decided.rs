use crate::apaxos::focal_history::FocalHistory;
use crate::apaxos::focal_history::WITH_CURRENT_EVENT;

pub type Decided<T> = FocalHistory<T, { WITH_CURRENT_EVENT }>;
