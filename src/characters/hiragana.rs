use crate::core::{draw, logic::Point};

pub fn a(pos: Point, scale: f64, thickness: u16, color: u16, delay: u32, fastforwarding: draw::Fastforwarding) {
    draw::curve(Point{x: pos.x+((5_f64*scale) as u16), y: pos.y+((30_f64*scale) as u16)}, (100_f64*scale) as u16, thickness, 0.0, color, delay, 0.0, fastforwarding);
    draw::curve(Point{x: pos.x+((55_f64*scale) as u16), y: pos.y+((10_f64*scale) as u16)}, (120_f64*scale) as u16, thickness, 110.0, color, delay, -0.35, fastforwarding);
    draw::curve(Point{x: pos.x+((85_f64*scale) as u16), y: pos.y+((50_f64*scale) as u16)}, (80_f64*scale) as u16, thickness, 90.0, color, delay, 0.4, fastforwarding);
    draw::curve(Point{x: pos.x+((54_f64*scale) as u16), y: pos.y+((124_f64*scale) as u16)}, (41_f64*scale) as u16, thickness, 150.0, color, delay, 0.68, fastforwarding);
    draw::curve(Point{x: pos.x+((13_f64*scale) as u16), y: pos.y+((117_f64*scale) as u16)}, (76_f64*scale) as u16, thickness, -140.0, color, delay, 1.45, fastforwarding);
    draw::curve(Point{x: pos.x+((53_f64*scale) as u16), y: pos.y+((54_f64*scale) as u16)}, (40_f64*scale) as u16, thickness, 0.0, color, delay, 0.3, fastforwarding);
    draw::curve(Point{x: pos.x+((90_f64*scale) as u16), y: pos.y+((66_f64*scale) as u16)}, (45_f64*scale) as u16, thickness, 30.0, color, delay, 1.05, fastforwarding);
}

pub fn i(pos: Point, scale: f64, thickness: u16, color: u16, delay: u32, fastforwarding: draw::Fastforwarding) {
    draw::curve(Point{x: pos.x+((20_f64*scale) as u16), y: pos.y+((25_f64*scale) as u16)}, (80_f64*scale) as u16, thickness, 110.0, color, delay, -0.5, fastforwarding);
    draw::curve(Point{x: pos.x+((30_f64*scale) as u16), y: pos.y+((100_f64*scale) as u16)}, (25_f64*scale) as u16, thickness, 75.0, color, delay, -1.7, fastforwarding);
    draw::curve(Point{x: pos.x+((85_f64*scale) as u16), y: pos.y+((35_f64*scale) as u16)}, (60_f64*scale) as u16, thickness, 60.0, color, delay, 0.4, fastforwarding);
}

pub fn u(pos: Point, scale: f64, thickness: u16, color: u16, delay: u32, fastforwarding: draw::Fastforwarding) {
    draw::curve(Point{x: pos.x+((28_f64*scale) as u16), y: pos.y+((10_f64*scale) as u16)}, (50_f64*scale) as u16, thickness, -5.0, color, delay, 0.3, fastforwarding);
    draw::curve(Point{x: pos.x+((13_f64*scale) as u16), y: pos.y+((45_f64*scale) as u16)}, (80_f64*scale) as u16, thickness, -15.0, color, delay, 0.3, fastforwarding);
    draw::curve(Point{x: pos.x+((93_f64*scale) as u16), y: pos.y+((50_f64*scale) as u16)}, (20_f64*scale) as u16, thickness, 5.0, color, delay, 1.2, fastforwarding);
    draw::curve(Point{x: pos.x+((99_f64*scale) as u16), y: pos.y+((70_f64*scale) as u16)}, (80_f64*scale) as u16, thickness, 110.0, color, delay, 0.5, fastforwarding);
}

pub fn e(pos: Point, scale: f64, thickness: u16, color: u16, delay: u32, fastforwarding: draw::Fastforwarding) {
    draw::curve(Point{x: pos.x+((25_f64*scale) as u16), y: pos.y+((10_f64*scale) as u16)}, (50_f64*scale) as u16, thickness, 5.0, color, delay, 0.0, fastforwarding);
    draw::curve(Point{x: pos.x+((10_f64*scale) as u16), y: pos.y+((30_f64*scale) as u16)}, (90_f64*scale) as u16, thickness, 0.0, color, delay, 0.0, fastforwarding);
    draw::curve(Point{x: pos.x+((100_f64*scale) as u16), y: pos.y+((30_f64*scale) as u16)}, (124_f64*scale) as u16, thickness, 137.0, color, delay, 0.0, fastforwarding);
    draw::curve(Point{x: pos.x+((45_f64*scale) as u16), y: pos.y+((83_f64*scale) as u16)}, (30_f64*scale) as u16, thickness, -50.0, color, delay, 1.4, fastforwarding);
    draw::curve(Point{x: pos.x+((72_f64*scale) as u16), y: pos.y+((97_f64*scale) as u16)}, (35_f64*scale) as u16, thickness, 90.0, color, delay, -1.0, fastforwarding);
}

pub fn o(pos: Point, scale: f64, thickness: u16, color: u16, delay: u32, fastforwarding: draw::Fastforwarding) {
    draw::curve(Point{x: pos.x+((5_f64*scale) as u16), y: pos.y+((22_f64*scale) as u16)}, (85_f64*scale) as u16, thickness, 0.0, color, delay, 0.0, fastforwarding);
    draw::curve(Point{x: pos.x+((50_f64*scale) as u16), y: pos.y}, (108_f64*scale) as u16, thickness, 80.0, color, delay, 0.2, fastforwarding);
    draw::curve(Point{x: pos.x+((50_f64*scale) as u16), y: pos.y+((110_f64*scale) as u16)}, (45_f64*scale) as u16, thickness, 140.0, color, delay, 0.79, fastforwarding);
    draw::curve(Point{x: pos.x+((10_f64*scale) as u16), y: pos.y+((110_f64*scale) as u16)}, (81_f64*scale) as u16, thickness, -140.0, color, delay, 1.49, fastforwarding);
    draw::curve(Point{x: pos.x+((50_f64*scale) as u16), y: pos.y+((45_f64*scale) as u16)}, (45_f64*scale) as u16, thickness, 0.0, color, delay, 0.3, fastforwarding);
    draw::curve(Point{x: pos.x+((91_f64*scale) as u16), y: pos.y+((57_f64*scale) as u16)}, (45_f64*scale) as u16, thickness, 40.0, color, delay, 1.0, fastforwarding);
    draw::curve(Point{x: pos.x+((90_f64*scale) as u16), y: pos.y+((5_f64*scale) as u16)}, (20_f64*scale) as u16, thickness, 55.0, color, delay, 0.0, fastforwarding);
}

pub fn ka(pos: Point, scale: f64, thickness: u16, color: u16, delay: u32, fastforwarding: draw::Fastforwarding) {
    // First stroke: the vertical line
    draw::curve(Point{x: pos.x+((10_f64*scale) as u16), y: pos.y+((5_f64*scale) as u16)}, (70_f64*scale) as u16, thickness, 90.0, color, delay, 0.0, fastforwarding);
    // Second stroke: the sweeping curve crossing the vertical line
    draw::curve(Point{x: pos.x+((10_f64*scale) as u16), y: pos.y+((25_f64*scale) as u16)}, (100_f64*scale) as u16, thickness, -30.0, color, delay, 0.2, fastforwarding);
    // Third stroke: the small loop in the top-right part
    draw::curve(Point{x: pos.x+((70_f64*scale) as u16), y: pos.y+((5_f64*scale) as u16)}, (30_f64*scale) as u16, thickness, 60.0, color, delay, 0.4, fastforwarding);
    // Optional refinements: finer details and adjustments can be added
    draw::curve(Point{x: pos.x+((85_f64*scale) as u16), y: pos.y+((40_f64*scale) as u16)}, (50_f64*scale) as u16, thickness, 150.0, color, delay, 0.8, fastforwarding);
}

pub fn ki(pos: Point, scale: f64, thickness: u16, color: u16, delay: u32, fastforwarding: draw::Fastforwarding) {
    // Top horizontal stroke
    draw::curve(Point{x: pos.x+((10_f64*scale) as u16), y: pos.y+((5_f64*scale) as u16)}, (60_f64*scale) as u16, thickness, 0.0, color, delay, 0.0, fastforwarding);
    // Vertical stroke
    draw::curve(Point{x: pos.x+((30_f64*scale) as u16), y: pos.y+((5_f64*scale) as u16)}, (80_f64*scale) as u16, thickness, 90.0, color, delay, 0.2, fastforwarding);
    // First curved line crossing the vertical stroke
    draw::curve(Point{x: pos.x+((10_f64*scale) as u16), y: pos.y+((25_f64*scale) as u16)}, (50_f64*scale) as u16, thickness, -30.0, color, delay, 0.4, fastforwarding);
    // Second small loop in upper-right
    draw::curve(Point{x: pos.x+((60_f64*scale) as u16), y: pos.y+((10_f64*scale) as u16)}, (20_f64*scale) as u16, thickness, 60.0, color, delay, 0.6, fastforwarding);
}

pub fn ku(pos: Point, scale: f64, thickness: u16, color: u16, delay: u32, fastforwarding: draw::Fastforwarding) {
    // Single curved stroke
    draw::curve(Point{x: pos.x+((20_f64*scale) as u16), y: pos.y+((10_f64*scale) as u16)}, (100_f64*scale) as u16, thickness, -45.0, color, delay, 0.0, fastforwarding);
}

pub fn ke(pos: Point, scale: f64, thickness: u16, color: u16, delay: u32, fastforwarding: draw::Fastforwarding) {
    // Vertical stroke
    draw::curve(Point{x: pos.x+((10_f64*scale) as u16), y: pos.y+((5_f64*scale) as u16)}, (70_f64*scale) as u16, thickness, 90.0, color, delay, 0.0, fastforwarding);
    // Horizontal top stroke
    draw::curve(Point{x: pos.x+((30_f64*scale) as u16), y: pos.y+((5_f64*scale) as u16)}, (50_f64*scale) as u16, thickness, 0.0, color, delay, 0.2, fastforwarding);
    // Curved stroke crossing the vertical
    draw::curve(Point{x: pos.x+((20_f64*scale) as u16), y: pos.y+((30_f64*scale) as u16)}, (90_f64*scale) as u16, thickness, -30.0, color, delay, 0.4, fastforwarding);
}

pub fn ko(pos: Point, scale: f64, thickness: u16, color: u16, delay: u32, fastforwarding: draw::Fastforwarding) {
    // First short horizontal stroke
    draw::curve(Point{x: pos.x+((10_f64*scale) as u16), y: pos.y+((10_f64*scale) as u16)}, (40_f64*scale) as u16, thickness, 0.0, color, delay, 0.0, fastforwarding);
    // Second longer horizontal stroke below
    draw::curve(Point{x: pos.x+((10_f64*scale) as u16), y: pos.y+((40_f64*scale) as u16)}, (80_f64*scale) as u16, thickness, 0.0, color, delay, 0.2, fastforwarding);
}

pub fn sa(pos: Point, scale: f64, thickness: u16, color: u16, delay: u32, fastforwarding: draw::Fastforwarding) {
    // First short horizontal stroke
    draw::curve(Point{x: pos.x+((10_f64*scale) as u16), y: pos.y+((5_f64*scale) as u16)}, (40_f64*scale) as u16, thickness, 0.0, color, delay, 0.0, fastforwarding);
    // Second curved stroke
    draw::curve(Point{x: pos.x+((30_f64*scale) as u16), y: pos.y+((10_f64*scale) as u16)}, (70_f64*scale) as u16, thickness, -30.0, color, delay, 0.2, fastforwarding);
    // Final bottom stroke
    draw::curve(Point{x: pos.x+((30_f64*scale) as u16), y: pos.y+((40_f64*scale) as u16)}, (40_f64*scale) as u16, thickness, 90.0, color, delay, 0.4, fastforwarding);
}

pub fn shi(pos: Point, scale: f64, thickness: u16, color: u16, delay: u32, fastforwarding: draw::Fastforwarding) {
    // Single sweeping curved stroke
    draw::curve(Point{x: pos.x+((10_f64*scale) as u16), y: pos.y+((10_f64*scale) as u16)}, (120_f64*scale) as u16, thickness, -45.0, color, delay, 0.0, fastforwarding);
}

pub fn su(pos: Point, scale: f64, thickness: u16, color: u16, delay: u32, fastforwarding: draw::Fastforwarding) {
    // First short stroke
    draw::curve(Point{x: pos.x+((10_f64*scale) as u16), y: pos.y+((5_f64*scale) as u16)}, (30_f64*scale) as u16, thickness, 0.0, color, delay, 0.0, fastforwarding);
    // Sweeping curved stroke
    draw::curve(Point{x: pos.x+((30_f64*scale) as u16), y: pos.y+((10_f64*scale) as u16)}, (120_f64*scale) as u16, thickness, -45.0, color, delay, 0.2, fastforwarding);
}

pub fn se(pos: Point, scale: f64, thickness: u16, color: u16, delay: u32, fastforwarding: draw::Fastforwarding) {
    // First horizontal stroke
    draw::curve(Point{x: pos.x+((10_f64*scale) as u16), y: pos.y+((5_f64*scale) as u16)}, (60_f64*scale) as u16, thickness, 0.0, color, delay, 0.0, fastforwarding);
    // Vertical stroke
    draw::curve(Point{x: pos.x+((30_f64*scale) as u16), y: pos.y+((5_f64*scale) as u16)}, (80_f64*scale) as u16, thickness, 90.0, color, delay, 0.2, fastforwarding);
    // Curved stroke below
    draw::curve(Point{x: pos.x+((10_f64*scale) as u16), y: pos.y+((35_f64*scale) as u16)}, (100_f64*scale) as u16, thickness, -30.0, color, delay, 0.4, fastforwarding);
}

pub fn so(pos: Point, scale: f64, thickness: u16, color: u16, delay: u32, fastforwarding: draw::Fastforwarding) {
    // Single long curved stroke
    draw::curve(Point{x: pos.x+((10_f64*scale) as u16), y: pos.y+((5_f64*scale) as u16)}, (120_f64*scale) as u16, thickness, -30.0, color, delay, 0.0, fastforwarding);
}
