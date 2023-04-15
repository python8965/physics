use crate::app::graphics::define::BoxedPlotDraw;
use crate::app::simulations::classic_simulation::template::stamp::CSObjectStamp;

#[derive(Debug, Clone, Default)]
pub struct CSPlotObjects {
    pub static_item_func: Vec<fn() -> Vec<BoxedPlotDraw>>,
    pub stamps: Vec<CSObjectStamp>,
}

impl CSPlotObjects {
    pub fn add_static_item(mut self, item: fn() -> Vec<BoxedPlotDraw>) -> Self {
        self.static_item_func.push(item);
        self
    }

    pub fn add_stamp(mut self, stamp: CSObjectStamp) -> Self {
        self.stamps.push(stamp);
        self
    }

    pub fn get_plot_items(&self) -> Vec<BoxedPlotDraw> {
        self.static_item_func.iter().fold(vec![], |mut acc, func| {
            acc.extend(func());
            acc
        })
    }

    pub fn get_stamps(&mut self) -> &mut Vec<CSObjectStamp> {
        &mut self.stamps
    }
}
