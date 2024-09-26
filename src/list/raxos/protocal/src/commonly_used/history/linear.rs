use std::collections::BTreeMap;

use crate::apaxos::history::History;
use crate::Types;

#[derive(Clone, Debug)]
struct LinearHistory<T: Types> {
    time_events: BTreeMap<T::Time, T::Event>,
}

impl<T: Types> Default for LinearHistory<T> {
    fn default() -> Self {
        Self {
            time_events: BTreeMap::new(),
        }
    }
}

/// Linear history requires the Time to be totally ordered.
impl<T> History<T> for LinearHistory<T>
where
    T: Types<History = Self>,
    T::Time: Ord,
{
    fn do_append(&mut self, time: T::Time, event: T::Event) {
        self.time_events.insert(time, event);
    }

    fn get(&self, time: &T::Time) -> Option<&T::Event> {
        self.time_events.get(&time)
    }

    fn lower_bounds(&self, time: T::Time) -> Self {
        let time_events =
            self.time_events.clone().into_iter().take_while(|(t, _)| t <= &time).collect();
        Self { time_events }
    }

    fn maximals(&self) -> impl Iterator<Item = (T::Time, T::Event)> {
        let last = self.time_events.iter().last();
        last.into_iter().map(|(t, e)| (*t, e.clone()))
    }

    fn do_merge(&mut self, other: Self) {
        // linear history just be replaced by the other when merging.
        *self = other;
    }
}
