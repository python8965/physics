use crate::app::graphics::define::DrawShapeType;
use crate::app::graphics::plot;
use crate::app::graphics::plot::ObjectTraceLine;
use crate::app::NVec2;
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

#[derive(Clone, Debug)]
pub struct CSObjectState {
    pub position: NVec2,
    pub momentum: NVec2,
    pub mass: f64,
    pub velocity_list: Vec<NVec2>,
}

impl CSObjectState {
    pub fn position(mut self, position: NVec2) -> Self {
        self.position = position;
        self
    }

    pub fn momentum(mut self, momentum: NVec2) -> Self {
        self.momentum = momentum;
        self
    }

    pub fn mass(mut self, mass: f64) -> Self {
        self.mass = mass;
        self
    }

    pub fn velocity_list(mut self, velocity_list: Vec<NVec2>) -> Self {
        self.velocity_list = velocity_list;
        self
    }

    pub(crate) fn velocity(&self) -> NVec2 {
        // P = mv , v = P/m
        self.momentum.div(self.mass)
    }

    pub fn sigma_force(&self) -> NVec2 {
        // ΣF = F1 + F2 + F3 + ...
        self.velocity_list.iter().sum::<NVec2>() * self.mass
    }

    pub fn acceleration(&self) -> NVec2 {
        // a = F/m
        self.sigma_force().div(self.mass)
    }

    pub fn scale(&self) -> f64 {
        5.0 + (self.mass / 2.0)
    }
}

impl Default for CSObjectState {
    fn default() -> Self {
        Self {
            position: Default::default(),
            momentum: Default::default(),
            mass: 10.0,
            velocity_list: vec![NVec2::new(0.0, -9.8)],
        }
    }
}
