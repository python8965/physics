

use eframe::epaint::Color32;
use egui::plot::{Arrows, Line, PlotImage, PlotUi, Points, Polygon, Text};

pub mod items;

pub enum PlotColor {
    Object,
    VelocityVector,
    ForceVector,
    SigmaForceVector,
    TraceLine,
    Equation,
    Stamp,
    StampText,
}

impl PlotColor {
    pub fn get_color(&self) -> Color32 {
        match self {
            PlotColor::Object => Color32::GRAY,
            PlotColor::VelocityVector => Color32::BLUE,
            PlotColor::ForceVector => Color32::RED,
            PlotColor::SigmaForceVector => Color32::DARK_RED,
            PlotColor::TraceLine => Color32::GOLD,
            PlotColor::Equation => Color32::WHITE,
            PlotColor::Stamp => Color32::YELLOW,
            PlotColor::StampText => Color32::GREEN,
        }
    }
}

pub enum PlotTextSize {
    Small,
    Medium,
    Large,
    Equation,
}

impl PlotTextSize {
    pub fn get_size(&self) -> f64 {
        match self {
            PlotTextSize::Small => 1.0,
            PlotTextSize::Medium => 2.0,
            PlotTextSize::Large => 5.0,
            PlotTextSize::Equation => 10.0,
        }
    }
}

pub type BoxedPlotDraw = Box<dyn PlotDraw>;

pub trait PlotDraw {
    fn draw(self: Box<Self>, plot_ui: &mut PlotUi);
}

impl PlotDraw for Points {
    fn draw(self: Box<Self>, plot_ui: &mut PlotUi) {
        plot_ui.points(*self);
    }
}

impl PlotDraw for Polygon {
    fn draw(self: Box<Self>, plot_ui: &mut PlotUi) {
        plot_ui.polygon(*self);
    }
}

impl PlotDraw for Arrows {
    fn draw(self: Box<Self>, plot_ui: &mut PlotUi) {
        plot_ui.arrows(*self);
    }
}

impl PlotDraw for Line {
    fn draw(self: Box<Self>, plot_ui: &mut PlotUi) {
        plot_ui.line(*self);
    }
}

impl PlotDraw for Text {
    fn draw(self: Box<Self>, plot_ui: &mut PlotUi) {
        plot_ui.text(*self);
    }
}
impl PlotDraw for PlotImage {
    fn draw(self: Box<Self>, plot_ui: &mut PlotUi) {
        plot_ui.image(*self);
    }
}

pub trait PlotDrawHelper {
    fn draw(self, plot_ui: &mut PlotUi);
}

impl PlotDrawHelper for Points {
    fn draw(self, plot_ui: &mut PlotUi) {
        plot_ui.points(self);
    }
}

impl PlotDrawHelper for Polygon {
    fn draw(self, plot_ui: &mut PlotUi) {
        plot_ui.polygon(self);
    }
}

impl PlotDrawHelper for Arrows {
    fn draw(self, plot_ui: &mut PlotUi) {
        plot_ui.arrows(self);
    }
}

impl PlotDrawHelper for Line {
    fn draw(self, plot_ui: &mut PlotUi) {
        plot_ui.line(self);
    }
}

impl PlotDrawHelper for Text {
    fn draw(self, plot_ui: &mut PlotUi) {
        plot_ui.text(self);
    }
}
impl PlotDrawHelper for PlotImage {
    fn draw(self, plot_ui: &mut PlotUi) {
        plot_ui.image(self);
    }
}
