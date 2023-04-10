use egui::plot::{PlotPoint, PlotUi};

pub fn update_simulation_state(state: &mut CSimState, plot_ui: &mut PlotUi) {
    let zoom = plot_ui.plot_bounds().width();
    let pointer = plot_ui.pointer_coordinate();

    *state = CSimState {
        zoom,
        pointer,
        ..*state
    };
}

#[derive(Clone, Copy, Debug)]
pub struct CSimState {
    pub(crate) settings: PlotViewFilter,
    pub(crate) pointer: Option<PlotPoint>,
    pub(crate) gravity: bool,
    pub(crate) time_mul: f64,
    pub(crate) time: f64,
    pub(crate) current_step: usize,
    pub(crate) max_step: usize,
    pub(crate) sim_started: bool,
    pub(crate) zoom: f64,
}

impl Default for CSimState {
    fn default() -> Self {
        Self {
            time: 0.0,
            current_step: 0,
            max_step: 0,
            settings: PlotViewFilter::default(),
            sim_started: false,
            pointer: None,
            zoom: 1.0,
            time_mul: 1.0,
            gravity: true,
        }
    }
}

impl CSimState {
    pub fn is_sim_started(&self) -> bool {
        self.sim_started
    }

    pub fn reset(&mut self) {
        self.time = 0.0;
        self.sim_started = false;
        self.current_step = 0;
        self.max_step = 0;
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct PlotViewFilter {
    pub(crate) acceleration: bool,
    pub(crate) sigma_force: bool,
    pub(crate) velocity: bool,
    pub(crate) trace: bool,
    pub(crate) text: bool,
    pub(crate) stamp: bool,
    pub(crate) equation: bool,
}

impl Default for PlotViewFilter {
    fn default() -> Self {
        Self {
            acceleration: true,
            sigma_force: false,
            velocity: true,
            trace: true,
            text: false,
            stamp: true,
            equation: true,
        }
    }
}
