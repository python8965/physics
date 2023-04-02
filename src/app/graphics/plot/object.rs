use crate::app::graphics::define::PlotDrawItem;
use crate::app::simulations::classic_simulation::template::stamp::CSObjectStamp;

pub struct CSPlotObjects {
    pub static_item_func: Vec<fn() -> Vec<PlotDrawItem>>,
    pub stamps: Vec<CSObjectStamp>,
}

impl Default for CSPlotObjects {
    fn default() -> Self {
        Self {
            static_item_func: vec![],
            stamps: vec![],
        }
    }
}

impl CSPlotObjects {
    pub fn add_static_item(mut self, item: fn() -> Vec<PlotDrawItem>) -> Self {
        self.static_item_func.push(item);
        self
    }

    pub fn add_stamp(mut self, stamp: CSObjectStamp) -> Self {
        self.stamps.push(stamp);
        self
    }

    pub fn get_plot_items(&self) -> Vec<PlotDrawItem> {
        self.static_item_func.iter().fold(vec![], |mut acc, func| {
            acc.extend(func());
            acc
        })
    }

    pub fn get_stamps(&mut self) -> &mut Vec<CSObjectStamp> {
        &mut self.stamps
    }
}
