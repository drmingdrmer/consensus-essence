use std::collections::BTreeMap;
use std::collections::HashMap;

use crate::apaxos::history::History;
use crate::Types;

#[derive(Clone, Debug)]
pub struct GenericHistory<T>
where T: Types<History = Self>
{
    maximals: HashMap<T::Time, ()>,
    time_events: HashMap<T::Time, T::Event>,
}

impl<T> Default for GenericHistory<T>
where T: Types<History = Self>
{
    fn default() -> Self {
        Self {
            maximals: HashMap::new(),
            time_events: HashMap::new(),
        }
    }
}

impl<T: Types> History<T> for GenericHistory<T>
where T: Types<History = Self>
{
    fn do_append(&mut self, time: T::Time, event: T::Event) {
        self.time_events.insert(time, event);
        for max in self.maximals.keys().copied().collect::<Vec<_>>() {
            if max <= time {
                self.maximals.remove(&max);
            }
        }
    }

    fn get(&self, time: &T::Time) -> Option<&T::Event> {
        self.time_events.get(time)
    }

    fn lower_bounds(&self, time: T::Time) -> Self {
        let time_events = self
            .time_events
            .iter()
            .filter(|(t, _ev)| t <= &time)
            .map(|(t, ev)| (*t, ev.clone()))
            .collect::<HashMap<_, _>>();

        Self {
            maximals: build_maximals(&time_events),
            time_events,
        }
    }

    fn maximals(&self) -> impl Iterator<Item = (T::Time, T::Event)> {
        self.maximals.keys().copied().map(move |t| (t, self.time_events[&t].clone()))
    }

    fn do_merge(&mut self, other: Self) {
        for (time, event) in other.time_events {
            self.time_events.insert(time, event);
        }

        for (time, _) in other.maximals {
            self.maximals.insert(time, ());
        }
    }
}

/// Build a map of **maximal** times from a map of time events.
fn build_maximals<T: Types>(time_events: &HashMap<T::Time, T::Event>) -> HashMap<T::Time, ()> {
    let mut maximals = HashMap::new();
    for time in time_events.keys() {
        for max in maximals.keys().copied().collect::<Vec<_>>() {
            if time > &max {
                maximals.remove(&max);
            }
        }

        maximals.insert(*time, ());
    }
    maximals
}
