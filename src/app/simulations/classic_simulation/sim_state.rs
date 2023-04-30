use std::fmt::Debug;

#[derive(Clone, Debug)]
pub struct ChangeNotifier<T: Debug + Clone + Copy + PartialEq> {
    value: T,
    changed: bool,
}

impl<T: Debug + Clone + Copy + PartialEq> From<T> for ChangeNotifier<T> {
    fn from(value: T) -> Self {
        Self {
            value,
            changed: false,
        }
    }
}

impl<T: Debug + Clone + Copy + PartialEq> ChangeNotifier<T> {
    pub fn get(&mut self) -> Option<T> {
        if self.changed {
            self.changed = false;
            Some(self.value)
        } else {
            None
        }
    }

    pub fn get_mut(&mut self) -> &mut T {
        &mut self.value
    }

    pub fn changed(&mut self) {
        self.changed = true;
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
    }
}
