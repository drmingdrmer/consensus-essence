use crate::apaxos::history::History;
use crate::apaxos::history_view::HistoryView;
use crate::Types;

pub struct BasicView<T, H>
where
    T: Types,
    H: History<T>,
{
    current_time: T::Time,
    history: H,
}

impl<T, H> HistoryView<T, H> for BasicView<T, H>
where
    T: Types,
    H: History<T>,
{
    fn current_time(&self) -> T::Time {
        self.current_time
    }

    fn append(self, event: T::Event) -> Result<H, H> {
        let mut history = self.history;
        history.do_merge(H::new().append(self.current_time, event));
        history
    }
}
