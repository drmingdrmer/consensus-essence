use std::collections::BTreeMap;

use crate::apaxos::history::History;
use crate::Types;

pub type Mode = bool;
pub const SINGLE_VALUE: Mode = false;
pub const MULTI_VALUE: Mode = true;

#[derive(Clone, Debug)]
pub struct LinearHistory<T: Types, const MODE: Mode> {
    time_events: BTreeMap<T::Time, T::Event>,
}

impl<T: Types, const MODE: Mode> LinearHistory<T, MODE>
where T::Time: Ord
{
    pub fn latest_time_value(&self) -> Option<(&T::Time, &T::Event)> {
        self.time_events.last_key_value()
    }

    pub fn latest_time(&self) -> Option<T::Time> {
        self.latest_time_value().map(|(t, _ev)| *t)
    }

    pub fn latest_value(&self) -> Option<T::Event> {
        self.latest_time_value().map(|(_t, v)| v.clone())
    }
}

impl<T: Types, const MODE: Mode> Default for LinearHistory<T, MODE> {
    fn default() -> Self {
        Self {
            time_events: BTreeMap::new(),
        }
    }
}

/// Linear history requires the Time to be totally ordered.
impl<T, const MODE: Mode> History<T> for LinearHistory<T, MODE>
where
    T: Types<History = Self>,
    T::Time: Ord,
{
    fn do_append(&mut self, time: T::Time, event: T::Event) {
        // In a single value mode(such as classic paxos),
        // it disallows to append new event if there is already one.
        // Because the history can not be changed.
        // In such case, just use the last value.
        let ev = match MODE {
            SINGLE_VALUE => {
                if let Some((_t, ev)) = self.time_events.last_key_value() {
                    ev.clone()
                } else {
                    event
                }
            }
            MULTI_VALUE => event,
        };

        self.time_events.insert(time, ev);
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
