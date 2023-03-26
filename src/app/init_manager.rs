use crate::app::simulations::object::ClassicSimulationObject;
use crate::app::simulations::template::init::SimInit;
use crate::app::simulations::template::{
    ClassicSimulationPreset, ClassicSimulationType, PlotObjectFnVec,
};
use egui::Ui;

#[derive(Default)]
pub struct SimulationInitManager {
    is_sim_initializing: bool,
    is_sim_ready: bool,
    initializing_data: Option<Box<dyn SimInit>>,
}

impl SimulationInitManager {
    pub fn new_simulation(
        &mut self,
        simulation_type: ClassicSimulationType,
    ) -> (Vec<ClassicSimulationObject>, PlotObjectFnVec) {
        self.is_sim_initializing = false;
        self.is_sim_ready = false;

        self.initializing_data = simulation_type.get_data();

        let ClassicSimulationPreset {
            simulation_objects,
            objects_fn,
        } = simulation_type.get_preset_with_ui();

        if self.initializing_data.is_some() {
            if self.is_sim_ready {
            } else {
                self.is_sim_initializing = true;
            }
        } else {
            self.is_sim_ready = true;
        }

        (simulation_objects, objects_fn)
    }

    pub fn ui(&mut self, ui: &mut Ui) {
        if self.is_sim_initializing {
            self.initializing_data.as_mut().unwrap().ui(ui);
        }
    }

    pub fn is_initializing(&self) -> bool {
        self.is_sim_initializing
    }

    pub fn get_current_sim_init_type(&self) -> ClassicSimulationType {
        self.initializing_data.as_ref().unwrap().to_type()
    }

    pub fn resume(&mut self) {
        self.is_sim_initializing = false;
        self.is_sim_ready = true;
    }
}
