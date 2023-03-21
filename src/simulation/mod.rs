use egui::plot::{Arrows, Line, PlotUi, Polygon, Text};
use egui::Color32;

use std::fmt::{Debug, Formatter};

pub mod drawing;
pub mod engine;
pub mod manager;
pub mod object;
pub mod template;

pub type Float = f64;
pub type Vec2 = nalgebra::Vector2<f64>;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum DrawShapeType {
    Circle,
    Box,
}

pub enum PlotDrawItem {
    Polygon(Polygon),
    Arrows(Arrows), // Arrows with debug text
    Text(Text),
    Line(Line),
}

pub enum PlotVectorType {
    Velocity,
    Force,
    SigmaForce,
}

impl PlotVectorType {
    pub fn to_color(&self) -> Color32 {
        match self {
            PlotVectorType::Velocity => Color32::DARK_RED,
            PlotVectorType::Force => Color32::DEBUG_COLOR,
            PlotVectorType::SigmaForce => Color32::GREEN,
        }
    }
}

impl Debug for PlotDrawItem {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            PlotDrawItem::Polygon(_) => "DrawShape",
            PlotDrawItem::Arrows(_) => "Arrow",
            PlotDrawItem::Text(_) => "Text",
            PlotDrawItem::Line(_) => "Line",
        })
    }
}

impl PlotDrawItem {
    fn draw(self, plot_ui: &mut PlotUi) {
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
        }
    }
}
