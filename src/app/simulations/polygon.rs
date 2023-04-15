use egui::plot::PlotPoint;

pub fn is_inside(pos: PlotPoint, plotpoint: &[PlotPoint]) -> bool {
    let mut contact = 0;
    for (p1, p2) in plotpoint.windows(2).map(|x| (x[0], x[1])) {
        if (pos.y > p1.y) != (pos.y > p2.y) {
            let at_x = (p2.x - p1.x) * (pos.y - p1.y) / (p2.y - p1.y) + p1.x;

            if at_x > pos.x {
                contact += 1;
            }
        }
    }
    contact % 2 > 0
}
