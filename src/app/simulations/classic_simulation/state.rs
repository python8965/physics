use std::cell::RefCell;
use std::fmt::Debug;

#[derive(Clone, Debug)]
pub struct ChangeNotifier<T: Debug + Clone + Copy> {
    value: T,
    changed: RefCell<bool>,
}

impl<T: Default + Debug + Clone + Copy> From<T> for ChangeNotifier<T> {
    fn from(value: T) -> Self {
        Self {
            value,
            changed: RefCell::from(false),
        }
    }
}

impl<T: Default + Debug + Clone + Copy> ChangeNotifier<T> {
    pub fn get(&self) -> Option<T> {
        if self.changed.replace(false) {
            Some(self.value)
        } else {
            None
        }
    }

    pub fn get_mut(&mut self) -> &mut T {
        &mut self.value
    }

    pub fn changed(&self) {
        self.changed.replace(true);
    }
}

#[derive(Clone, Debug)]
pub struct CSimSettings {
    pub(crate) plot_filter: PlotViewFilter,
    pub(crate) gravity: ChangeNotifier<bool>,
}

impl Default for CSimSettings {
    fn default() -> Self {
        Self {
            plot_filter: PlotViewFilter::default(),
            gravity: true.into(),
        }
    }
}

impl CSimSettings {
    pub fn ui(&mut self, ui: &mut egui::Ui) {
        ui.collapsing("Plot Settings", |ui| {
            self.plot_filter.ui(ui);
        });

        ui.collapsing("Simulation Settings", |ui| {
            if ui.checkbox(self.gravity.get_mut(), "Gravity?").changed() {
                self.gravity.changed();
            };
        });
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

impl PlotViewFilter {
    pub fn ui(&mut self, ui: &mut egui::Ui) {
        ui.checkbox(&mut self.acceleration, "Acceleration");
        ui.checkbox(&mut self.sigma_force, "Sigma Force");
        ui.checkbox(&mut self.velocity, "Velocity");
        ui.checkbox(&mut self.trace, "Trace");
        ui.checkbox(&mut self.text, "Text");
        ui.checkbox(&mut self.stamp, "Stamp");
        ui.checkbox(&mut self.equation, "Equation");
    }
}
