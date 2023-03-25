use crate::app::graphics::plotting::{ObjectTraceLine, SimulationPlot};
use crate::app::simulations::simengine::{ClassicSimulation, Simulation};
use crate::app::simulations::state::{PlotInfoFilter, SimulationState};
use crate::app::simulations::template::{ClassicSimulationPreset, ClassicSimulationType};
use crate::app::Float;

pub struct SimulationManager {
    simulation: Option<Box<dyn Simulation>>,

    sim_state: SimulationState,

    simulation_plot: SimulationPlot,

    sim_time_multiplier: f64,

    is_paused: bool,
    last_step_time: f64,
}

impl Default for SimulationManager {
    fn default() -> Self {
        Self {
            simulation: None,
            sim_state: SimulationState::default(),
            simulation_plot: SimulationPlot::default(),
            sim_time_multiplier: 1.0,
            is_paused: true,
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

    pub fn state(&mut self) -> SimulationState {
        self.sim_state
    }

    pub fn set_state(&mut self, state: SimulationState) {
        self.sim_state = state
    }

    pub fn get_simulation(
        &mut self,
    ) -> (
        &mut Option<Box<dyn Simulation>>,
        &mut SimulationPlot,
        &mut SimulationState,
    ) {
        (
            &mut self.simulation,
            &mut self.simulation_plot,
            &mut self.sim_state,
        )
    }

    pub fn new_simulation(&mut self, new_simulation: ClassicSimulationType) {
        let ClassicSimulationPreset {
            simulation_objects,
            objects_fn,
        } = new_simulation.get_preset();

        self.simulation_plot = SimulationPlot::new(simulation_objects.len(), objects_fn);

        self.simulation
            .replace(Box::new(ClassicSimulation::from(simulation_objects)));

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

    pub fn toggle_animation(&mut self) {
        self.is_paused = !self.is_paused;
    }
}
