use crate::app::audio::player::MusicPlayer;
use egui::plot::{Legend, Plot};
use egui::{Slider, Widget};
use nalgebra::Vector2;

use crate::app::manager::SimulationManager;
use crate::app::simulations::state::update_simulation_state;
use crate::app::simulations::template::SIM;
use crate::app::util::FrameHistory;

mod audio;
mod graphics;
mod init_manager;
mod manager;
mod simulations;
mod util;

pub type Float = f64;
pub type NVec2 = Vector2<Float>;

#[derive(Default)]
pub struct State {
    simulation_manager: SimulationManager,
    music_player: MusicPlayer,
    frame_history: FrameHistory,
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

        setup_custom_fonts(&_cc.egui_ctx);

        Default::default()
    }
}

fn setup_custom_fonts(_ctx: &egui::Context) {
    // Start with the default fonts (we will be adding to them rather than replacing them).
    // let mut fonts = egui::FontDefinitions::default();

    // // Install my own font (maybe supporting non-latin characters).
    // // .ttf and .otf files supported.
    // fonts.font_data.insert(
    //     "my_font".to_owned(),
    //     egui::FontData::from_static(include_bytes!("C:\\Windows\\fonts\\Malgun.ttf")),
    // );
    //
    // // Put my font first (highest priority) for proportional text:
    // fonts
    //     .families
    //     .entry(egui::FontFamily::Proportional)
    //     .or_default()
    //     .insert(0, "my_font".to_owned());
    //
    // // Put my font as last fallback for monospace:
    // fonts
    //     .families
    //     .entry(egui::FontFamily::Monospace)
    //     .or_default()
    //     .push("my_font".to_owned());
    //
    // // Tell egui to use these fonts:
    // ctx.set_fonts(fonts);

    // let mut style = (*ctx.style()).clone();
    // style.text_styles = [
    //     (Heading, FontId::new(30.0, Proportional)),
    //     (Body, FontId::new(18.0, Proportional)),
    //     (Monospace, FontId::new(14.0, Proportional)),
    //     (Button, FontId::new(14.0, Proportional)),
    //     (Small, FontId::new(10.0, Proportional)),
    // ]
    // .into();
    // ctx.set_style(style);
}

impl eframe::App for State {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        let current_time = ctx.input(|i| i.time);
        let cpu_usage = frame.info().cpu_usage;

        self.simulation_manager.step(current_time);
        self.frame_history.on_new_frame(current_time, cpu_usage);
        egui::SidePanel::left("Simulation Type").show(ctx, |ui| {
            ui.collapsing("CONTROL INFO (click)", |ui| {
                ui.label("Mouse drag : move chart\nCtrl + Drag : zoom")
            });

            ui.separator();
            ui.collapsing("Drawing Filter", |ui| {
                ui.checkbox(&mut self.simulation_manager.settings_mut().text, "text");
                ui.checkbox(&mut self.simulation_manager.settings_mut().force, "force");
                ui.checkbox(
                    &mut self.simulation_manager.settings_mut().velocity,
                    "velocity",
                );
                ui.checkbox(
                    &mut self.simulation_manager.settings_mut().sigma_force,
                    "sigma_force",
                );
                ui.checkbox(&mut self.simulation_manager.settings_mut().trace, "trace");
            });

            ui.separator();

            ui.label(format!("fps : {:.0?}", self.frame_history.fps()));

            ui.label(format!(
                "Elapsed Time (ΣΔt) = {:.2?}",
                self.simulation_manager.get_time()
            ));

            ui.horizontal(|ui| {
                ui.label("Time mul");
                let _slider =
                    Slider::new(self.simulation_manager.time_multiplier(), 0.5..=4.0).ui(ui);
            });

            if ui.button("button").clicked() {
                self.music_player.play_audio();
            }

            ui.separator();

            ui.horizontal(|ui| {
                let paused = self.simulation_manager.get_pause();
                let text = if paused { "Resume" } else { "Pause" };

                if ui.selectable_label(paused, text).clicked() {
                    self.simulation_manager.toggle_pause();
                }
            });

            let _buttons = SIM
                .iter()
                .map(|sim_type| {
                    let button = ui.button(sim_type.get_name());

                    if button.clicked() {
                        self.simulation_manager.new_simulation(*sim_type);
                    }

                    button
                })
                .collect::<Vec<_>>();

            ui.separator();

            self.simulation_manager.initialize_ui(ui);

            // TODO: Source Code Demonstrate
            // if ui.button("source code of current app").clicked() {
            //     egui::Window::new("Source Code").show(ctx, |ui| {
            //         ui.label(format!(
            //             "{:?}",
            //             self.simulation_manager.get_simulation_type()
            //         ));
            //     });
            // }

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
            // The central panel the region left after adding TopPanel's and SidePanel's

            if let (Some(simulation), simulation_plot, state) =
                self.simulation_manager.get_simulation()
            {
                let legend = Legend::default();
                let mut plot = Plot::new("Plot")
                    .allow_boxed_zoom(false)
                    .data_aspect(1.0)
                    .allow_double_click_reset(false)
                    .legend(legend);

                if simulation_plot.is_dragging_object() {
                    plot = plot.allow_drag(false)
                } else {
                    plot = plot.allow_drag(true)
                }

                let response = plot.show(ui, |plot_ui| {
                    update_simulation_state(state, plot_ui);
                    simulation_plot.draw(simulation, plot_ui, *state);
                    plot_ui.pointer_coordinate()
                });

                simulation_plot.input(simulation, response);
            }

            egui::warn_if_debug_build(ui);
        });

        ctx.request_repaint();
    }
}
