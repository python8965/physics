use crate::app::graphics::plotting::SimulationPlot;
use crate::app::init_manager::SimulationInitManager;
use crate::app::simulations::classic_simulation::{ClassicSimulation, Simulation};
use crate::app::simulations::state::{PlotSettings, SimulationState};
use crate::app::simulations::template::ClassicSimulationType;
use crate::app::Float;
use egui::Ui;
use std::time::Instant;
use tracing::info;

pub struct SimulationManager {
    simulation: Option<Box<dyn Simulation>>,

    sim_state: SimulationState,

    simulation_plot: SimulationPlot,

    sim_time_multiplier: f64,

    is_paused: bool,
    last_time: f64,

    init_manager: SimulationInitManager,
}

impl Default for SimulationManager {
    fn default() -> Self {
        Self {
            simulation: None,
            sim_state: SimulationState::default(),
            simulation_plot: SimulationPlot::default(),
            sim_time_multiplier: 1.0,
            is_paused: true,
            last_time: 0.0,
            init_manager: SimulationInitManager::default(),
        }
    }
}

impl SimulationManager {
    pub fn time_multiplier(&mut self) -> &mut f64 {
        &mut self.sim_time_multiplier
    }

    pub fn settings_mut(&mut self) -> &mut PlotSettings {
        &mut self.sim_state.settings
    }

    pub fn get_time(&self) -> f64 {
        self.sim_state.time
    }

    // pub fn state(&mut self) -> SimulationState {
    //     self.sim_state
    // }
    //
    // pub fn set_state(&mut self, state: SimulationState) {
    //     self.sim_state = state
    // }

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

    pub fn initialize_ui(&mut self, ui: &mut Ui) {
        self.init_manager.ui(ui);
    }

    pub fn new_simulation(&mut self, simulation_type: ClassicSimulationType) {
        let (simulation_objects, objects_fn) =
            self.init_manager.get_new_simulation_data(simulation_type);

        self.pause();
        self.simulation_plot = SimulationPlot::new(simulation_objects.len(), objects_fn);

        self.simulation
            .replace(Box::new(ClassicSimulation::from(simulation_objects)));

        self.sim_state.time = 0.0;
    }

    pub fn update_simulation(&mut self) {
        let (simulation_objects, objects_fn) = self.init_manager.get_update_simulation_data();
        let _ = std::mem::replace(
            self.simulation.as_mut().unwrap().get_children(),
            simulation_objects,
        );
        self.simulation_plot.objects_fn = objects_fn;
    }

    pub fn step(&mut self, current_time: f64) {
        if !self.is_paused {
            let mut dt = current_time - self.last_time;
            if dt > (1.0 / 60.0) {
                dt *= self.sim_time_multiplier;

                let start = Instant::now();

                if let Some(simulation) = &mut self.simulation {
                    simulation.step(dt as Float);
                }

                let sim_time = start.elapsed().as_secs_f64();

                dt += sim_time;

                self.sim_state.time += dt;

                self.last_time = current_time;
            }
        } else {
            if self.init_manager.is_initializing() {
                self.update_simulation();
            }

            self.last_time = current_time;
        }
    }

    fn pause(&mut self) {
        self.is_paused = true;
    }

    fn resume(&mut self) {
        self.init_manager.resume();

        self.is_paused = false;
    }

    pub fn toggle_pause(&mut self) {
        if self.is_paused {
            self.resume();
        } else {
            self.pause();
        }
    }

    pub fn get_pause(&self) -> bool {
        self.is_paused
    }
}
