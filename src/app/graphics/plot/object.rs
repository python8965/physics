use crate::app::graphics::define::PlotItem;
use crate::app::simulations::classic_simulation::template::stamp::CSObjectStamp;

#[derive(Debug, Clone, Default)]
pub struct CSPlotObjects {
    //TODO: PlotItemGenerator 단일화 하기.
    pub static_item_func: Vec<fn() -> Vec<PlotItem>>,
    pub stamps: Vec<CSObjectStamp>,
}

impl CSPlotObjects {
    pub fn add_static_item(mut self, item: fn() -> Vec<PlotItem>) -> Self {
        self.static_item_func.push(item);
        self
    }

    pub fn add_stamp(mut self, stamp: CSObjectStamp) -> Self {
        self.stamps.push(stamp);
        self
    }

    pub fn get_plot_items(&self) -> Vec<PlotItem> {
        self.static_item_func.iter().fold(vec![], |mut acc, func| {
            acc.extend(func());
            acc
        })
    }

    pub fn get_stamps(&mut self) -> &mut Vec<CSObjectStamp> {
        &mut self.stamps
    }
}
