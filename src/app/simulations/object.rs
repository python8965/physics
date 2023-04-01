use crate::app::graphics::define::DrawShapeType;
use crate::app::NVec2;
use std::ops::Div;

pub struct ClassicSimulationObjectBuilder {
    current_state: Option<ObjectState>,
    init_state: Option<ObjectState>,
    shape: Option<DrawShapeType>,
}

impl ClassicSimulationObjectBuilder {
    pub fn new() -> Self {
        Self {
            current_state: None,
            init_state: None,
            shape: None,
        }
    }

    pub fn state(mut self, state: ObjectState) -> Self {
        self.current_state = Some(state.clone());
        self.init_state = Some(state);
        self
    }

    pub fn shape(mut self, shape: DrawShapeType) -> Self {
        self.shape = Some(shape);
        self
    }

    pub fn get(self) -> ClassicSimulationObject {
        ClassicSimulationObject {
            state: self.current_state.unwrap_or_default(),
            init_state: self.init_state.unwrap_or_default(),
            shape: self.shape.unwrap_or(DrawShapeType::Circle),
        }
    }
}

#[derive(Clone, Debug)]
pub struct ObjectState {
    pub position: NVec2,
    pub momentum: NVec2,
    pub mass: f64,
    pub force_list: Vec<NVec2>,
}

impl ObjectState {
    pub(crate) fn velocity(&self) -> NVec2 {
        // P = mv , v = P/m
        self.momentum.div(self.mass)
    }

    pub fn sigma_force(&self) -> NVec2 {
        // ΣF = F1 + F2 + F3 + ...
        self.force_list.iter().sum::<NVec2>()
    }

    pub fn acceleration(&self) -> NVec2 {
        // a = F/m
        self.sigma_force().div(self.mass)
    }

    pub fn scale(&self) -> f64 {
        5.0 + (self.mass / 2.0)
    }
}

impl Default for ObjectState {
    fn default() -> Self {
        Self {
            position: Default::default(),
            momentum: Default::default(),
            mass: 10.0,
            force_list: vec![NVec2::new(0.0, -9.8)],
        }
    }
}

#[derive(Clone, Debug)]
pub struct ClassicSimulationObject {
    pub state: ObjectState,
    init_state: ObjectState,

    pub shape: DrawShapeType,
}

impl Default for ClassicSimulationObject {
    fn default() -> Self {
        Self {
            state: Default::default(),
            init_state: Default::default(),
            shape: DrawShapeType::Circle,
        }
    }
}

impl ClassicSimulationObject {
    pub fn init_state(&self) -> &ObjectState {
        &self.init_state
    }
}
