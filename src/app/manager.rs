use crate::app::graphics::plot::CSPlot;
use crate::app::simulations::classic_simulation::{ClassicSimulation, Simulation};
use crate::app::Float;
use egui::Ui;
use instant::Instant;
use tracing::info;

mod init_manager;

use crate::app::simulations::classic_simulation::state::{CSimState, PlotViewFilter};
use crate::app::simulations::classic_simulation::template::{CSPreset, CSTemplate};
pub use init_manager::SimulationInitManager;

pub struct SimulationManager {
    simulation: Option<Box<dyn Simulation>>,

    sim_state: CSimState,

    simulation_plot: CSPlot,

    sim_time_multiplier: f64,

    is_paused: bool,

    init_manager: SimulationInitManager,
    last_time_stamp: Instant,
}

impl Default for SimulationManager {
    fn default() -> Self {
        Self {
            simulation: None,
            sim_state: CSimState::default(),
            simulation_plot: CSPlot::default(),
            sim_time_multiplier: 1.0,
            is_paused: true,
            init_manager: SimulationInitManager::default(),
            last_time_stamp: Instant::now(),
        }
    }
}

impl SimulationManager {
    pub fn time_multiplier(&mut self) -> &mut f64 {
        &mut self.sim_time_multiplier
    }

    pub fn settings_mut(&mut self) -> &mut PlotViewFilter {
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
        &mut CSPlot,
        &mut CSimState,
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

    pub fn new_simulation(&mut self, simulation_template: CSTemplate) {
        let CSPreset {
            simulation_objects,
            plot_objects,
        } = self
            .init_manager
            .get_new_simulation_data(simulation_template);

        self.pause();
        self.simulation_plot = CSPlot::new(plot_objects);

        self.simulation
            .replace(Box::new(ClassicSimulation::from(simulation_objects)));

        self.sim_state.time = 0.0;
    }

    pub fn update_simulation(&mut self) {
        let CSPreset {
            simulation_objects,
            plot_objects,
        } = self.init_manager.get_update_simulation_data();
        let _ = std::mem::replace(
            self.simulation.as_mut().unwrap().get_children(),
            simulation_objects,
        );
        self.simulation_plot.plot_objects = plot_objects;
    }

    pub fn step(&mut self) {
        if !self.is_paused {
            let mut dt = self.last_time_stamp.elapsed().as_secs_f64();
            info!("dt: {}", dt);
            if dt > (1.0 / 60.0) {
                self.last_time_stamp = Instant::now();
                dt *= self.sim_time_multiplier;

                if let Some(simulation) = &mut self.simulation {
                    simulation.step(dt as Float);
                }

                self.sim_state.time += dt;
            }
        } else {
            if self.init_manager.is_initializing() {
                self.update_simulation();
            }

            self.last_time_stamp = Instant::now();
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
