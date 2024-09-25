use crate::apaxos::history::History;
use crate::Types;

#[derive(Clone, Debug)]
pub struct OneSlotHistory<T: Types> {
    history: Option<(T::Time, T::Event)>,
}

impl<T: Types> Default for OneSlotHistory<T> {
    fn default() -> Self {
        todo!()
    }
}

impl<T: Types> OneSlotHistory<T> {
    pub fn value_time(&self) -> Option<T::Time> {
        self.history.clone().map(|(t, _)| t)
    }

    pub fn value(&self) -> Option<&T::Event> {
        self.history.as_ref().map(|(_, e)| e)
    }
}

impl<T: Types> History<T> for OneSlotHistory<T> {
    fn get(&self, time: &T::Time) -> Option<&T::Event> {
        todo!()
    }

    fn history_view(&self, time: T::Time) -> Self::View {
        todo!()
    }

    fn maximals(&self) -> impl Iterator<Item = (T::Time, T::Event)> {
        self.history.clone().into_iter()
    }

    fn do_merge(&mut self, other: Self) {
        todo!()
    }
}
