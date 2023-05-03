use crate::app::NVec2;
use crate::app::simulations::classic_simulation::object::shape::ObjectShape;

#[repr(usize)]
pub enum ForceIndex {
    Attached = 0,
    UserInteraction = 1,
    MAX = 2,
}

#[derive(Clone, Debug)]
pub struct CSObjectState {
    pub position: NVec2,
    pub velocity: NVec2,
    pub last_velocity: NVec2,
    pub mass: f64,
    pub acc_list: Vec<NVec2>,
    pub shape: ObjectShape,
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
            last_velocity: Default::default(),
            mass: 10.0,
            acc_list: vec![NVec2::zeros(); ForceIndex::MAX as usize],
            shape: ObjectShape::default(),
        }
    }
}

pub struct CSObjectStateBuilder {
    state: CSObjectState,
}

impl CSObjectStateBuilder {
    pub fn new() -> Self {
        Self {
            state: Default::default(),
        }
    }

    pub fn from_state(state: CSObjectState) -> Self {
        Self { state }
    }

    pub fn position(&mut self, position: NVec2) -> &mut Self {
        self.state.position = position;
        self
    }

    pub fn velocity(&mut self, velocity: NVec2) -> &mut Self {
        self.state.velocity = velocity;
        self
    }

    pub fn mass(&mut self, mass: f64) -> &mut Self {
        self.state.mass = mass;
        self
    }

    pub fn build(self) -> CSObjectState {
        self.state
    }
}
