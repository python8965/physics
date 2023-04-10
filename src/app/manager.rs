use crate::app::graphics::plot::CSPlot;
use crate::app::simulations::classic_simulation::{
    ClassicSimulation, GlobalForceSlot, Simulation, GRAVITY, ZERO_FORCE,
};
use crate::app::{manager, Float};
use egui::Ui;
use instant::Instant;

use crate::app::simulations::classic_simulation::state::{CSimState, PlotViewFilter};
use crate::app::simulations::classic_simulation::template::init::SimulationInit;
use crate::app::simulations::classic_simulation::template::{CSPreset, CSTemplate};

/// This is the main simulation manager. It is responsible for managing the simulation and the plot.
pub struct SimulationManager {
    simulation: Option<Box<dyn Simulation>>,

    sim_state: CSimState,

    timestep: Vec<f32>,

    simulation_plot: CSPlot,

    is_paused: bool,

    last_time_stamp: Instant,

    is_sim_initializing: bool,

    initializing_data: Option<Box<dyn SimulationInit>>,
}

impl Default for SimulationManager {
    fn default() -> Self {
        Self {
            simulation: None,
            sim_state: CSimState::default(),
            timestep: vec![],
            simulation_plot: CSPlot::default(),
            is_paused: true,
            last_time_stamp: Instant::now(),
            is_sim_initializing: false,
            initializing_data: None,
        }
    }
}

/// simple getter and setter
impl SimulationManager {
    pub fn time_multiplier(&mut self) -> &mut f64 {
        &mut self.sim_state.time_mul
    }

    pub fn settings_mut(&mut self) -> &mut PlotViewFilter {
        &mut self.sim_state.settings
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
        self.simulation_plot = CSPlot::new(plot_objects);
        let mut simulation: Box<dyn Simulation> =
            Box::new(ClassicSimulation::from(simulation_objects));

        set_global_gravity(self.sim_state.gravity, &mut simulation);

        self.simulation.replace(simulation);

        self.sim_state.reset();
        self.timestep.clear();
    }

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

    pub(super) fn pause(&mut self) {
        self.is_paused = true;
    }

    pub(super) fn resume(&mut self) {
        if self.is_sim_initializing {
            self.is_sim_initializing = false;
            if let Some(simulation) = self.simulation.as_mut() {
                simulation.get_children().iter_mut().for_each(|obj| {
                    obj.init();
                });
            }
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
        if !self.timestep.is_empty() {
            self.sim_state.time = self.timestep[self.sim_state.current_step] as f64;

            for obj in self.simulation.as_mut().unwrap().get_children() {
                obj.state = obj.state_at_step(self.sim_state.current_step);
            }
        }
    }
}

pub fn set_global_gravity(toggle: bool, simulation: &mut Box<dyn Simulation>) {
    simulation.set_global_force(
        GlobalForceSlot::Gravity,
        if toggle { GRAVITY } else { ZERO_FORCE },
    );
}

/// for ui
impl SimulationManager {
    pub fn initialize_ui(&mut self, ui: &mut Ui) {
        if self.is_sim_initializing {
            self.initializing_data.as_mut().unwrap().ui(ui);
        }
    }

    pub fn settings_ui(&mut self, ui: &mut Ui) {
        ui.collapsing("Simulation Settings", |ui| {
            if ui
                .checkbox(&mut self.sim_state.gravity, "Gravity?")
                .changed()
            {
                if let Some(simulation) = self.simulation.as_mut() {
                    set_global_gravity(self.sim_state.gravity, simulation);
                }
            };
        });
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
    pub fn step(&mut self) {
        if !self.is_paused && self.sim_state.sim_started {
            let mut dt = self.last_time_stamp.elapsed().as_secs_f64();

            self.last_time_stamp = Instant::now();
            dt *= self.sim_state.time_mul;

            if let Some(simulation) = &mut self.simulation {
                self.sim_state.max_step = self.timestep.len();
                self.sim_state.current_step = self.sim_state.max_step;
                self.timestep.push(self.sim_state.time as f32);

                simulation.step(dt as Float);
            }

            self.sim_state.time += dt;
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

                let _ = std::mem::replace(
                    self.simulation.as_mut().unwrap().get_children(),
                    simulation_objects,
                );
                self.simulation_plot.plot_objects = plot_objects;
            }

            self.last_time_stamp = Instant::now();
        }
    }
}
