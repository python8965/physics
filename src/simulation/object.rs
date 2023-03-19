use crate::simulation::drawing::ObjectTraceLine;
use crate::simulation::{DrawShapeType, Float};
use egui::{Pos2, Vec2};

#[derive()]
pub struct SimulationObject {
    pub position: Pos2,
    pub momentum: Vec2,

    pub force_list: Vec<Vec2>,

    pub mass: Float,

    pub shape: DrawShapeType,
    pub scale: Option<Float>,

    pub trace: ObjectTraceLine,
}

impl Default for SimulationObject {
    fn default() -> Self {
        Self {
            position: Pos2::new(0.0, 0.0),
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
        self.momentum / self.mass
    }

    pub(crate) fn get_scale(&self) -> Float {
        if self.scale.is_some() {
            self.scale.unwrap()
        } else {
            1.0 + (self.mass / 1.5)
        }
    }
}
