pub mod debug;

use crate::app::graphics::plot::SimPlot;

use crate::app::simulations::classic_simulation::sim_state::CSimSettings;
use crate::app::simulations::classic_simulation::{ClassicSimulation, Simulation};
use egui::Ui;
use getset::Getters;
use instant::Instant;
use crate::app::manager::debug::DebugShapeStorage;

use crate::app::simulations::classic_simulation::template::init::SimulationInit;
use crate::app::simulations::classic_simulation::template::{CSPreset, CSTemplate};
use crate::app::simulations::state::{
    SimulationSettings, SimulationState, SpecificSimulationSettings,
};

pub const SIMULATION_TICK: f64 = 1.0 / 240.0;

/// This is the main simulation manager. It is responsible for managing the simulation and the plot.
#[derive(Getters)]
pub struct SimulationManager {
    simulation: Option<Box<dyn Simulation>>,
    #[getset(get = "pub")]
    sim_state: SimulationState,
    simulation_plot: SimPlot,

    debug_store: debug::DebugShapeStorage,
    is_paused: bool,
    last_time_stamp: Instant,
    is_sim_initializing: bool,
    initializing_data: Option<Box<dyn SimulationInit>>,
}

impl Default for SimulationManager {
    fn default() -> Self {
        Self {
            simulation: None,
            sim_state: SimulationState::default(),
            simulation_plot: SimPlot::default(),
            debug_store: Default::default(),
            is_paused: true,
            last_time_stamp: Instant::now(),
            is_sim_initializing: false,
            initializing_data: None,
        }
    }
}

/// simple getter and setter
impl SimulationManager {
    pub fn time_multiplier(&mut self) -> &mut usize {
        &mut self.sim_state.time_mul
    }
    pub fn get_time(&self) -> f64 {
        self.sim_state.time
    }
    pub fn get_pause(&self) -> bool {
        self.is_paused
    }
    pub fn timestep(&self) -> usize {
        self.sim_state.max_step
    }
    pub fn current_timestep_mut(&mut self) -> &mut usize {
        &mut self.sim_state.current_step
    }
    pub fn is_initializing(&self) -> bool {
        self.is_sim_initializing
    }
}

/// getter and setter for the simulation
impl SimulationManager {
    pub fn new_simulation(&mut self, simulation_template: CSTemplate) {
        self.is_sim_initializing = false;

        self.initializing_data = simulation_template.get_data();

        let CSPreset {
            simulation_objects,
            plot_objects,
        } = simulation_template.get_preset_with_ui();

        if self.initializing_data.is_some() {
            self.is_sim_initializing = true;
        }

        self.pause();
        self.simulation_plot = SimPlot::new(plot_objects);
        let simulation: Box<dyn Simulation> = Box::new(ClassicSimulation::from(simulation_objects));
        self.sim_state.settings = SimulationSettings::new(
            SpecificSimulationSettings::CSimSettings(CSimSettings::default()),
        );

        self.simulation.replace(simulation);

        self.sim_state.reset();
    }

    pub fn get_simulation(
        &mut self,
    ) -> (
        &mut Option<Box<dyn Simulation>>,
        &mut SimPlot,
        &mut SimulationState,
        &mut DebugShapeStorage
    ) {
        (
            &mut self.simulation,
            &mut self.simulation_plot,
            &mut self.sim_state,
            &mut self.debug_store,
        )
    }

    pub(super) fn pause(&mut self) {
        self.is_paused = true;
    }

    pub(super) fn resume(&mut self) {
        if self.is_sim_initializing {
            self.is_sim_initializing = false;
        }

        if self.sim_state.max_step == self.sim_state.current_step {
            self.is_paused = false;
        }
    }

    pub fn toggle_pause(&mut self) {
        if self.is_paused {
            self.resume();
        } else {
            self.pause();
        }
    }

    pub fn timestep_changed(&mut self) {
        self.pause();
        self.sim_state.time = SIMULATION_TICK * self.sim_state.current_step as f64;

        self.simulation
            .as_mut()
            .unwrap()
            .at_time_step(self.sim_state.current_step);
    }
}

/// for ui
impl SimulationManager {
    pub fn initialize_ui(&mut self, ui: &mut Ui) {
        if self.is_sim_initializing {
            self.initializing_data.as_mut().unwrap().ui(ui);
        }
    }

    pub fn operation_ui(&mut self, ui: &mut Ui) {
        if let Some(simulation) = self.simulation.as_mut() {
            simulation.operation_ui(ui);
        }
    }

    pub fn settings_ui(&mut self, ui: &mut Ui) {
        self.sim_state.settings.ui(ui);
    }

    pub fn inspection_ui(&mut self, ui: &mut Ui) {
        if let Some(simulation) = self.simulation.as_mut() {
            ui.collapsing("Simulation Inspect", |ui| {
                simulation.inspection_ui(ui);
                ui.separator();
            });
        }
    }
}

/// for simulation tick
impl SimulationManager {
    pub fn simulation_step(&mut self) {
        if let Some(simulation) = &mut self.simulation {
            self.sim_state.max_step += 1;
            self.sim_state.current_step = self.sim_state.max_step;

            simulation.step(&mut self.sim_state,&mut self.debug_store);
        }

        self.sim_state.time += SIMULATION_TICK;
    }

    pub fn step(&mut self) {
        if !self.is_paused && self.sim_state.sim_started {
            //let mut dt = self.last_time_stamp.elapsed().as_secs_f64();

            self.last_time_stamp = Instant::now();

            for _ in 0..self.sim_state.time_mul {
                self.simulation_step();
            }
        } else {
            if self.is_sim_initializing {
                let CSPreset {
                    simulation_objects,
                    plot_objects,
                } = self
                    .initializing_data
                    .as_ref()
                    .unwrap()
                    .to_simulation_type()
                    .get_preset_with_ui();

                self.simulation = Some(Box::new(ClassicSimulation::from(simulation_objects)));

                self.simulation_plot.plot_objects = plot_objects;
            }

            self.last_time_stamp = Instant::now();
        }
    }
}
