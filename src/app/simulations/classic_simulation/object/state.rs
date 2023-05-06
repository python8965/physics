use crate::app::simulations::classic_simulation::object::shape::{ContactInfo, ObjectShape};
use crate::app::simulations::classic_simulation::CSimObject;
use crate::app::NVec2;
use nalgebra::vector;

pub trait Collision {
    fn contact(&self, ops: &CSObjectState) -> Option<ContactInfo>;
}

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

impl Collision for CSObjectState {
    fn contact(&self, ops: &CSObjectState) -> Option<ContactInfo> {
        match (self.shape, ops.shape) {
            (ObjectShape::Circle(circle), ObjectShape::Circle(circle2)) => {
                let dist = (self.position - ops.position).magnitude();
                let penetration = circle.radius + circle2.radius - dist;
                if penetration > 0.0 {
                    let delta_pos = (self.position - ops.position);
                    dbg!(delta_pos.norm());
                    let contact_normal = if delta_pos.norm() == 0.0 {
                        vector![0.0, 0.0]
                    } else {
                        delta_pos.normalize()
                    };

                    dbg!(self.position, ops.position);
                    let contact_point = self.position - contact_normal * circle.radius;

                    let contact_momentum = (self.momentum() + ops.momentum()).norm();
                    Some(ContactInfo {
                        contact_point,
                        contact_normal,
                        penetration,
                        contact_momentum,
                    })
                } else {
                    None
                }
            }
            _ => None,
        }
    }
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
