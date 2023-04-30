pub mod drawing;

use crate::app::graphics::define::DrawShapeType;
use getset::Getters;
use std::fmt::Debug;

use crate::app::simulations::state::SimulationState;
use crate::app::NVec2;

pub type AttachedFn = fn(&mut CSObjectState);

pub struct CSimObjectTimeline {}

pub struct CSimObjectBuilder {
    init_state: Option<CSObjectState>,
    init_timestep: Option<usize>,
    shape: Option<DrawShapeType>,
    attached: Option<AttachedFn>,
}

impl CSimObjectBuilder {
    pub fn new(state: CSObjectState) -> Self {
        Self {
            init_state: Some(state),
            init_timestep: None,
            shape: None,
            attached: None,
        }
    }

    pub fn init_timestep(mut self, init_timestep: usize) -> Self {
        self.init_timestep = Some(init_timestep);
        self
    }

    pub fn shape(mut self, shape: DrawShapeType) -> Self {
        self.shape = Some(shape);
        self
    }

    pub fn attached(mut self, attached: AttachedFn) -> Self {
        self.attached = Some(attached);
        self
    }

    pub fn build(self) -> CSimObject {
        let init_timestep = self.init_timestep.unwrap_or(0);
        CSimObject {
            state_timeline: vec![self.init_state.unwrap_or_default()],
            init_timestep,
            timestep: init_timestep,
            shape: self.shape.unwrap_or(DrawShapeType::Circle),
            hide: false,
            attached: self.attached,
        }
    }
}

#[derive(Clone, Getters)]
pub struct CSimObject {
    state_timeline: Vec<CSObjectState>,
    init_timestep: usize,
    timestep: usize,

    #[getset(get = "pub")]
    shape: DrawShapeType,
    #[getset(get = "pub")]
    hide: bool,
    #[getset(get = "pub")]
    attached: Option<AttachedFn>,
}

impl Default for CSimObject {
    fn default() -> Self {
        Self {
            state_timeline: vec![],
            init_timestep: 0,
            timestep: 0,
            shape: DrawShapeType::Circle,
            hide: false,
            attached: None,
        }
    }
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

#[repr(usize)]
pub enum ForceIndex {
    Attached = 0,
    UserInteraction = 1,
    MAX = 2,
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

pub struct CSObjectStateBuilder {
    state: CSObjectState,
}

impl CSObjectStateBuilder {
    pub fn new() -> Self {
        Self {
            state: Default::default(),
        }
    }

    pub fn from_state(state: CSObjectState) -> Self {
        Self { state }
    }

    pub fn position(&mut self, position: NVec2) -> &mut Self {
        self.state.position = position;
        self
    }

    pub fn velocity(&mut self, velocity: NVec2) -> &mut Self {
        self.state.velocity = velocity;
        self
    }

    pub fn mass(&mut self, mass: f64) -> &mut Self {
        self.state.mass = mass;
        self
    }

    pub fn build(self) -> CSObjectState {
        self.state
    }
}
