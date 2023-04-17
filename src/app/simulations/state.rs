use crate::app::simulations::classic_simulation::state::CSimSettings;
use egui::plot::{PlotPoint, PlotUi};
use paste::paste;

macro_rules! cast {
    ($target: expr, $pat: path) => {{
        if let $pat(a) = $target {
            Some(a)
        } else {
            None
        }
    }};
}

macro_rules! cast_impl_inner {
    ( $name:ident {$($body:tt)*} ($variant:ident) $($tail:tt)* ) => {
        cast_impl_inner!{
            $name// Enum name
            {
                $($body)*  // Previously-built variants
                paste! {
                    pub fn [<as_ $variant:snake>](&self) -> Option<&$variant>{
                        cast!(self, SimulationSettings::$variant)
                    }

                    pub fn [<as_ $variant:snake _mut>](&mut self) -> Option<&mut $variant>{
                        cast!(self, SimulationSettings::$variant)
                    }
                }
            }
            $($tail)* // Unprocessed variants
        }
    };

    // When there are no more variants, emit the enum definition
    ( $name:ident {$($body:tt)*} ) => {
        impl $name { $($body)* }
    };
}

macro_rules! cast_impl {
    ( $name:ident,$($variants:ident),* ) => {
        cast_impl_inner!{ $name {} $(($variants))* }
    }
}
#[derive(Clone, Copy, Debug)]
pub struct BSimSettings {}

cast_impl!(SimulationSettings, CSimSettings);

#[derive(Clone, Debug)]
pub enum SimulationSettings {
    CSimSettings(CSimSettings),
    BSimSettings(BSimSettings),
    None,
}

impl Default for SimulationSettings {
    fn default() -> Self {
        Self::None
    }
}

impl SimulationSettings {
    pub fn ui(&mut self, ui: &mut egui::Ui) {
        match self {
            Self::None => {}
            Self::CSimSettings(settings) => {
                settings.ui(ui);
            }
            SimulationSettings::BSimSettings(_b) => {}
        }
    }
}

#[derive(Clone, Debug)]
pub struct SimulationState {
    pub settings: SimulationSettings,
    pub(crate) pointer: Option<PlotPoint>,
    pub(crate) time_mul: usize,
    pub(crate) time: f64,
    pub(crate) current_step: usize,
    pub(crate) max_step: usize,
    pub(crate) sim_started: bool,
    pub(crate) zoom: f64,
}

impl Default for SimulationState {
    fn default() -> Self {
        Self {
            pointer: None,
            settings: SimulationSettings::None,
            time: 0.0,
            current_step: 0,
            max_step: 0,
            sim_started: false,
            zoom: 1.0,
            time_mul: 1,
        }
    }
}

impl SimulationState {
    pub(crate) fn update_simulation_state(&mut self, plot_ui: &mut PlotUi) {
        let zoom = plot_ui.plot_bounds().width();
        let pointer = plot_ui.pointer_coordinate();

        self.zoom = zoom;
        self.pointer = pointer;
    }

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
