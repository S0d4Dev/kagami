use crate::core::{draw, logic::Point};

pub fn a(pos: Point, scale: f64, thickness: u16, color: u16, delay: u32, fastforwarding: draw::Fastforwarding) {
    draw::curve(Point{x: pos.x, y: pos.y+((20_f64*scale) as u16)}, (100_f64*scale) as u16, thickness, 0.0, color, delay, 0.0, fastforwarding);
    draw::curve(Point{x: pos.x+((50_f64*scale) as u16), y: pos.y}, (120_f64*scale) as u16, thickness, 110.0, color, delay, -0.35, fastforwarding);
    draw::curve(Point{x: pos.x+((80_f64*scale) as u16), y: pos.y+((40_f64*scale) as u16)}, (80_f64*scale) as u16, thickness, 90.0, color, delay, 0.4, fastforwarding);
    draw::curve(Point{x: pos.x+((50_f64*scale) as u16), y: pos.y+((110_f64*scale) as u16)}, (45_f64*scale) as u16, thickness, 140.0, color, delay, 0.79, fastforwarding);
    draw::curve(Point{x: pos.x+((10_f64*scale) as u16), y: pos.y+((110_f64*scale) as u16)}, (81_f64*scale) as u16, thickness, -140.0, color, delay, 1.49, fastforwarding);
    draw::curve(Point{x: pos.x+((50_f64*scale) as u16), y: pos.y+((45_f64*scale) as u16)}, (40_f64*scale) as u16, thickness, 0.0, color, delay, 0.3, fastforwarding);
    draw::curve(Point{x: pos.x+((85_f64*scale) as u16), y: pos.y+((54_f64*scale) as u16)}, (45_f64*scale) as u16, thickness, 30.0, color, delay, 1.05, fastforwarding);
}

pub fn i(pos: Point, scale: f64, thickness: u16, color: u16, delay: u32, fastforwarding: draw::Fastforwarding) {
    draw::curve(Point{x: pos.x+((20_f64*scale) as u16), y: pos.y+((15_f64*scale) as u16)}, (80_f64*scale) as u16, thickness, 110.0, color, delay, -0.5, fastforwarding);
    draw::curve(Point{x: pos.x+((30_f64*scale) as u16), y: pos.y+((90_f64*scale) as u16)}, (25_f64*scale) as u16, thickness, 75.0, color, delay, -1.7, fastforwarding);
    draw::curve(Point{x: pos.x+((85_f64*scale) as u16), y: pos.y+((25_f64*scale) as u16)}, (60_f64*scale) as u16, thickness, 60.0, color, delay, 0.4, fastforwarding);
}

pub fn u(pos: Point, scale: f64, thickness: u16, color: u16, delay: u32, fastforwarding: draw::Fastforwarding) {
    draw::curve(Point{x: pos.x+((35_f64*scale) as u16), y: pos.y+((5_f64*scale) as u16)}, (50_f64*scale) as u16, thickness, -5.0, color, delay, 0.3, fastforwarding);
    draw::curve(Point{x: pos.x+((20_f64*scale) as u16), y: pos.y+((40_f64*scale) as u16)}, (80_f64*scale) as u16, thickness, -15.0, color, delay, 0.3, fastforwarding);
    draw::curve(Point{x: pos.x+((100_f64*scale) as u16), y: pos.y+((45_f64*scale) as u16)}, (20_f64*scale) as u16, thickness, 5.0, color, delay, 1.2, fastforwarding);
    draw::curve(Point{x: pos.x+((106_f64*scale) as u16), y: pos.y+((65_f64*scale) as u16)}, (80_f64*scale) as u16, thickness, 110.0, color, delay, 0.5, fastforwarding);
}

pub fn e(pos: Point, scale: f64, thickness: u16, color: u16, delay: u32, fastforwarding: draw::Fastforwarding) {
    draw::curve(Point{x: pos.x+((30_f64*scale) as u16), y: pos.y+((5_f64*scale) as u16)}, (50_f64*scale) as u16, thickness, 5.0, color, delay, 0.0, fastforwarding);
    draw::curve(Point{x: pos.x+((15_f64*scale) as u16), y: pos.y+((25_f64*scale) as u16)}, (90_f64*scale) as u16, thickness, 0.0, color, delay, 0.0, fastforwarding);
    draw::curve(Point{x: pos.x+((105_f64*scale) as u16), y: pos.y+((25_f64*scale) as u16)}, (120_f64*scale) as u16, thickness, 137.0, color, delay, 0.0, fastforwarding);
    draw::curve(Point{x: pos.x+((50_f64*scale) as u16), y: pos.y+((78_f64*scale) as u16)}, (30_f64*scale) as u16, thickness, -50.0, color, delay, 1.4, fastforwarding);
    draw::curve(Point{x: pos.x+((77_f64*scale) as u16), y: pos.y+((92_f64*scale) as u16)}, (25_f64*scale) as u16, thickness, 90.0, color, delay, -1.1, fastforwarding);
}

pub fn o(pos: Point, scale: f64, thickness: u16, color: u16, delay: u32, fastforwarding: draw::Fastforwarding) {
    draw::curve(Point{x: pos.x, y: pos.y+((22_f64*scale) as u16)}, (85_f64*scale) as u16, thickness, 0.0, color, delay, 0.0, fastforwarding);
    draw::curve(Point{x: pos.x+((50_f64*scale) as u16), y: pos.y}, (108_f64*scale) as u16, thickness, 80.0, color, delay, 0.2, fastforwarding);
    draw::curve(Point{x: pos.x+((50_f64*scale) as u16), y: pos.y+((110_f64*scale) as u16)}, (45_f64*scale) as u16, thickness, 140.0, color, delay, 0.79, fastforwarding);
    draw::curve(Point{x: pos.x+((10_f64*scale) as u16), y: pos.y+((110_f64*scale) as u16)}, (81_f64*scale) as u16, thickness, -140.0, color, delay, 1.49, fastforwarding);
    draw::curve(Point{x: pos.x+((50_f64*scale) as u16), y: pos.y+((45_f64*scale) as u16)}, (45_f64*scale) as u16, thickness, 0.0, color, delay, 0.3, fastforwarding);
    draw::curve(Point{x: pos.x+((91_f64*scale) as u16), y: pos.y+((57_f64*scale) as u16)}, (45_f64*scale) as u16, thickness, 40.0, color, delay, 1.0, fastforwarding);
    draw::curve(Point{x: pos.x+((90_f64*scale) as u16), y: pos.y+((5_f64*scale) as u16)}, (20_f64*scale) as u16, thickness, 55.0, color, delay, 0.0, fastforwarding);
}