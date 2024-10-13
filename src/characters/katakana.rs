use crate::core::{draw, logic::Point};

pub fn a(pos: Point, scale: f64, thickness: u16, color: u16, delay: u32, fastforwarding: draw::Fastforwarding) {
    draw::curve(Point{x: pos.x+((10_f64*scale) as u16), y: pos.y+((20_f64*scale) as u16)}, (85_f64*scale) as u16, thickness, 0.0, color, delay, 0.0, fastforwarding);
    draw::curve(Point{x: pos.x+((95_f64*scale) as u16), y: pos.y+((20_f64*scale) as u16)}, (35_f64*scale) as u16, thickness, 110.0, color, delay, 0.3, fastforwarding);
    draw::curve(Point{x: pos.x+((50_f64*scale) as u16), y: pos.y+((40_f64*scale) as u16)}, (60_f64*scale) as u16, thickness, 90.0, color, delay, 0.4, fastforwarding);
}

pub fn i(pos: Point, scale: f64, thickness: u16, color: u16, delay: u32, fastforwarding: draw::Fastforwarding) {
    draw::curve(Point{x: pos.x+((10_f64*scale) as u16), y: pos.y+((34_f64*scale) as u16)}, (90_f64*scale) as u16, thickness, 0.0, color, delay, -0.4, fastforwarding);
    draw::curve(Point{x: pos.x+((50_f64*scale) as u16), y: pos.y+((30_f64*scale) as u16)}, (70_f64*scale) as u16, thickness, 90.0, color, delay, 0.0, fastforwarding);
}

pub fn u(pos: Point, scale: f64, thickness: u16, color: u16, delay: u32, fastforwarding: draw::Fastforwarding) {
    draw::curve(Point{x: pos.x+((50_f64*scale) as u16), y: pos.y+((5_f64*scale) as u16)}, (10_f64*scale) as u16, thickness, 90.0, color, delay, 0.0, fastforwarding);
    draw::curve(Point{x: pos.x+((10_f64*scale) as u16), y: pos.y+((20_f64*scale) as u16)}, (85_f64*scale) as u16, thickness, 0.0, color, delay, 0.0, fastforwarding);
    draw::curve(Point{x: pos.x+((95_f64*scale) as u16), y: pos.y+((20_f64*scale) as u16)}, (100_f64*scale) as u16, thickness, 90.0, color, delay, 0.5, fastforwarding);
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