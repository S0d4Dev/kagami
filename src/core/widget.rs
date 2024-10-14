use crate::core::{logic, draw, theme};

pub fn rect(rect: logic::Rect, theme: theme::Theme, thickness: u16) {
    logic::display::wait_for_vblank();
    logic::display::push_rect_uniform(
        logic::Rect{x : rect.x - 1, y: rect.y, width: rect.width, height: rect.height + 1}, 
        logic::Color{rgb565: theme.bg}
    );
    draw::rounded_rect(logic::Point{x: rect.x, y: rect.y + 2}, rect.width, rect.height, thickness-2, theme.overlay);
    draw::rounded_rect(logic::Point{x: rect.x, y: rect.y - 4}, rect.width, rect.height, thickness, theme.accent);
}

pub fn guide_lines(theme: theme::Theme) {
    draw::line(logic::Point{x: 30, y: 120}, 150, 8, theme.overlay, false);
    draw::line(logic::Point{x: 105, y: 45}, 155, 8, theme.overlay, true);
}

pub fn character(pos: logic::Point, theme: theme::Theme) {
    draw::line(logic::Point{x: pos.x, y: pos.y+5}, 70, 15, theme.overlay, false);
    draw::line(logic::Point{x: pos.x, y: pos.y}, 70, 16, theme.accent, false);
}

pub fn info(pos: logic::Point, theme: theme::Theme, text: draw::Text) {
    logic::display::wait_for_vblank();
    draw::line(logic::Point{x: pos.x, y: pos.y+5}, 30, 10, theme.overlay, false);
    draw::line(logic::Point{x: pos.x, y: pos.y}, 30, 12, theme.accent, false);
    draw::line(logic::Point{x: pos.x, y: pos.y}, 30, 8, text.bg_color, false);
    logic::display::draw_string(
        text.text, 
        logic::Point{x: pos.x - 10 + text.xoffset, y: pos.y - 6}, 
        false, logic::Color{rgb565 : text.text_color}, 
        logic::Color{rgb565 : text.bg_color}
    );
}

pub fn theme(theme: theme::Theme) {
    logic::display::wait_for_vblank();
    draw::line(logic::Point{x: 30, y: 30}, 210, 19, theme.subtext, false);
    draw::line(logic::Point{x: 30, y: 25}, 210, 20, theme.overlay, false);
    logic::display::wait_for_vblank();
    draw::line(logic::Point{x: 90, y: 25}, 150, 14, theme.bg, false);
    logic::display::draw_string(
        theme.name, 
        logic::Point{x: 92, y: 17}, 
        true, 
        logic::Color{rgb565 : theme.text}, 
        logic::Color{rgb565 : theme.bg}
    );
    draw::dot(logic::Point{x: 30, y: 25}, 14, theme.tertiary_accent, 0, draw::Fastforwarding{enabled: false, scale: 1.0});
    draw::dot(logic::Point{x: 45, y: 25}, 14, theme.secondary_accent, 0, draw::Fastforwarding{enabled: false, scale: 1.0});
    draw::dot(logic::Point{x: 60, y: 25}, 14, theme.accent, 0, draw::Fastforwarding{enabled: false, scale: 1.0});
}

pub fn difficulty(pos: logic::Point, theme: theme::Theme<'_>) {
    logic::display::wait_for_vblank();
    draw::line(logic::Point{x: pos.x, y: pos.y+5}, 70, 16, theme.overlay, false);
    draw::line(logic::Point{x: pos.x, y: pos.y}, 70, 16, theme.accent, false);
    draw::line(logic::Point{x: pos.x, y: pos.y}, 70, 6, theme.bg, false);
    draw::dot(logic::Point{x: pos.x, y: pos.y}, 10, theme.tertiary_accent, 0, draw::Fastforwarding{enabled: false, scale: 1.0});
}

pub fn slider(pos: logic::Point, theme: theme::Theme<'_>, level: u8) {
    logic::timing::msleep(50);
    logic::display::wait_for_vblank();
    for i in 1..(14*level).into() {
        draw::dot(logic::Point{x: pos.x+i, y: pos.y}, 10, theme.tertiary_accent, 1500, draw::Fastforwarding{enabled: false, scale: 1.0});
    }
}