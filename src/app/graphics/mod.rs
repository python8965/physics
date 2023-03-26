use egui::plot::{Arrows, Line, PlotUi, Polygon, Text};
use egui::Color32;

use std::fmt::{Debug, Formatter};

pub mod plotting;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
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

unsafe impl Send for PlotDrawItem {}
unsafe impl Sync for PlotDrawItem {}

pub enum PlotColor {
    Object,
    VelocityVector,
    ForceVector,
    SigmaForceVector,
    TraceLine,
}

impl PlotColor {
    pub fn get_color(&self) -> Color32 {
        match self {
            PlotColor::Object => Color32::GRAY,
            PlotColor::VelocityVector => Color32::BLUE,
            PlotColor::ForceVector => Color32::RED,
            PlotColor::SigmaForceVector => Color32::DARK_RED,
            PlotColor::TraceLine => Color32::DARK_GRAY,
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
        }
    }
}
