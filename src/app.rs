use egui::plot::{CoordinatesFormatter, Corner, Line, Plot, PlotPoints};
use egui::{Slider, Widget};
use log::debug;

use crate::simulation::manager::SimulationManager;
use crate::simulation::template::SIM;

pub struct State {
    simulation_manager: SimulationManager,
}

impl Default for State {
    fn default() -> Self {
        Self {
            // Example stuff:
            simulation_manager: SimulationManager::default(),
        }
    }
}

impl State {
    /// Called once before the first frame.
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.

        // if let Some(storage) = cc.storage {
        //     return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        // }

        Default::default()
    }
}

impl eframe::App for State {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let current_time = ctx.input(|i| i.time);
        self.simulation_manager.step(current_time);

        egui::SidePanel::left("Simulation Type").show(ctx, |ui| {
            ui.collapsing("CONTROL INFO (click)", |ui| {
                ui.label("Mouse drag : move chart\nCtrl + Drag : zoom")
            });

            ui.separator();
            ui.collapsing("Drawing Filter", |ui| {
                ui.checkbox(&mut self.simulation_manager.filter_mut().text, "text");
                ui.checkbox(&mut self.simulation_manager.filter_mut().force, "force");
                ui.checkbox(
                    &mut self.simulation_manager.filter_mut().velocity,
                    "velocity",
                );
                ui.checkbox(
                    &mut self.simulation_manager.filter_mut().sigma_force,
                    "sigma_force",
                );
                ui.checkbox(&mut self.simulation_manager.filter_mut().trace, "trace");
            });

            ui.separator();

            ui.label(format!(
                "Elapsed Time (ΣΔt) = {:.2?}",
                self.simulation_manager.get_time()
            ));
            ui.horizontal(|ui| {
                ui.label("Time mul");
                let slider =
                    Slider::new(self.simulation_manager.time_multiplier(), 0.5..=2.0).ui(ui);
            });

            ui.separator();

            ui.vertical(|ui| {
                ui.horizontal(|ui| {
                    if ui.small_button("Pause/Resume").clicked() {
                        self.simulation_manager.toggle_animation();
                    }
                });

                let _buttons = SIM
                    .iter()
                    .map(|sim_type| {
                        let button = ui.button(sim_type.as_str());

                        if button.clicked() {
                            self.simulation_manager.new_simulation(*sim_type);
                        }

                        button
                    })
                    .collect::<Vec<_>>();

                // TODO: Source Code Demonstrate
                // if ui.button("source code of current simulation").clicked() {
                //     egui::Window::new("Source Code").show(ctx, |ui| {
                //         ui.label(format!(
                //             "{:?}",
                //             self.simulation_manager.get_simulation_type()
                //         ));
                //     });
                // }
            });

            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                ui.horizontal(|ui| {
                    ui.spacing_mut().item_spacing.x = 0.0;
                    ui.label("made by ");
                    ui.hyperlink_to("python8965", "https://github.com/python8965");
                    ui.label(".");
                });

                ui.horizontal(|ui| {
                    ui.spacing_mut().item_spacing.x = 0.0;
                    ui.label("powered by ");
                    ui.hyperlink_to("egui", "https://github.com/emilk/egui");
                    ui.label(" and ");
                    ui.hyperlink_to(
                        "eframe",
                        "https://github.com/emilk/egui/tree/master/crates/eframe",
                    );
                    ui.label(".");
                });
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            let state = self.simulation_manager.get_state();
            // The central panel the region left after adding TopPanel's and SidePanel's
            if let Some(simulation) = self.simulation_manager.get_simulation() {
                let _plot = Plot::new("Plot")
                    .allow_boxed_zoom(false)
                    .view_aspect(1.0)
                    .show(ui, |plot_ui| {
                        simulation.draw(plot_ui, state);

                        plot_ui.line(Line::new(PlotPoints::new(vec![
                            [-100.0, -100.0],
                            [-100.0, 100.0],
                            [100.0, 100.0],
                            [100.0, -100.0],
                            [-100.0, -100.0],
                        ])))
                    });
            }

            egui::warn_if_debug_build(ui);
        });

        ctx.request_repaint();
    }
}
