

use eframe::epaint::Color32;
use egui::plot::{Arrows, Line, PlotImage, PlotUi, Points, Polygon, Text};

pub mod items;

pub enum PlotColor {
    Object,
    VelocityVector,
    ForceVector,
    SigmaForceVector,
    TraceLine,
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
            PlotColor::Stamp => Color32::YELLOW,
            PlotColor::StampText => Color32::GREEN,
        }
    }
}

pub enum PlotTextSize {
    Small,
    Medium,
    Large,
}

impl PlotTextSize {
    pub fn get_size(&self) -> f64 {
        match self {
            PlotTextSize::Small => 1.0,
            PlotTextSize::Medium => 2.0,
            PlotTextSize::Large => 5.0,
        }
    }
}


pub enum PlotItem{
    Points(Points),
    Polygon(Polygon),
    Arrows(Arrows),
    Line(Line),
    Text(Text),
    PlotImage(PlotImage),
}

impl PlotItem{
    pub fn draw(self, plot_ui: &mut PlotUi){
        match self {
            PlotItem::Points(points) => {
                plot_ui.points(points)
            }
            PlotItem::Polygon(polygon) => {
                plot_ui.polygon(polygon)
            }
            PlotItem::Arrows(arrows) => {
                plot_ui.arrows(arrows)
            }
            PlotItem::Line(line) => {
                plot_ui.line(line)
            }
            PlotItem::Text(text) => {
                plot_ui.text(text)
            }
            PlotItem::PlotImage(plot_image) => {
                plot_ui.image(plot_image)
            }
        }
    }
}

impl From<Points> for PlotItem{
    fn from(points: Points) -> Self {
        PlotItem::Points(points)
    }
}

impl From<Polygon> for PlotItem{
    fn from(polygon: Polygon) -> Self {
        PlotItem::Polygon(polygon)
    }
}

impl From<Arrows> for PlotItem{
    fn from(arrows: Arrows) -> Self {
        PlotItem::Arrows(arrows)
    }
}

impl From<Line> for PlotItem{
    fn from(line: Line) -> Self {
        PlotItem::Line(line)
    }
}

impl From<Text> for PlotItem{
    fn from(text: Text) -> Self {
        PlotItem::Text(text)
    }
}

