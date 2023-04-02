use std::fmt::{Debug, Formatter};
use std::ops::RangeInclusive;


use crate::app::simulations::classic_simulation::object::CSObjectState;

#[derive(Clone, Debug)]
pub struct CSObjectStampResult {
    pub label: String,
    pub name: String,
    pub time: f64,
    pub state: CSObjectState,
}

impl Default for CSObjectStampResult {
    fn default() -> Self {
        Self {
            label: String::new(),
            name: String::new(),
            time: f64::NAN,
            state: CSObjectState::default(),
        }
    }
}

impl CSObjectStampResult {
    pub fn label(mut self, label: impl Into<String>) -> Self {
        self.label = label.into();
        self
    }

    pub fn state(mut self, state: CSObjectState) -> Self {
        self.state = state;
        self
    }

    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = name.into();
        self
    }

    pub fn time(mut self, time: f64) -> Self {
        self.time = time;
        self
    }
}

pub type CSObjectStampFunction = fn(&CSObjectState, f64) -> Option<CSObjectStampResult>;

#[derive(Clone)]
pub enum StampState {
    Stamped(CSObjectStampResult),
    NotStamped(CSObjectStampFunction),
}

impl Debug for StampState {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            StampState::Stamped(result) => write!(f, "Stamped({:?})", result),
            StampState::NotStamped(_) => write!(f, "NotStamped"),
        }
    }
}

#[derive(Clone, Debug)]
pub struct CSObjectStamp {
    state: StampState,
    stamp_range: RangeInclusive<usize>,
}

impl CSObjectStamp {
    pub fn new(function: CSObjectStampFunction, range: impl Into<RangeInclusive<usize>>) -> Self {
        Self {
            state: StampState::NotStamped(function),
            stamp_range: range.into(),
        }
    }

    pub fn get_data(
        &mut self,
        obj_state: &CSObjectState,
        obj_index: usize,
        time: f64,
    ) -> Option<CSObjectStampResult> {
        if !self.stamp_range.contains(&obj_index) {
            None
        } else {
            match &self.state {
                StampState::Stamped(result) => Some(result.clone()),
                StampState::NotStamped(func) => {
                    let result = func(obj_state, time);

                    if let Some(result) = result {
                        self.state = StampState::Stamped(result.clone());
                        Some(result)
                    } else {
                        None
                    }
                }
            }
        }
    }
}
