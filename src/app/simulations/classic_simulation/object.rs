use crate::app::graphics::define::DrawShapeType;

use crate::app::graphics::plot::ObjectTraceLine;
use crate::app::NVec2;
use nalgebra::{vector, SMatrix};
use std::ops::Div;

#[derive(Clone, Debug)]
pub struct CSObject {
    pub state: CSObjectState,
    init_state: CSObjectState,

    pub trace_line: ObjectTraceLine,

    pub shape: DrawShapeType,
}

impl Default for CSObject {
    fn default() -> Self {
        Self {
            state: Default::default(),
            init_state: Default::default(),
            trace_line: ObjectTraceLine::new(),
            shape: DrawShapeType::Circle,
        }
    }
}

impl CSObject {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn init_state(&self) -> &CSObjectState {
        &self.init_state
    }

    pub fn state(mut self, state: CSObjectState) -> Self {
        self.state = state.clone();
        self.init_state = state;
        self
    }

    pub fn shape(mut self, shape: DrawShapeType) -> Self {
        self.shape = shape;
        self
    }
}

pub const GRAVITY: SMatrix<f64, 2, 1> = vector![0.0, -9.8];
pub const ZERO_FORCE: SMatrix<f64, 2, 1> = vector![0.0, 0.0];

#[repr(usize)]
pub enum ForceIndex {
    Gravity = 0,
    UserInteraction = 1,
    MAX = 2,
}

#[derive(Clone, Debug)]
pub struct CSObjectState {
    pub position: NVec2,
    pub velocity: NVec2,
    pub mass: f64,
    pub acc_list: Vec<NVec2>,
}

impl CSObjectState {
    pub(crate) fn momentum(&self) -> NVec2 {
        // P = mv , v = P/m
        self.velocity * self.mass
    }

    pub fn sigma_force(&self) -> NVec2 {
        // ΣF = F1 + F2 + F3 + ...
        self.acceleration() / self.mass
    }

    pub fn acceleration(&self) -> NVec2 {
        // a = F/m
        self.acc_list.iter().sum::<NVec2>()
    }

    pub fn scale(&self) -> f64 {
        5.0 + (self.mass / 4.0)
    }
}

impl Default for CSObjectState {
    fn default() -> Self {
        Self {
            position: Default::default(),
            velocity: Default::default(),
            mass: 10.0,
            acc_list: {
                let mut acc_list = vec![NVec2::zeros(); ForceIndex::MAX as usize];
                acc_list[ForceIndex::Gravity as usize] = GRAVITY;
                acc_list
            },
        }
    }
}
