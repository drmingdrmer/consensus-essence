use crate::apaxos::history::History;
use crate::apaxos::history_view::HistoryView;
use crate::Types;

#[derive(Debug, Clone, Default)]
pub struct Decided<T>
where T: Types
{
    current_time: T::Time,
    history: T::History,
}

impl<T> Decided<T>
where T: Types
{
    pub fn new(current_time: T::Time, history: T::History) -> Self {
        debug_assert!(history.get(&current_time).is_some());
        Self {
            current_time,
            history,
        }
    }

    pub fn current_time(&self) -> T::Time {
        self.current_time
    }

    pub fn into_history(self) -> T::History {
        self.history
    }
}
