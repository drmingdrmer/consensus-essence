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

impl<T: Types> History<T> for OneSlotHistory<T> {
    fn append(&mut self, time: T::Time, event: T::Event) {
        todo!()
    }

    fn get(&self, time: &T::Time) -> Option<&T::Event> {
        todo!()
    }

    fn visible(&self, time: T::Time) -> Self {
        todo!()
    }

    fn maximals(&self) -> impl Iterator<Item = (T::Time, T::Event)> {
        self.history.clone().into_iter()
    }

    fn do_merge(&mut self, other: Self) {
        todo!()
    }
}
