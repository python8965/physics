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

#[derive(Clone, Copy, Debug, Default)]
pub struct CSimState {
    pub(crate) time: f64,
    pub(crate) settings: PlotViewFilter,
    pub(crate) pointer: Option<PlotPoint>,
    pub(crate) zoom: f64,
}

impl CSimState {
    pub fn is_sim_started(&self) -> bool {
        !(self.time == 0.0)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct PlotViewFilter {
    pub(crate) force: bool,
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
            force: false,
            sigma_force: true,
            velocity: true,
            trace: true,
            text: false,
            stamp: true,
            equation: true,
        }
    }
}
