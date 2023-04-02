use std::fmt::{Debug, Formatter};

use eframe::epaint::Color32;
use egui::plot::{
    Arrows, Line, LineStyle, PlotImage, PlotPoint, PlotPoints, PlotUi, Polygon, Text,
};
use egui::Stroke;

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

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum DrawShapeType {
    Circle,
    Box,
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

pub enum PlotDrawItem {
    Polygon(Polygon),
    Arrows(Arrows), // Arrows with debug text
    Line(Line),
    Text(Text),
    Image(PlotImage),
}

unsafe impl Send for PlotDrawItem {}

unsafe impl Sync for PlotDrawItem {}

impl Debug for PlotDrawItem {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            PlotDrawItem::Polygon(_) => "DrawShape",
            PlotDrawItem::Arrows(_) => "Arrow",
            PlotDrawItem::Text(_) => "Text",
            PlotDrawItem::Line(_) => "Line",
            PlotDrawItem::Image(_) => "Image",
        })
    }
}

impl PlotDrawItem {
    pub(crate) fn draw(self, plot_ui: &mut PlotUi) {
        match self {
            PlotDrawItem::Polygon(polygon) => {
                plot_ui.polygon(polygon);
            }
            PlotDrawItem::Arrows(arrows) => {
                plot_ui.arrows(arrows);
            }
            PlotDrawItem::Text(text) => {
                plot_ui.text(text);
            }
            PlotDrawItem::Line(line) => {
                plot_ui.line(line);
            }
            PlotDrawItem::Image(image) => {
                plot_ui.image(image);
            }
        }
    }
}
