use crate::logic;
use crate::draw::{line, rounded_rect, dot, Text, Fastforwarding};
use crate::logic::display::{wait_for_vblank, push_rect_uniform, draw_string};
use crate::theme::Theme;

pub fn rect_widget(rect: logic::Rect, theme: Theme, thickness: u16) {
    wait_for_vblank();
    push_rect_uniform(logic::Rect{x : rect.x - 1, y: rect.y, width: rect.width, height: rect.height + 1}, logic::Color{rgb565: theme.bg});
    rounded_rect(logic::Point{x: rect.x, y: rect.y + 2}, rect.width, rect.height, thickness-2, theme.overlay);
    rounded_rect(logic::Point{x: rect.x, y: rect.y - 4}, rect.width, rect.height, thickness, theme.accent);
}

pub fn character_widget(pos: logic::Point, theme: Theme, text: Text) {
    line(logic::Point{x: pos.x, y: pos.y+5}, 70, 15, theme.overlay, false);
    line(logic::Point{x: pos.x, y: pos.y}, 70, 16, text.bg_color, false);
    unsafe {
        draw_string(text.text.as_ptr(), logic::Point{x: pos.x + text.xoffset, y: pos.y - 10}, text.big, logic::Color{rgb565 : text.text_color}, logic::Color{rgb565 : text.bg_color});
    }
}

pub fn info_widget(pos: logic::Point, theme: Theme, text: Text) {
    line(logic::Point{x: pos.x, y: pos.y+5}, 30, 10, theme.overlay, false);
    line(logic::Point{x: pos.x, y: pos.y}, 30, 12, theme.accent, false);
    line(logic::Point{x: pos.x, y: pos.y}, 30, 8, text.bg_color, false);
    unsafe {
        draw_string(text.text.as_ptr(), logic::Point{x: pos.x - 10 + text.xoffset, y: pos.y - 6}, false, logic::Color{rgb565 : text.text_color}, logic::Color{rgb565 : text.bg_color});
    }
}

pub fn theme_widget(theme: Theme) {
    wait_for_vblank();
    line(logic::Point{x: 30, y: 30}, 210, 19, theme.subtext, false);
    line(logic::Point{x: 30, y: 25}, 210, 20, theme.overlay, false);
    wait_for_vblank();
    line(logic::Point{x: 90, y: 25}, 150, 14, theme.bg, false);
    unsafe { 
        draw_string(theme.name.as_ptr(), logic::Point{x: 92, y: 17}, true, logic::Color{rgb565 : theme.text}, logic::Color{rgb565 : theme.bg});
    }
    dot(logic::Point{x: 30, y: 25}, 14, theme.tertiary_accent, 0, Fastforwarding{enabled: false, scale: 1.0});
    dot(logic::Point{x: 45, y: 25}, 14, theme.secondary_accent, 0, Fastforwarding{enabled: false, scale: 1.0});
    dot(logic::Point{x: 60, y: 25}, 14, theme.accent, 0, Fastforwarding{enabled: false, scale: 1.0});
}

pub fn difficulty_widget(pos: logic::Point, theme: Theme, level: u8) {
    wait_for_vblank();
    line(logic::Point{x: pos.x, y: pos.y+5}, 70, 16, theme.overlay, false);
    line(logic::Point{x: pos.x, y: pos.y}, 70, 16, theme.accent, false);
    line(logic::Point{x: pos.x, y: pos.y}, 70, 6, theme.bg, false);
    if level == 1 {
        line(logic::Point{x: pos.x, y: pos.y}, 5, 10, theme.tertiary_accent, false);
    } else if level == 2 {
        line(logic::Point{x: pos.x, y: pos.y}, 15, 10, theme.tertiary_accent, false);
    } else if level == 3 {
        line(logic::Point{x: pos.x, y: pos.y}, 30, 10, theme.tertiary_accent, false);
    } else if level == 4 {
        line(logic::Point{x: pos.x, y: pos.y}, 50, 10, theme.tertiary_accent, false);
    } else if level == 5 {
        line(logic::Point{x: pos.x, y: pos.y}, 70, 10, theme.tertiary_accent, false);
    }
}