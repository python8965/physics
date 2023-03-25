use egui::plot::{PlotPoint, PlotUi};

pub fn update_simulation_state(state: &mut SimulationState, plot_ui: &mut PlotUi) {
    let zoom = plot_ui.plot_bounds().width();
    let pointer = plot_ui.pointer_coordinate();

    *state = SimulationState {
        zoom,
        pointer,
        ..*state
    };
}

#[derive(Clone, Copy, Debug, Default)]
pub struct SimulationState {
    pub(crate) time: f64,
    pub(crate) filter: PlotInfoFilter,
    pub(crate) pointer: Option<PlotPoint>,
    pub(crate) zoom: f64,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct PlotInfoFilter {
    pub(crate) force: bool,
    pub(crate) sigma_force: bool,
    pub(crate) velocity: bool,
    pub(crate) trace: bool,
    pub(crate) text: bool,
}

impl Default for PlotInfoFilter {
    fn default() -> Self {
        Self {
            force: false,
            sigma_force: true,
            velocity: true,
            trace: true,
            text: false,
        }
    }
}
