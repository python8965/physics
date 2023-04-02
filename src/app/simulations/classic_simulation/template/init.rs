use crate::app::simulations::classic_simulation::template::CSTemplate;
use egui::{Slider, Ui, Widget};
use std::fmt::Debug;

pub trait SimulationInit: Debug {
    fn ui(&mut self, ui: &mut Ui);

    fn to_simulation_type(&self) -> CSTemplate;
}

#[derive(Clone, Copy, Debug)]
pub struct BasicSimInit {
    pub theta: f64,
    pub start_velocity_mul: f64,
    pub mass: f64,
}

impl SimulationInit for BasicSimInit {
    fn ui(&mut self, ui: &mut Ui) {
        Slider::new(&mut self.theta, 0.0..=90.0)
            .text("Theta Degree")
            .ui(ui);

        Slider::new(&mut self.start_velocity_mul, 10.0..=100.0)
            .text("Velocity Mul")
            .ui(ui);

        Slider::new(&mut self.mass, 1.0..=100.0).text("Mass").ui(ui);
    }

    fn to_simulation_type(&self) -> CSTemplate {
        CSTemplate::BasicSimWithInit(*self)
    }
}
