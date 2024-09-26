use crate::apaxos::decided::Decided;
use crate::apaxos::history::History;
use crate::apaxos::history_view::HistoryView;
use crate::Types;

#[derive(Debug, Clone, Default)]
pub struct BasicView<T>
where T: Types
{
    current_time: T::Time,
    history: T::History,
}

impl<T> BasicView<T>
where T: Types
{
    pub fn new(current_time: T::Time, history: T::History) -> Self {
        Self {
            current_time,
            history,
        }
    }
}

impl<T> HistoryView<T> for BasicView<T>
where T: Types
{
    fn current_time(&self) -> T::Time {
        self.current_time
    }

    fn append(mut self, event: T::Event) -> Result<Decided<T>, Decided<T>> {
        if let Some(_ev) = self.history.get(&self.current_time) {
            //
        } else {
            self.history.append(self.current_time, event).unwrap();
        }

        Ok(Decided::new(self.current_time, self.into_history()))
    }

    fn into_history(self) -> T::History {
        self.history
    }
}
