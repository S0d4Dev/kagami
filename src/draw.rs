// use crate::theme::Theme;
use crate::logic;

pub struct Text<'a> {
    pub text: &'a str,
    pub big: bool,
    pub text_color: u16,
    pub bg_color: u16,
    pub delay: u32,
}

#[derive(Copy, Clone)]
pub struct Fastforwarding {
    pub enabled: bool,
    pub scale: f32, 
}

pub fn clearscreen(default_color:u16) {
    logic::display::push_rect_uniform(logic::Rect{ x: 0, y: 0, width: 320, height: 240}, logic::Color{rgb565: default_color});
}

pub fn string(text: &str, pos: logic::Point, big: bool, text_color: u16, bg_color: u16, delay: u32) {
    // for (i, c) in text.chars().enumerate() {
    //     logic::display::draw_string(
    //         &(c as u8),
    //         logic::Point { x: pos.x + i as u16 * 10, y: pos.y },
    //         big,
    //         logic::Color { rgb565: text_color },
    //         logic::Color { rgb565: bg_color },
    //     );
    //     logic::timing::usleep(delay);
    // }
    unsafe {logic::display::draw_string(text.as_ptr(),logic::Point { x: pos.x, y: pos.y },big,logic::Color { rgb565: text_color },logic::Color { rgb565: bg_color },); }
    logic::timing::usleep(delay);
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
            logic::Color { rgb565: color }
        );
    }
    if fastforwarding.enabled {
        if logic::keyboard::KeyboardState::scan().key_down(logic::keyboard::Key::Plus) {
            logic::timing::usleep(delay-((delay as f32 / fastforwarding.scale) as u32));
        } else if logic::keyboard::KeyboardState::scan().key_down(logic::keyboard::Key::Minus) {
            logic::timing::usleep(delay+((delay as f32 / fastforwarding.scale) as u32));
        } else if logic::keyboard::KeyboardState::scan().key_down(logic::keyboard::Key::Exe) {
            logic::timing::usleep(0);
        } else {
            logic::timing::usleep(delay);
        }
    } else {
        logic::timing::usleep(delay);
    }
}

pub fn line(pos: logic::Point, length: u16, thickness: u16, color: u16, column: bool) {
    dot(logic::Point{x: pos.x, y: pos.y}, thickness, color, 0, Fastforwarding{enabled: false, scale: 1.0});
    if column {
        logic::display::push_rect_uniform(logic::Rect{x: pos.x-thickness+1, y: pos.y, width: (thickness*2)-1, height: length}, logic::Color {rgb565: color});
        dot(logic::Point{x: pos.x, y: pos.y+length}, thickness, color, 0, Fastforwarding{enabled: false, scale: 1.0});
    } else {
        logic::display::push_rect_uniform(logic::Rect{x: pos.x, y: pos.y-thickness+1, width: length-1, height: (thickness*2)-1}, logic::Color {rgb565: color});
        dot(logic::Point{x: pos.x+length, y: pos.y}, thickness, color, 0, Fastforwarding{enabled: false, scale: 1.0});
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
            if logic::keyboard::KeyboardState::scan().key_down(logic::keyboard::Key::Home) {
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
    logic::display::push_rect_uniform(logic::Rect{x: pos.x + 8, y: pos.y - thickness, width: length-16, height: thickness * 2}, logic::Color{rgb565: color});
    curve(
        logic::Point{ x: pos.x + length - 7, y: pos.y},
        11,
        thickness,
        0.0,
        color,
        0,
        0.8,
        Fastforwarding{enabled : false, scale: 1.0},
    );
    logic::display::push_rect_uniform(logic::Rect{x: pos.x + length - thickness, y: pos.y + 8, width: thickness * 2, height: height - 16}, logic::Color{rgb565: color});
    curve(
        logic::Point{ x: pos.x + length, y: pos.y + height - 7},
        11,
        thickness,
        90.0,
        color,
        0,
        0.8,
        Fastforwarding{enabled : false, scale: 1.0},
    );
    logic::display::push_rect_uniform(logic::Rect{x: pos.x + 8, y: pos.y + height - thickness, width: length - 16, height: thickness * 2}, logic::Color{rgb565: color});
    curve(
        logic::Point{ x: pos.x + 7, y: pos.y + height},
        11,
        thickness,
        180.0,
        color,
        0,
        0.8,
        Fastforwarding{enabled : false, scale: 1.0},
    );
    logic::display::push_rect_uniform(logic::Rect{x: pos.x - thickness, y: pos.y + 8, width: thickness * 2, height: height - 16}, logic::Color{rgb565: color});
    curve(
        logic::Point{ x: pos.x, y: pos.y + 7},
        11,
        thickness,
        270.0,
        color,
        0,
        0.8,
        Fastforwarding{enabled : false, scale: 1.0},
    );
}