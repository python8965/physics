
use egui::plot::{Arrows};
use crate::app::simulations::classic_simulation::object::shape::{ObjectShape};

use crate::app::NVec2;
use nalgebra::{vector};
use crate::app::graphics::define::{ PlotItem};
use crate::app::simulations::classic_simulation::event::CollisionEvent;

trait ListAdd<Rhs = Self> {
    type Output;
    fn add(self, rhs: Rhs) -> Self::Output;
}

impl ListAdd for [f64; 2]{
    type Output = [f64; 2];

    fn add(self, rhs: [f64; 2]) -> Self {
        [self[0] + rhs[0], self[1] + rhs[1]]
    }
}

pub trait Collision {
    fn contact(&self, ops: &CSObjectState) -> Option<CollisionEvent>;
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
    fn contact(&self, ops: &CSObjectState) -> Option<CollisionEvent> {
        let mut shape = None;
        let info = match (self.shape, ops.shape) {
            (ObjectShape::Circle(circle), ObjectShape::Circle(circle2)) => {
                let dist = (self.position - ops.position).magnitude();
                let penetration = circle.radius + circle2.radius - dist;

                if penetration > 0.0 {
                    let delta_pos = self.position - ops.position;
                    let contact_normal = if delta_pos.norm() == 0.0 {
                        vector![0.0, 0.0]
                    } else {
                        delta_pos.normalize()
                    };

                    let contact_point = self.position - contact_normal * circle.radius;

                    //
                    let scale1 = contact_normal.yx().dot(&self.momentum())
                        / contact_normal.yx().dot(&contact_normal);

                    let scale2 = contact_normal.yx().dot(&ops.momentum())
                        / contact_normal.yx().dot(&contact_normal);

                    let obj1_scale = scale2;
                    let obj2_scale = scale1;

                    let obj1_velocity = contact_normal * obj1_scale / self.mass;
                    let obj2_velocity = contact_normal * -obj2_scale / ops.mass;

                    let _raw_contact_point = contact_point.data.0[0];

                    let raw_self_position = self.position.data.0[0];
                    let raw_ops_position = ops.position.data.0[0];

                    shape = Some(Box::new(move ||{
                        vec![
                            PlotItem::Arrows(Arrows::new(vec![raw_self_position], vec![
                                raw_ops_position
                            ]).name("contact_normal")),
                            PlotItem::Arrows(Arrows::new(vec![
                                raw_self_position

                            ], vec![

                                raw_self_position.add(obj1_velocity.data.0[0]),
                            ]).name("obj1_velocity")),
                            PlotItem::Arrows(Arrows::new(vec![
                                raw_ops_position

                            ], vec![
                                raw_ops_position.add(obj2_velocity.data.0[0]),
                            ]).name("obj2_velocity")),
                        ]
                    }));

                    Some(CollisionEvent {
                        contact_point,
                        contact_normal,
                        penetration,

                        obj1_state: self.clone(),
                        obj2_state: ops.clone(),

                        obj1_velocity,
                        obj2_velocity
                    })
                } else {
                    None
                }
            }
            _ => None,
        };

        info
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
