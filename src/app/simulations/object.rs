use crate::app::graphics::DrawShapeType;
use crate::app::NVec2;
use std::ops::Div;

#[derive(Clone, Debug)]
pub struct ClassicSimulationObject {
    pub position: NVec2,
    pub momentum: NVec2,

    pub force_list: Vec<NVec2>,

    pub mass: f64,

    pub shape: DrawShapeType,
    pub scale: Option<f64>,
}

impl Default for ClassicSimulationObject {
    fn default() -> Self {
        Self {
            position: NVec2::new(0.0, 0.0),
            momentum: Default::default(),
            force_list: vec![],
            mass: 1.0,
            shape: DrawShapeType::Circle,
            scale: None,
        }
    }
}

impl ClassicSimulationObject {
    pub(crate) fn velocity(&self) -> NVec2 {
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
