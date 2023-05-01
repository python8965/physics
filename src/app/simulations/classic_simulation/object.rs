pub mod builder;
pub mod drawing;
pub mod shape;
pub mod state;

use getset::Getters;

use state::{CSObjectState, CSObjectStateBuilder};

use crate::app::simulations::state::SimulationState;

use crate::app::simulations::classic_simulation::object::shape::ObjectShape;

pub type AttachedFn = fn(&mut CSObjectState);

#[derive(Default, Clone, Getters)]
pub struct CSimObject {
    state_timeline: Vec<CSObjectState>,
    init_timestep: usize,
    timestep: usize,

    #[getset(get = "pub")]
    shape: ObjectShape,
    #[getset(get = "pub")]
    hide: bool,
    #[getset(get = "pub")]
    attached: Option<AttachedFn>,
}

impl CSimObject {
    pub fn update(&mut self, sim_state: &SimulationState) {
        self.timestep = sim_state.current_step;
    }

    pub fn save_state(&mut self) {
        self.state_timeline.push(self.current_state());
    }

    pub fn local_timestep(&self, timestep: usize) -> Option<usize> {
        timestep.checked_sub(self.init_timestep)
    }

    pub fn last_state(&self) -> Option<CSObjectState> {
        self.state_timeline.last().cloned()
    }

    pub fn at_timestep(&mut self, timestep: usize) {
        if self.local_timestep(timestep).is_some() {
            self.hide = false;
            self.timestep = timestep;
        } else {
            self.hide = true;
        }
    }

    pub fn current_state(&self) -> CSObjectState {
        self.local_timestep(self.timestep)
            .and_then(|timestep| self.state_timeline.get(timestep.saturating_sub(1)).cloned())
            .unwrap_or_else(|| CSObjectStateBuilder::new().build())
    }

    pub fn current_state_mut(&mut self) -> &mut CSObjectState {
        self.local_timestep(self.timestep)
            .and_then(|timestep| self.state_timeline.get_mut(timestep.saturating_sub(1)))
            .unwrap()
    }

    pub fn state_at_timestep(&self, current_timestep: usize) -> Option<CSObjectState> {
        if self.init_timestep <= current_timestep {
            Some(self.state_timeline[current_timestep - self.init_timestep].clone())
        } else {
            None
        }
    }

    pub fn inspection_ui(&self, ui: &mut egui::Ui) {
        egui::Grid::new("object_inspection_ui").show(ui, |ui| {
            ui.label("Position");
            ui.label(format!("{:?}", self.current_state().position));
            ui.end_row();

            ui.label("Velocity");
            ui.label(format!("{:?}", self.current_state().velocity));
            ui.end_row();

            ui.label("Acceleration");
            ui.label(format!("{:?}", self.current_state().acceleration()));
            ui.end_row();

            ui.label("Sigma Force");
            ui.label(format!("{:?}", self.current_state().sigma_force()));
            ui.end_row();

            ui.label("Mass");
            ui.label(format!("{:?}", self.current_state().mass));
            ui.end_row();
        });
    }
}
