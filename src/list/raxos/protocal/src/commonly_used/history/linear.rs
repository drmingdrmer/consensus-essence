use std::collections::BTreeMap;

use crate::apaxos::history::History;
use crate::Types;

#[derive(Clone, Debug)]
struct LinearHistory<T: Types> {
    history: BTreeMap<T::Time, T::Event>,
}

impl<T: Types> Default for LinearHistory<T> {
    fn default() -> Self {
        Self {
            history: BTreeMap::new(),
        }
    }
}

/// Linear history requires the Time to be totally ordered.
impl<T: Types> History<T> for LinearHistory<T>
where T::Time: Ord
{
    fn append(&mut self, time: T::Time, event: T::Event) {
        todo!()
    }

    fn get(&self, time: &T::Time) -> Option<&T::Event> {
        self.history.get(&time)
    }

    fn visible(&self, time: T::Time) -> Self {
        let history = self.history.clone().into_iter().take_while(|(t, _)| t <= &time).collect();
        Self { history }
    }

    fn maximals(&self) -> impl Iterator<Item = (T::Time, T::Event)> {
        let last = self.history.iter().last();
        last.into_iter().map(|(t, e)| (*t, e.clone()))
    }

    fn do_merge(&mut self, other: Self) {
        // linear history just be replaced by the other when merging.
        *self = other;
    }
}
