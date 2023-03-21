use crate::simulation::drawing::PlotInfoFilter;
use crate::simulation::engine::{SimState, Simulation};
use crate::simulation::template::SimulationType;
use crate::simulation::Float;

pub struct SimulationManager {
    simulation: Option<Box<dyn Simulation>>,

    sim_state: SimState,

    sim_time_multiplier: f64,

    is_paused: bool,
    current_sim_type: SimulationType,
    last_step_time: f64,
}

impl Default for SimulationManager {
    fn default() -> Self {
        Self {
            simulation: None,
            sim_state: SimState::default(),
            sim_time_multiplier: 1.0,
            is_paused: true,
            current_sim_type: Default::default(),
            last_step_time: 0.0,
        }
    }
}

impl SimulationManager {
    pub fn time_multiplier(&mut self) -> &mut f64 {
        &mut self.sim_time_multiplier
    }

    pub fn filter_mut(&mut self) -> &mut PlotInfoFilter {
        &mut self.sim_state.filter
    }

    pub fn get_time(&self) -> f64 {
        self.sim_state.time
    }

    pub fn get_state(&self) -> SimState {
        self.sim_state
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
        self.simulation.replace(new_simulation.to_simulation());

        self.sim_state.time = 0.0;
    }

    pub fn step(&mut self, last_time: f64) {
        if !self.is_paused {
            let mut dt = last_time - self.last_step_time;

            dt *= self.sim_time_multiplier;

            self.sim_state.time += dt;

            self.last_step_time = last_time;

            if let Some(simulation) = &mut self.simulation {
                if !self.is_paused {
                    self.sim_state.time += dt;
                    simulation.step(dt as Float);
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
