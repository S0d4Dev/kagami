use crate::core::{draw, logic::Point};

pub fn a(pos: Point, scale: f64, thickness: u16, color: u16, delay: u32, fastforwarding: draw::Fastforwarding) {
    draw::curve(Point{x: pos.x+((15_f64*scale) as u16), y: pos.y+((30_f64*scale) as u16)}, (85_f64*scale) as u16, thickness, 0.0, color, delay, 0.0, fastforwarding);
    draw::curve(Point{x: pos.x+((100_f64*scale) as u16), y: pos.y+((30_f64*scale) as u16)}, (35_f64*scale) as u16, thickness, 110.0, color, delay, 0.3, fastforwarding);
    draw::curve(Point{x: pos.x+((55_f64*scale) as u16), y: pos.y+((50_f64*scale) as u16)}, (60_f64*scale) as u16, thickness, 90.0, color, delay, 0.4, fastforwarding);
}

pub fn i(pos: Point, scale: f64, thickness: u16, color: u16, delay: u32, fastforwarding: draw::Fastforwarding) {
    draw::curve(Point{x: pos.x+((10_f64*scale) as u16), y: pos.y+((34_f64*scale) as u16)}, (90_f64*scale) as u16, thickness, 0.0, color, delay, -0.3, fastforwarding);
    draw::curve(Point{x: pos.x+((50_f64*scale) as u16), y: pos.y+((30_f64*scale) as u16)}, (70_f64*scale) as u16, thickness, 90.0, color, delay, 0.0, fastforwarding);
}

pub fn u(pos: Point, scale: f64, thickness: u16, color: u16, delay: u32, fastforwarding: draw::Fastforwarding) {
    draw::curve(Point{x: pos.x+((50_f64*scale) as u16), y: pos.y+((10_f64*scale) as u16)}, (10_f64*scale) as u16, thickness, 90.0, color, delay, 0.0, fastforwarding);
    draw::curve(Point{x: pos.x+((10_f64*scale) as u16), y: pos.y+((25_f64*scale) as u16)}, (85_f64*scale) as u16, thickness, 0.0, color, delay, 0.0, fastforwarding);
    draw::curve(Point{x: pos.x+((95_f64*scale) as u16), y: pos.y+((25_f64*scale) as u16)}, (100_f64*scale) as u16, thickness, 90.0, color, delay, 0.7, fastforwarding);
}

pub fn e(pos: Point, scale: f64, thickness: u16, color: u16, delay: u32, fastforwarding: draw::Fastforwarding) {
    draw::curve(Point{x: pos.x+((20_f64*scale) as u16), y: pos.y+((10_f64*scale) as u16)}, (60_f64*scale) as u16, thickness, 0.0, color, delay, 0.0, fastforwarding);
    draw::curve(Point{x: pos.x+((50_f64*scale) as u16), y: pos.y+((10_f64*scale) as u16)}, (80_f64*scale) as u16, thickness, 90.0, color, delay, 0.0, fastforwarding);
    draw::curve(Point{x: pos.x+((5_f64*scale) as u16), y: pos.y+((90_f64*scale) as u16)}, (90_f64*scale) as u16, thickness, 0.0, color, delay, 0.0, fastforwarding);
}

pub fn o(pos: Point, scale: f64, thickness: u16, color: u16, delay: u32, fastforwarding: draw::Fastforwarding) {
    draw::curve(Point{x: pos.x+((10_f64*scale) as u16), y: pos.y+((20_f64*scale) as u16)}, (85_f64*scale) as u16, thickness, 0.0, color, delay, 0.0, fastforwarding);
    draw::curve(Point{x: pos.x+((50_f64*scale) as u16), y: pos.y+((20_f64*scale) as u16)}, (80_f64*scale) as u16, thickness, 90.0, color, delay, 0.0, fastforwarding);
    draw::curve(Point{x: pos.x+((15_f64*scale) as u16), y: pos.y+((80_f64*scale) as u16)}, (70_f64*scale) as u16, thickness, 0.0, color, delay, 0.0, fastforwarding);
}

pub fn ka(pos: Point, scale: f64, thickness: u16, color: u16, delay: u32, fastforwarding: draw::Fastforwarding) {
    // First vertical stroke
    draw::curve(Point{x: pos.x+((10_f64*scale) as u16), y: pos.y+((5_f64*scale) as u16)}, (70_f64*scale) as u16, thickness, 1.57, color, delay, 0.0, fastforwarding); // 1.57 rad ≈ 90°
    // Second diagonal stroke crossing the vertical line
    draw::curve(Point{x: pos.x+((20_f64*scale) as u16), y: pos.y+((30_f64*scale) as u16)}, (80_f64*scale) as u16, thickness, -0.75, color, delay, 0.2, fastforwarding);
    // Third short horizontal stroke on the top-right
    draw::curve(Point{x: pos.x+((50_f64*scale) as u16), y: pos.y+((5_f64*scale) as u16)}, (40_f64*scale) as u16, thickness, 0.0, color, delay, 0.4, fastforwarding);
}

pub fn ki(pos: Point, scale: f64, thickness: u16, color: u16, delay: u32, fastforwarding: draw::Fastforwarding) {
    // First horizontal stroke at the top
    draw::curve(Point{x: pos.x+((10_f64*scale) as u16), y: pos.y+((5_f64*scale) as u16)}, (60_f64*scale) as u16, thickness, 0.0, color, delay, 0.0, fastforwarding);
    // Second vertical stroke
    draw::curve(Point{x: pos.x+((30_f64*scale) as u16), y: pos.y+((5_f64*scale) as u16)}, (80_f64*scale) as u16, thickness, 1.57, color, delay, 0.2, fastforwarding); // 1.57 rad ≈ 90°
    // Third diagonal stroke crossing the vertical
    draw::curve(Point{x: pos.x+((10_f64*scale) as u16), y: pos.y+((30_f64*scale) as u16)}, (60_f64*scale) as u16, thickness, -0.75, color, delay, 0.4, fastforwarding);
}

pub fn ku(pos: Point, scale: f64, thickness: u16, color: u16, delay: u32, fastforwarding: draw::Fastforwarding) {
    // First short curved stroke
    draw::curve(Point{x: pos.x+((5_f64*scale) as u16), y: pos.y+((10_f64*scale) as u16)}, (60_f64*scale) as u16, thickness, -0.25, color, delay, 0.0, fastforwarding); // Slight curve to the bottom
    // Second longer curved stroke below
    draw::curve(Point{x: pos.x+((10_f64*scale) as u16), y: pos.y+((40_f64*scale) as u16)}, (100_f64*scale) as u16, thickness, -0.4, color, delay, 0.2, fastforwarding);
}

pub fn ke(pos: Point, scale: f64, thickness: u16, color: u16, delay: u32, fastforwarding: draw::Fastforwarding) {
    // First vertical stroke
    draw::curve(Point{x: pos.x+((10_f64*scale) as u16), y: pos.y+((5_f64*scale) as u16)}, (80_f64*scale) as u16, thickness, 1.57, color, delay, 0.0, fastforwarding); // 1.57 rad ≈ 90°
    // Second diagonal stroke crossing the vertical
    draw::curve(Point{x: pos.x+((10_f64*scale) as u16), y: pos.y+((30_f64*scale) as u16)}, (70_f64*scale) as u16, thickness, -0.75, color, delay, 0.2, fastforwarding);
    // Third short horizontal stroke at the top
    draw::curve(Point{x: pos.x+((50_f64*scale) as u16), y: pos.y+((5_f64*scale) as u16)}, (40_f64*scale) as u16, thickness, 0.0, color, delay, 0.4, fastforwarding);
}

pub fn ko(pos: Point, scale: f64, thickness: u16, color: u16, delay: u32, fastforwarding: draw::Fastforwarding) {
    // First horizontal stroke at the top
    draw::curve(Point{x: pos.x+((10_f64*scale) as u16), y: pos.y+((10_f64*scale) as u16)}, (50_f64*scale) as u16, thickness, 0.0, color, delay, 0.0, fastforwarding);
    // Second horizontal stroke below
    draw::curve(Point{x: pos.x+((10_f64*scale) as u16), y: pos.y+((40_f64*scale) as u16)}, (100_f64*scale) as u16, thickness, 0.0, color, delay, 0.2, fastforwarding);
}

pub fn sa(pos: Point, scale: f64, thickness: u16, color: u16, delay: u32, fastforwarding: draw::Fastforwarding) {
    // First short horizontal stroke
    draw::curve(Point{x: pos.x+((5_f64*scale) as u16), y: pos.y+((5_f64*scale) as u16)}, (40_f64*scale) as u16, thickness, 0.0, color, delay, 0.0, fastforwarding);
    // Second diagonal stroke
    draw::curve(Point{x: pos.x+((20_f64*scale) as u16), y: pos.y+((10_f64*scale) as u16)}, (70_f64*scale) as u16, thickness, -0.75, color, delay, 0.2, fastforwarding);
    // Final stroke, small curve at the bottom
    draw::curve(Point{x: pos.x+((30_f64*scale) as u16), y: pos.y+((40_f64*scale) as u16)}, (40_f64*scale) as u16, thickness, 1.57, color, delay, 0.4, fastforwarding);
}

pub fn shi(pos: Point, scale: f64, thickness: u16, color: u16, delay: u32, fastforwarding: draw::Fastforwarding) {
    // Single curved stroke
    draw::curve(Point{x: pos.x+((5_f64*scale) as u16), y: pos.y+((5_f64*scale) as u16)}, (120_f64*scale) as u16, thickness, -0.35, color, delay, 0.0, fastforwarding); // Gentle curve down
}

pub fn su(pos: Point, scale: f64, thickness: u16, color: u16, delay: u32, fastforwarding: draw::Fastforwarding) {
    // First short horizontal stroke
    draw::curve(Point{x: pos.x+((10_f64*scale) as u16), y: pos.y+((5_f64*scale) as u16)}, (30_f64*scale) as u16, thickness, 0.0, color, delay, 0.0, fastforwarding);
    // Second longer curved stroke below
    draw::curve(Point{x: pos.x+((20_f64*scale) as u16), y: pos.y+((40_f64*scale) as u16)}, (100_f64*scale) as u16, thickness, -0.5, color, delay, 0.2, fastforwarding);
}

pub fn se(pos: Point, scale: f64, thickness: u16, color: u16, delay: u32, fastforwarding: draw::Fastforwarding) {
    // First horizontal stroke
    draw::curve(Point{x: pos.x+((10_f64*scale) as u16), y: pos.y+((5_f64*scale) as u16)}, (60_f64*scale) as u16, thickness, 0.0, color, delay, 0.0, fastforwarding);
    // Vertical stroke
    draw::curve(Point{x: pos.x+((30_f64*scale) as u16), y: pos.y+((5_f64*scale) as u16)}, (80_f64*scale) as u16, thickness, 90.0, color, delay, 0.2, fastforwarding);
    // Second short horizontal stroke at the bottom
    draw::curve(Point{x: pos.x+((10_f64*scale) as u16), y: pos.y+((60_f64*scale) as u16)}, (60_f64*scale) as u16, thickness, 0.0, color, delay, 0.4, fastforwarding);
}

pub fn so(pos: Point, scale: f64, thickness: u16, color: u16, delay: u32, fastforwarding: draw::Fastforwarding) {
    // Long diagonal sweeping stroke
    draw::curve(Point{x: pos.x+((10_f64*scale) as u16), y: pos.y+((5_f64*scale) as u16)}, (120_f64*scale) as u16, thickness, -45.0, color, delay, 0.0, fastforwarding);
}
