use crate::simulation::engine::{BasicSim, Simulation};
use crate::simulation::template::SimulationType;
use crate::simulation::Float;
use eframe::epaint::mutex::MutexGuard;
use egui::mutex::Mutex;
use std::cell::{Ref, RefCell};
use std::rc::Rc;
use std::sync::Arc;
use std::time::Duration;

pub struct SimulationManager {
    simulation: Option<Box<dyn Simulation>>,
    sim_time: f64,
    is_paused: bool,
    current_sim_type: SimulationType,
    last_step_time: f64,
}

impl Default for SimulationManager {
    fn default() -> Self {
        Self {
            simulation: None,
            sim_time: 0.0,
            is_paused: true,
            current_sim_type: Default::default(),
            last_step_time: 0.0,
        }
    }
}

impl SimulationManager {
    pub fn get_time(&self) -> f64 {
        self.sim_time
    }

    pub fn get_simulation_type(&self) -> SimulationType {
        self.current_sim_type
    }

    pub fn get_simulation(&mut self) -> Option<&mut Box<dyn Simulation>> {
        if let Some(sim) = &mut self.simulation {
            Some(sim)
        } else {
            None
        }
    }

    pub fn new_simulation(&mut self, new_simulation: SimulationType) {
        let old = self.simulation.replace(new_simulation.as_func());

        if let Some(mut sim) = old {
            sim.finish();
        }

        self.sim_time = 0.0;
    }

    pub fn step(&mut self, last_time: f64) {
        if !self.is_paused {
            let dt = last_time - self.last_step_time;
            self.sim_time += dt;

            self.last_step_time = last_time;

            if let Some(simulation) = &mut self.simulation {
                if simulation.finished() {
                } else if !self.is_paused {
                    self.sim_time += dt;
                    simulation.step((dt as Float));
                }
            }
        } else {
            self.last_step_time = last_time;
        }
    }

    pub fn reset_simulation(&mut self) {
        if self.current_sim_type != SimulationType::None {
            self.new_simulation(self.current_sim_type);
        }
    }

    pub fn toggle_animation(&mut self) {
        self.is_paused = !self.is_paused;
    }
}
