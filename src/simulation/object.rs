use crate::simulation::drawing::ObjectTraceLine;
use crate::simulation::{DrawShapeType, Vec2};
use std::ops::Div;

#[derive()]
pub struct SimulationObject {
    pub position: Vec2,
    pub momentum: Vec2,

    pub force_list: Vec<Vec2>,

    pub mass: f64,

    pub shape: DrawShapeType,
    pub scale: Option<f64>,

    pub trace: ObjectTraceLine,
}

impl Default for SimulationObject {
    fn default() -> Self {
        Self {
            position: Vec2::new(0.0, 0.0),
            momentum: Default::default(),
            force_list: vec![],
            mass: 1.0,
            shape: DrawShapeType::Circle,
            scale: None,
            trace: ObjectTraceLine::new(),
        }
    }
}

impl SimulationObject {
    pub(crate) fn velocity(&self) -> Vec2 {
        // p = mv -> v = p/m
        self.momentum.div(self.mass)
    }

    pub(crate) fn get_scale(&self) -> f64 {
        if self.scale.is_some() {
            self.scale.unwrap()
        } else {
            1.0 + (self.mass / 1.5)
        }
    }
}
