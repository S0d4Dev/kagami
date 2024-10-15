use crate::core::logic;

pub struct Text<'a> {
    pub text: &'a str,
    pub xoffset: u16,
    pub big: bool,
    pub text_color: u16,
    pub bg_color: u16,
}

#[derive(Copy, Clone)]
pub struct Fastforwarding {
    pub enabled: bool,
    pub scale: f32,
}

pub fn dot(pos: logic::Point, radius: u16, color: u16, delay: u32, fastforwarding: Fastforwarding) {
    for y in 0..(radius * 2) {
        let dy = y as i32 - radius as i32;
        let mut dx = 0;
        while dx * dx + dy * dy <= (radius as i32 * radius as i32) {
            dx += 1;
        }
        dx -= 1;
        logic::display::push_rect_uniform(
            logic::Rect {
                x: pos.x - dx as u16,
                y: pos.y - radius + y,
                width: (dx * 2) as u16,
                height: 1
            },
            color
        );
    }
    if fastforwarding.enabled {
        if logic::keypress(logic::keyboard::Key::Plus) {
            logic::timing::usleep(delay-((delay as f32 / fastforwarding.scale) as u32));
        } else if logic::keypress(logic::keyboard::Key::Minus) {
            logic::timing::usleep(delay+((delay as f32 / fastforwarding.scale) as u32));
        } else if logic::keypress(logic::keyboard::Key::Left) || logic::keypress(logic::keyboard::Key::Right) {
            logic::timing::usleep(300);
        } else if logic::keypress(logic::keyboard::Key::Exe) {
            logic::timing::usleep(0);
        } else {
            logic::timing::usleep(delay);
        }
    } else {
        logic::timing::usleep(delay);
    }
}

pub fn line(pos: logic::Point, length: u16, thickness: u16, color: u16, column: bool) {
    dot(logic::Point{x: pos.x+1, y: pos.y}, thickness, color, 0, Fastforwarding{enabled: false, scale: 1.0});
    if column {
        logic::display::push_rect_uniform(
            logic::Rect{x: pos.x-thickness+1, y: pos.y, width: (thickness*2)-1, height: length}, 
            color
        );
        dot(
            logic::Point{x: pos.x+1, y: pos.y+length}, 
            thickness, 
            color, 
            0, 
            Fastforwarding{enabled: false, scale: 1.0}
        );
    } else {
        logic::display::push_rect_uniform(
            logic::Rect{x: pos.x, y: pos.y-thickness+1, width: length-1, height: (thickness*2)-1}, 
            color
        );
        dot(
            logic::Point{x: pos.x+length, y: pos.y}, 
            thickness, 
            color, 
            0, 
            Fastforwarding{enabled: false, scale: 1.0}
        );
    }
}

#[allow(clippy::too_many_arguments)]
pub fn curve(pos: logic::Point, length: u16, thickness: u16, orientation: f64, color: u16, delay: u32, curve: f64, fastforwarding: Fastforwarding) {
    use core::f64::consts;
    let pi: f64 = consts::PI;
    let radians = orientation * (pi / 180.0);
    let mut dx;
    let mut dy;
    for i in 0..length {
        if curve != 0.0 {
            dx = libm::round(i as f64 * libm::cos(radians + (curve / length as f64) * i as f64)) as i16;
            dy = libm::round(i as f64 * libm::sin(radians + (curve / length as f64) * i as f64)) as i16;
        } else {
            dx = libm::round(i as f64 * libm::cos(radians)) as i16;
            dy = libm::round(i as f64 * libm::sin(radians)) as i16;
        }
        dot(
            logic::Point{
                x: (pos.x as i16 + dx) as u16,
                y: (pos.y as i16 + dy) as u16
            },
            thickness,
            color,
            if i >= length - 1{
                0
            } else {
                delay
            },
            fastforwarding,
        );
    }
}

pub fn rounded_rect(pos: logic::Point, length: u16, height: u16, thickness: u16, color: u16) {
    logic::display::wait_for_vblank();
    logic::display::push_rect_uniform(
        logic::Rect{x: pos.x + 8, y: pos.y - thickness, width: length-16, height: thickness * 2}, 
        color
    );
    curve(
        logic::Point{ x: pos.x + length - 7, y: pos.y},
        11,
        thickness,
        0.0,
        color,
        0,
        0.8,
        Fastforwarding{enabled : false, scale: 1.0}
    );
    logic::display::push_rect_uniform(
        logic::Rect{x: pos.x + length - thickness, y: pos.y + 8, width: thickness * 2, height: height - 16},
        color
    );
    curve(
        logic::Point{ x: pos.x + length, y: pos.y + height - 7},
        11,
        thickness,
        90.0,
        color,
        0,
        0.8,
        Fastforwarding{enabled : false, scale: 1.0}
    );
    logic::display::push_rect_uniform(
        logic::Rect{x: pos.x + 8, y: pos.y + height - thickness, width: length - 16, height: thickness * 2},
        color
    );
    curve(
        logic::Point{ x: pos.x + 7, y: pos.y + height},
        11,
        thickness,
        180.0,
        color,
        0,
        0.8,
        Fastforwarding{enabled : false, scale: 1.0}
    );
    logic::display::push_rect_uniform(
        logic::Rect{x: pos.x - thickness, y: pos.y + 8, width: thickness * 2, height: height - 16},
        color
    );
    curve(
        logic::Point{ x: pos.x, y: pos.y + 7},
        11,
        thickness,
        270.0,
        color,
        0,
        0.8,
        Fastforwarding{enabled : false, scale: 1.0}
    );
}