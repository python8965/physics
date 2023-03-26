use crate::app::simulations::template::ClassicSimulationType;
use egui::{Slider, Ui, Widget};
use std::fmt::Debug;

pub trait SimInit: Debug {
    fn ui(&mut self, ui: &mut Ui);

    fn to_type(&self) -> ClassicSimulationType;
}
#[derive(Clone, Copy, Debug)]
pub struct BasicSimInit {
    pub theta: f64,
}

impl SimInit for BasicSimInit {
    fn ui(&mut self, ui: &mut Ui) {
        Slider::new(&mut self.theta, 0.0..=90.0)
            .text("Theta Degree")
            .ui(ui);
    }

    fn to_type(&self) -> ClassicSimulationType {
        ClassicSimulationType::BasicSimWithInit(*self)
    }
}
