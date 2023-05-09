use egui::plot::{GridInput, GridMark, Legend, Plot};
use egui::{ScrollArea, Slider, Widget};
use nalgebra::Vector2;

use crate::app::audio::player::MusicPlayer;
use crate::app::graphics::image::ImageManager;
use crate::app::graphics::plot::InputMessage;
use crate::app::manager::SimulationManager;
use crate::app::simulations::classic_simulation::template::get_sim_list;
use crate::app::util::FrameHistory;

mod audio;
mod graphics;
mod io;
pub mod manager;
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
        let ctx = &_cc.egui_ctx;

        new_with_context(ctx);
        Self {
            ..Default::default()
        }
    }
}

fn new_with_context(ctx: &egui::Context) {
    ctx.set_visuals(egui::Visuals::dark()); // always dark mode
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

        puffin::profile_function!();
        puffin::GlobalProfiler::lock().new_frame(); // call once per frame!
        puffin_egui::profiler_window(ctx);

        self.simulation_manager.step();

        self.frame_history.on_new_frame(current_time, cpu_usage);

        let memo = "caucation !! in-simulation-pos is not exactly same with calculated pos_{{final}}\n\
                          Because we can only get the simulation time discretely instead of continuously,\n\
                          and there may be an error in floating point operations.\n\n\
                          And you'll also see that the equation doesn't match if you give it your own strength.\n\
                          This is because this formula is only valid in situations of equal acceleration(ΣF=ma).\n\
                          ";

        if self.simulation_manager.is_initializing() {
            egui::SidePanel::right("Control Panel").show(ctx, |ui| {
                ScrollArea::new([false, true]).show(ui, |ui| {
                    ui.heading("Initializing Simulation...");
                    ui.label("Click resume to start the simulation.");
                    ui.separator();

                    self.simulation_manager.initialize_ui(ui);
                });
            });
        } else {
            egui::SidePanel::right("Inspection").show(ctx, |ui| {
                ScrollArea::new([false, true]).show(ui, |ui| {
                    ui.heading("Inspection");
                    ui.label("Click Object to inspect it.");
                    ui.separator();

                    self.simulation_manager.inspection_ui(ui);
                });
            });
        }

        egui::TopBottomPanel::top("Top Panel").show(ctx, |ui| {
            self.simulation_manager.operation_ui(ui);
        });

        egui::TopBottomPanel::bottom("Bottom Panel").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label("Timeline");
                let length = self.simulation_manager.timestep();

                ui.spacing_mut().slider_width = (length as f32 / 2.0 + 100.0).clamp(100.0, 500.0);
                if Slider::new(self.simulation_manager.current_timestep_mut(), 0..=length)
                    .ui(ui)
                    .dragged()
                {
                    self.simulation_manager.timestep_changed();
                }

                if ui.add_enabled(
                    self.simulation_manager.timestep() != 0,
                    egui::Button::new("Prev")
                ).clicked() {
                    *self.simulation_manager.current_timestep_mut() -= 1;
                    self.simulation_manager.timestep_changed();
                }

                if ui.add_enabled(
                    true,
                    egui::Button::new("Next")
                ).clicked() {
                    *self.simulation_manager.current_timestep_mut() += 1;
                    self.simulation_manager.timestep_changed();
                }
            });
        });

        egui::SidePanel::left("Simulation Type").show(ctx, |ui| {
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

                ui.with_layout(egui::Layout::top_down(egui::Align::LEFT), |ui| {
                    ScrollArea::new([false, true]).show(ui, |ui| {
                        ui.collapsing("Program Info", |ui| {
                            ui.label(
                                "Control Info\n\
                                       .    drag      /      zoom    \n\
                                       PC:  just drag / ctrl + scroll\n\
                                       Mobile: touch  /  pinch\n\
                                       ",
                            );

                            ui.selectable_label(true, "Simulation Info")
                                .on_hover_text(memo);

                            ui.label(
                                "Update Info : Input Shift + F5 (only desktop) \n\
                                                         use Secret(Private) Browser",
                            );

                            ui.horizontal(|ui| {
                                ui.spacing_mut().item_spacing.x = 0.0;
                                ui.hyperlink_to(
                                    "source code link (github)",
                                    "https://github.com/python8965/physics",
                                );
                                ui.label(".");
                            });
                        });

                        ui.separator();

                        ui.label(format!("fps : {:.0?}", self.frame_history.fps()));

                        ui.label(format!(
                            "At Time (ΣΔt) = {:.2?}",
                            self.simulation_manager.get_time()
                        ));

                        ui.horizontal(|ui| {
                            ui.label("Time mul");
                            let _slider =
                                Slider::new(self.simulation_manager.time_multiplier(), 1..=5)
                                    .ui(ui);
                        });

                        ui.separator();

                        self.simulation_manager.settings_ui(ui);

                        ui.separator();

                        ui.horizontal(|ui| {
                            ctx.input(|i| {
                                if i.key_pressed(egui::Key::Space) {
                                    self.simulation_manager.toggle_pause();
                                }
                            });

                            let paused = self.simulation_manager.get_pause();
                            let text = if paused { "Resume" } else { "Pause" };

                            if ui.selectable_label(paused, text).clicked() {
                                self.simulation_manager.toggle_pause();
                            }
                        });

                        let _buttons = get_sim_list()
                            .iter()
                            .map(|sim_type| {
                                let button = ui.button(sim_type.get_name());

                                if button.clicked() {
                                    self.simulation_manager.new_simulation(sim_type.clone());
                                }

                                button
                            })
                            .collect::<Vec<_>>();

                        ui.separator();

                        ui.collapsing("Laboratory", |ui| {
                            self.music_player.ui(ui);
                        });

                        ui.separator();

                        // TODO: Source Code Demonstrate
                        // if ui.button("source code of current app").clicked() {
                        //     egui::Window::new("Source Code").show(ctx, |ui| {
                        //         ui.label(format!(
                        //             "{:?}",
                        //             self.simulation_manager.get_simulation_type()
                        //         ));
                        //     });
                        // }
                    });
                });
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's
            puffin::profile_scope!("Plot");
            // egui::widgets::global_dark_light_mode_switch(ui);
            //
            // egui::Frame::canvas(ui.style()).show(ui, |ui| {
            //     puffin::profile_scope!("Plot::show");
            // });

            fn no_spacer(_: GridInput) -> Vec<GridMark> {
                vec![]
            }

            fn default_spacer(grid_input: GridInput) -> Vec<GridMark> {
                let log_base = 10.0;

                fn next_power(value: f64, base: f64) -> f64 {
                    assert_ne!(value, 0.0); // can be negative (typical for Y axis)
                    base.powi(value.abs().log(base).ceil() as i32)
                }

                fn fill_marks_between(
                    out: &mut Vec<GridMark>,
                    step_size: f64,
                    (min, max): (f64, f64),
                ) {
                    assert!(max > min);
                    let first = (min / step_size).ceil() as i64;
                    let last = (max / step_size).ceil() as i64;

                    let marks_iter = (first..last).map(|i| {
                        let value = (i as f64) * step_size;
                        GridMark { value, step_size }
                    });
                    out.extend(marks_iter);
                }

                fn generate_marks(step_sizes: [f64; 3], bounds: (f64, f64)) -> Vec<GridMark> {
                    let mut steps = vec![];
                    fill_marks_between(&mut steps, step_sizes[0], bounds);
                    fill_marks_between(&mut steps, step_sizes[1], bounds);
                    fill_marks_between(&mut steps, step_sizes[2], bounds);
                    steps
                }

                let smallest_visible_unit = next_power(grid_input.base_step_size, log_base) * 5.0;

                let step_sizes = [
                    smallest_visible_unit,
                    smallest_visible_unit * log_base,
                    smallest_visible_unit * log_base * log_base,
                ];

                generate_marks(step_sizes, grid_input.bounds)
            }

            egui::warn_if_debug_build(ui);

            if let (Some(simulation), simulation_plot, state, debug_store) =
                self.simulation_manager.get_simulation()
            {
                let legend = Legend::default();
                let mut plot = Plot::new("Plot")
                    .allow_boxed_zoom(false)
                    .data_aspect(1.0)
                    .allow_double_click_reset(false)
                    .legend(legend);

                if state.settings.grid {
                    plot = plot.x_grid_spacer(default_spacer);
                    plot = plot.y_grid_spacer(default_spacer);
                } else {
                    plot = plot.x_grid_spacer(no_spacer);
                    plot = plot.y_grid_spacer(no_spacer);
                }

                if simulation_plot.is_dragging_object() {
                    plot = plot.allow_drag(false)
                } else {
                    plot = plot.allow_drag(true)
                }

                let response = plot.show(ui, |plot_ui| {
                    state.update_simulation_state(plot_ui);
                    simulation_plot.draw(simulation, plot_ui, state, debug_store);

                    InputMessage {
                        clicked: plot_ui.plot_clicked(),
                        hovered: plot_ui.plot_hovered(),
                        pointer_pos: plot_ui.pointer_coordinate(),
                    }
                });

                simulation_plot.input(simulation, response, ui.ctx(), state);
            }
        });

        ctx.request_repaint();
    }
}
