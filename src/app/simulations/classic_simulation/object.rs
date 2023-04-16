pub mod drawing;

use crate::app::graphics::define::DrawShapeType;
use std::fmt::Debug;

use crate::app::graphics::plot::ObjectTraceLine;

use crate::app::NVec2;

pub type AttachedFn = fn(&mut CSObjectState, f64);

#[derive(Clone)]
pub struct CSimObject {
    pub state: CSObjectState,

    pub state_history: Vec<CSObjectStateHistory>,

    pub trace_line: ObjectTraceLine,

    pub init_timestep: usize,

    pub shape: DrawShapeType,

    pub hide: bool,

    pub attached: Option<AttachedFn>,
}

impl Default for CSimObject {
    fn default() -> Self {
        Self {
            state: Default::default(),
            state_history: vec![],
            trace_line: ObjectTraceLine::new(),
            init_timestep: 0,
            shape: DrawShapeType::Circle,
            hide: false,
            attached: None,
        }
    }
}

impl CSimObject {
    pub fn init_state(&self) -> CSObjectState {
        if let Some(state) = self.state_history.first() {
            state.state.clone()
        } else {
            self.state.clone()
        }
    }

    pub fn state_at_timestep(&self, current_timestep: usize) -> Option<CSObjectState> {
        let current_timestep = if current_timestep != 0 {
            current_timestep - 1
        } else {
            current_timestep
        };

        if self.init_timestep <= current_timestep {
            Some(
                self.state_history[current_timestep - self.init_timestep]
                    .state
                    .clone(),
            )
        } else {
            None
        }
    }

    pub fn inspection_ui(&self, ui: &mut egui::Ui) {
        egui::Grid::new("object_inspection_ui").show(ui, |ui| {
            ui.label("Position");
            ui.label(format!("{:?}", self.state.position));
            ui.end_row();

            ui.label("Velocity");
            ui.label(format!("{:?}", self.state.velocity));
            ui.end_row();

            ui.label("Acceleration");
            ui.label(format!("{:?}", self.state.acceleration()));
            ui.end_row();

            ui.label("Sigma Force");
            ui.label(format!("{:?}", self.state.sigma_force()));
            ui.end_row();

            ui.label("Mass");
            ui.label(format!("{:?}", self.state.mass));
            ui.end_row();
        });
    }
}

#[repr(usize)]
pub enum ForceIndex {
    Attached = 0,
    UserInteraction = 1,
    MAX = 2,
}

#[derive(Clone, Debug)]
pub struct CSObjectStateHistory {
    pub state: CSObjectState,
    pub dt: f64,
}

impl CSObjectStateHistory {
    pub fn new(state: CSObjectState, dt: f64) -> Self {
        Self { state, dt }
    }
}

#[derive(Clone, Debug)]
pub struct CSObjectState {
    pub position: NVec2,
    pub velocity: NVec2,
    pub last_velocity: NVec2,
    pub mass: f64,
    pub acc_list: Vec<NVec2>,
}

impl CSObjectState {
    pub(crate) fn momentum(&self) -> NVec2 {
        // P = mv , v = P/m
        self.velocity * self.mass
    }

    pub fn sigma_force(&self) -> NVec2 {
        // ΣF = F1 + F2 + F3 + ...
        self.acceleration() / self.mass
    }

    pub fn acceleration(&self) -> NVec2 {
        // a = F/m
        self.acc_list.iter().sum::<NVec2>()
    }

    pub fn scale(&self) -> f64 {
        5.0 + (self.mass / 4.0)
    }
}

impl Default for CSObjectState {
    fn default() -> Self {
        Self {
            position: Default::default(),
            velocity: Default::default(),
            last_velocity: Default::default(),
            mass: 10.0,
            acc_list: vec![NVec2::zeros(); ForceIndex::MAX as usize],
        }
    }
}
