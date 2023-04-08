use crate::app::simulations::classic_simulation::template::CSTemplate;
use egui::{Slider, Ui, Widget};
use std::fmt::Debug;

pub trait SimulationInit: Debug {
    fn ui(&mut self, ui: &mut Ui);

    fn to_simulation_type(&self) -> CSTemplate;
}

#[derive(Clone, Copy, Debug)]
pub struct BasicSimInitObjData{
    pub theta: f64,
    pub start_velocity_mul: f64,
    pub mass: f64,
}

#[derive(Clone, Debug)]
pub struct BasicSimInit {
    pub objects: Vec<BasicSimInitObjData>
}

impl SimulationInit for BasicSimInit {
    fn ui(&mut self, ui: &mut Ui) {
        if ui.button("Add Object").on_hover_text("Add Object").clicked() {
            self.objects.push(BasicSimInitObjData {
                theta: 0.0,
                start_velocity_mul: 10.0,
                mass: 10.0,
            });
        }

        if ui.button("Remove Object").on_hover_text("Remove Object").clicked() {
            self.objects.pop();
        }

        let mut remove = None;

        let _response = self.objects.iter_mut().enumerate().map(|(index, obj)|{
            ui.separator();

            let theta = Slider::new(&mut obj.theta, 0.0..=90.0)
                .text("Theta Degree")
                .ui(ui);

            let velocity = Slider::new(&mut obj.start_velocity_mul, 10.0..=100.0)
                .text("Velocity Mul")
                .ui(ui);

            let mass = Slider::new(&mut obj.mass, 1.0..=100.0).text("Mass").ui(ui);

            if ui.button("Remove this Object").clicked(){
                remove.replace(index);
            }
            ui.separator();
            (theta, velocity, mass)
        }).collect::<Vec<_>>();

        if let Some(index) = remove {
            self.objects.remove(index);
        }

    }

    fn to_simulation_type(&self) -> CSTemplate {
        CSTemplate::BasicSimWithInit(self.clone())
    }
}
