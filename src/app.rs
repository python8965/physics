use crate::simulation::{Simulation, SIM};
use egui::mutex::Mutex;
use egui::plot::{CoordinatesFormatter, Corner, Line, Plot, PlotPoints, PlotUi, Polygon};
use std::sync::Arc;
use std::time::Duration;

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
// if we add new fields, give them default values when deserializing old state
pub struct State {
    simulation: Option<Arc<Mutex<Simulation>>>,
}

impl Default for State {
    fn default() -> Self {
        Self {
            // Example stuff:
            simulation: None,
        }
    }
}

impl State {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
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
    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::SidePanel::left("Simulation Type").show(ctx, |ui| {
            ui.heading("Side Panel");

            ui.horizontal(|ui| {
                let buttons = SIM
                    .iter()
                    .map(|sim_type| {
                        let button = ui.button(sim_type.as_str());

                        if button.clicked() {
                            let old = self
                                .simulation
                                .replace(Arc::from(Mutex::new(sim_type.as_func())));

                            if let Some(mut sim) = old {
                                sim.lock().finish();
                            }

                            let sim_clone = self.simulation.clone().unwrap();

                            if self.simulation.is_some() {
                                std::thread::spawn(move || {
                                    let dt = 0.5;

                                    loop {
                                        {
                                            let mut sim = sim_clone.lock();
                                            sim.step(dt);
                                        }
                                        std::thread::sleep(Duration::from_secs_f32(dt));
                                    }
                                });
                            }
                        }

                        button
                    })
                    .collect::<Vec<_>>();
            });

            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
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
            // The central panel the region left after adding TopPanel's and SidePanel's
            if let Some(simulation) = &mut self.simulation {
                let mut simulation = simulation.lock();

                let plot = Plot::new("Plot")
                    .include_x(100.0)
                    .include_x(-100.0)
                    .allow_boxed_zoom(false)
                    .view_aspect(1.0)
                    .show(ui, |plot_ui| {
                        simulation.draw(plot_ui);

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
    }
}
