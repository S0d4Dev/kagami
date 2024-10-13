use crate::core::{draw, logic::Point};

pub fn exclamation(pos: Point, scale: f64, thickness: u16, color: u16, delay: u32, fastforwarding: draw::Fastforwarding) {
    draw::curve(Point{x: pos.x+((0_f64*scale) as u16), y: pos.y+((0_f64*scale) as u16)}, (60_f64*scale) as u16, thickness, 90.0, color, delay, 0.0, fastforwarding);
    draw::dot(Point{x: pos.x+((0_f64*scale) as u16), y: pos.y+((85_f64*scale) as u16)}, thickness+2, color, delay, fastforwarding)
}

pub fn question(pos: Point, scale: f64, thickness: u16, color: u16, delay: u32, fastforwarding: draw::Fastforwarding){
    draw::curve(Point{x: pos.x+((0_f64*scale) as u16), y: pos.y+((0_f64*scale) as u16)}, (75_f64*scale) as u16, thickness, 90.0, color, delay, 0.0, fastforwarding);
}

pub fn dot(pos: Point, scale: f64, thickness: u16, color: u16, delay: u32, fastforwarding: draw::Fastforwarding){
    draw::curve(Point{x: pos.x+((0_f64*scale) as u16), y: pos.y+((0_f64*scale) as u16)}, (20_f64*scale) as u16, thickness, 90.0, color, delay, 2.0, fastforwarding);
}