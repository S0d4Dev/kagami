use crate::{core::{draw,logic,theme}, characters::{hiragana, katakana}};

fn clear_characters(theme: theme::Theme) {
    logic::display::push_rect_uniform(logic::Rect{x: 230, y: 150, width: 70, height: 70}, theme.bg);
    logic::display::push_rect_uniform(logic::Rect{x: 30, y: 30, width: 160, height: 180}, theme.bg);
}

fn canva(theme: theme::Theme) {
    logic::display::push_rect_uniform(logic::Rect{ x: 0, y: 0, width: 320, height: 240}, theme.bg);
    rect(logic::Rect{x: 10, y: 20, width: 190, height: 210}, theme, 6);
    rect(logic::Rect{x: 220, y: 140, width: 90, height: 90}, theme, 6);
    info(logic::Point{x: 240, y: 135}, theme);
    logic::display::draw_string("clear\0", logic::Point{x: 237, y: 129}, false, theme.subtext, theme.bg);
    info(logic::Point{x: 90, y: 15}, theme);
    info(logic::Point{x: 145, y: 15}, theme);
    logic::display::draw_string("EXE\0", logic::Point{x: 150, y: 9}, false, theme.subtext, theme.bg);
    info(logic::Point{x: 222, y: 16}, theme);
    logic::display::draw_string("shift\0", logic::Point{x: 219, y: 10}, false, theme.subtext, theme.bg);
    info(logic::Point{x: 275, y: 16}, theme);
    draw::line(logic::Point{x: 275, y: 16}, 30, 8, theme.secondary_accent, false);
    logic::display::draw_string("paste\0", logic::Point{x: 273, y: 10}, false, theme.text, theme.secondary_accent);
    draw::line(logic::Point{x: 230, y: 55}, 70, 15, theme.overlay, false);
    draw::line(logic::Point{x: 230, y: 50}, 70, 16, theme.accent, false);
}

fn rect(rect: logic::Rect, theme: theme::Theme, thickness: u16) {
    logic::display::wait_for_vblank();
    logic::display::push_rect_uniform(
        logic::Rect{x : rect.x - 1, y: rect.y, width: rect.width, height: rect.height + 1}, 
        theme.bg
    );
    draw::rounded_rect(logic::Point{x: rect.x, y: rect.y + 2}, rect.width, rect.height, thickness-2, theme.overlay);
    draw::rounded_rect(logic::Point{x: rect.x, y: rect.y - 4}, rect.width, rect.height, thickness, theme.accent);
}

fn info(pos: logic::Point, theme: theme::Theme) {
    logic::display::wait_for_vblank();
    draw::line(logic::Point{x: pos.x, y: pos.y+5}, 30, 10, theme.overlay, false);
    draw::line(logic::Point{x: pos.x, y: pos.y}, 30, 12, theme.accent, false);
    draw::line(logic::Point{x: pos.x, y: pos.y}, 30, 8, theme.bg, false);
}

fn theme_preview(theme: theme::Theme) {
    logic::display::wait_for_vblank();
    draw::line(logic::Point{x: 30, y: 30}, 210, 19, theme.subtext, false);
    draw::line(logic::Point{x: 30, y: 25}, 210, 20, theme.overlay, false);
    logic::display::wait_for_vblank();
    draw::line(logic::Point{x: 90, y: 25}, 150, 14, theme.bg, false);
    logic::display::draw_string(
        theme.name, 
        logic::Point{x: 92, y: 17}, 
        true, 
        theme.text, 
        theme.bg
    );
    draw::dot(logic::Point{x: 30, y: 25}, 14, theme.tertiary_accent, 0, draw::Fastforwarding{enabled: false, scale: 1.0});
    draw::dot(logic::Point{x: 45, y: 25}, 14, theme.secondary_accent, 0, draw::Fastforwarding{enabled: false, scale: 1.0});
    draw::dot(logic::Point{x: 60, y: 25}, 14, theme.accent, 0, draw::Fastforwarding{enabled: false, scale: 1.0});
}

pub fn page() {
    let mut theme = theme::theme(None);
    let themecount = theme::themecount();
    let mut selected_character: u8 = 0;
    let mut characters_switched: bool = false;
    let mut needs_redraw: bool = true;
    let mut shift_pressed: bool = false;
    let mut difficulty: u8 = 1;
    canva(theme);
    'page: loop {
        logic::display::wait_for_vblank();
        if needs_redraw {
            logic::display::draw_string("+ / -\0", logic::Point{x: 88, y: 9}, false, theme.subtext, theme.bg);
            clear_characters(theme);
            logic::display::wait_for_vblank();
            draw::line(logic::Point{x: 230, y: 95}, 70, 16, theme.overlay, false);
            draw::line(logic::Point{x: 230, y: 90}, 70, 16, theme.accent, false);
            draw::line(logic::Point{x: 230, y: 90}, 70, 6, theme.bg, false);
            draw::dot(logic::Point{x: 230, y: 90}, 10, theme.tertiary_accent, 0, draw::Fastforwarding{enabled: false, scale: 1.0});
            draw::line(logic::Point{x: 30, y: 120}, 150, 8, theme.overlay, false);
            draw::line(logic::Point{x: 105, y: 45}, 155, 8, theme.overlay, true);
            if characters_switched {
                match selected_character {
                    0 => {
                        logic::display::draw_string("A\0", logic::Point{x: 260, y: 40}, true, theme.text, theme.accent);
                        hiragana::a(logic::Point{x: 245, y: 160}, 0.4, 4, theme.overlay, 0, draw::Fastforwarding{enabled: false, scale: 1.0});
                        hiragana::a(logic::Point{x: 245, y: 155}, 0.4, 4, theme.secondary_accent, 0, draw::Fastforwarding{enabled: false, scale: 1.0});
                        katakana::a(logic::Point{x: 40, y: 40}, 1.2, 6, theme.secondary_accent, 10000, draw::Fastforwarding{enabled: true, scale: 3.0});
                        difficulty = 2;
                    }
                    1 => {
                        logic::display::draw_string("I\0", logic::Point{x: 260, y: 40}, true, theme.text, theme.accent);
                        hiragana::i(logic::Point{x: 245, y: 160}, 0.4, 4, theme.overlay, 0, draw::Fastforwarding{enabled: false, scale: 1.0});
                        hiragana::i(logic::Point{x: 245, y: 155}, 0.4, 4, theme.secondary_accent, 0, draw::Fastforwarding{enabled: false, scale: 1.0});
                        katakana::i(logic::Point{x: 40, y: 40}, 1.2, 6, theme.secondary_accent, 10000, draw::Fastforwarding{enabled: true, scale: 3.0});
                        difficulty = 1;
                    }
                    2 => {
                        logic::display::draw_string("U\0", logic::Point{x: 260, y: 40}, true, theme.text, theme.accent);
                        hiragana::u(logic::Point{x: 245, y: 160}, 0.4, 4, theme.overlay, 0, draw::Fastforwarding{enabled: false, scale: 1.0});
                        hiragana::u(logic::Point{x: 245, y: 155}, 0.4, 4, theme.secondary_accent, 0, draw::Fastforwarding{enabled: false, scale: 1.0});
                        katakana::u(logic::Point{x: 40, y: 40}, 1.2, 6, theme.secondary_accent, 10000, draw::Fastforwarding{enabled: true, scale: 3.0});
                        difficulty = 2;
                    }
                    3 => {
                        logic::display::draw_string("E\0", logic::Point{x: 260, y: 40}, true, theme.text, theme.accent);
                        hiragana::e(logic::Point{x: 240, y: 160}, 0.5, 4, theme.overlay, 0, draw::Fastforwarding{enabled: false, scale: 1.0});
                        hiragana::e(logic::Point{x: 240, y: 155}, 0.5, 4, theme.secondary_accent, 0, draw::Fastforwarding{enabled: false, scale: 1.0});
                        katakana::e(logic::Point{x: 40, y: 40}, 1.2, 6, theme.secondary_accent, 10000, draw::Fastforwarding{enabled: true, scale: 3.0});
                        difficulty = 1;
                    }
                    4 => {
                        logic::display::draw_string("O\0", logic::Point{x: 260, y: 40}, true, theme.text, theme.accent);
                        hiragana::o(logic::Point{x: 240, y: 160}, 0.5, 4, theme.overlay, 0, draw::Fastforwarding{enabled: false, scale: 1.0});
                        hiragana::o(logic::Point{x: 240, y: 155}, 0.5, 4, theme.secondary_accent, 0, draw::Fastforwarding{enabled: false, scale: 1.0});
                        katakana::o(logic::Point{x: 40, y: 40}, 1.2, 6, theme.secondary_accent, 10000, draw::Fastforwarding{enabled: true, scale: 3.0});
                        difficulty = 1;
                    }
                    5 => {
                        logic::display::draw_string("Ka\0", logic::Point{x: 255, y: 40}, true, theme.text, theme.accent);
                        hiragana::ka(logic::Point{x: 240, y: 160}, 0.5, 4, theme.overlay, 0, draw::Fastforwarding{enabled: false, scale: 1.0});
                        hiragana::ka(logic::Point{x: 240, y: 155}, 0.5, 4, theme.secondary_accent, 0, draw::Fastforwarding{enabled: false, scale: 1.0});
                        katakana::ka(logic::Point{x: 40, y: 40}, 1.2, 6, theme.secondary_accent, 10000, draw::Fastforwarding{enabled: true, scale: 3.0});
                        difficulty = 1;
                    }
                    6 => {
                        logic::display::draw_string("Ki\0", logic::Point{x: 255, y: 40}, true, theme.text, theme.accent);
                        hiragana::ki(logic::Point{x: 240, y: 160}, 0.5, 4, theme.overlay, 0, draw::Fastforwarding{enabled: false, scale: 1.0});
                        hiragana::ki(logic::Point{x: 240, y: 155}, 0.5, 4, theme.secondary_accent, 0, draw::Fastforwarding{enabled: false, scale: 1.0});
                        katakana::ki(logic::Point{x: 40, y: 40}, 1.2, 6, theme.secondary_accent, 10000, draw::Fastforwarding{enabled: true, scale: 3.0});
                        difficulty = 1;
                    }
                    7 => {
                        logic::display::draw_string("Ku\0", logic::Point{x: 255, y: 40}, true, theme.text, theme.accent);
                        hiragana::ku(logic::Point{x: 240, y: 160}, 0.5, 4, theme.overlay, 0, draw::Fastforwarding{enabled: false, scale: 1.0});
                        hiragana::ku(logic::Point{x: 240, y: 155}, 0.5, 4, theme.secondary_accent, 0, draw::Fastforwarding{enabled: false, scale: 1.0});
                        katakana::ku(logic::Point{x: 40, y: 40}, 1.2, 6, theme.secondary_accent, 10000, draw::Fastforwarding{enabled: true, scale: 3.0});
                        difficulty = 1;
                    }
                    8 => {
                        logic::display::draw_string("Ke\0", logic::Point{x: 255, y: 40}, true, theme.text, theme.accent);
                        hiragana::ke(logic::Point{x: 240, y: 160}, 0.5, 4, theme.overlay, 0, draw::Fastforwarding{enabled: false, scale: 1.0});
                        hiragana::ke(logic::Point{x: 240, y: 155}, 0.5, 4, theme.secondary_accent, 0, draw::Fastforwarding{enabled: false, scale: 1.0});
                        katakana::ke(logic::Point{x: 40, y: 40}, 1.2, 6, theme.secondary_accent, 10000, draw::Fastforwarding{enabled: true, scale: 3.0});
                        difficulty = 1;
                    }
                    9 => {
                        logic::display::draw_string("Ko\0", logic::Point{x: 255, y: 40}, true, theme.text, theme.accent);
                        hiragana::ko(logic::Point{x: 240, y: 160}, 0.5, 4, theme.overlay, 0, draw::Fastforwarding{enabled: false, scale: 1.0});
                        hiragana::ko(logic::Point{x: 240, y: 155}, 0.5, 4, theme.secondary_accent, 0, draw::Fastforwarding{enabled: false, scale: 1.0});
                        katakana::ko(logic::Point{x: 40, y: 40}, 1.2, 6, theme.secondary_accent, 10000, draw::Fastforwarding{enabled: true, scale: 3.0});
                        difficulty = 1;
                    }
                    10 => {
                        logic::display::draw_string("Sa\0", logic::Point{x: 255, y: 40}, true, theme.text, theme.accent);
                        hiragana::sa(logic::Point{x: 240, y: 160}, 0.5, 4, theme.overlay, 0, draw::Fastforwarding{enabled: false, scale: 1.0});
                        hiragana::sa(logic::Point{x: 240, y: 155}, 0.5, 4, theme.secondary_accent, 0, draw::Fastforwarding{enabled: false, scale: 1.0});
                        katakana::sa(logic::Point{x: 40, y: 40}, 1.2, 6, theme.secondary_accent, 10000, draw::Fastforwarding{enabled: true, scale: 3.0});
                        difficulty = 1;
                    }
                    12 => {
                        logic::display::draw_string("Su\0", logic::Point{x: 255, y: 40}, true, theme.text, theme.accent);
                        hiragana::su(logic::Point{x: 240, y: 160}, 0.5, 4, theme.overlay, 0, draw::Fastforwarding{enabled: false, scale: 1.0});
                        hiragana::su(logic::Point{x: 240, y: 155}, 0.5, 4, theme.secondary_accent, 0, draw::Fastforwarding{enabled: false, scale: 1.0});
                        katakana::su(logic::Point{x: 40, y: 40}, 1.2, 6, theme.secondary_accent, 10000, draw::Fastforwarding{enabled: true, scale: 3.0});
                        difficulty = 1;
                    }
                    13 => {
                        logic::display::draw_string("Se\0", logic::Point{x: 255, y: 40}, true, theme.text, theme.accent);
                        hiragana::se(logic::Point{x: 240, y: 160}, 0.5, 4, theme.overlay, 0, draw::Fastforwarding{enabled: false, scale: 1.0});
                        hiragana::se(logic::Point{x: 240, y: 155}, 0.5, 4, theme.secondary_accent, 0, draw::Fastforwarding{enabled: false, scale: 1.0});
                        katakana::se(logic::Point{x: 40, y: 40}, 1.2, 6, theme.secondary_accent, 10000, draw::Fastforwarding{enabled: true, scale: 3.0});
                        difficulty = 1;
                    }
                    14 => {
                        logic::display::draw_string("So\0", logic::Point{x: 255, y: 40}, true, theme.text, theme.accent);
                        hiragana::so(logic::Point{x: 240, y: 160}, 0.5, 4, theme.overlay, 0, draw::Fastforwarding{enabled: false, scale: 1.0});
                        hiragana::so(logic::Point{x: 240, y: 155}, 0.5, 4, theme.secondary_accent, 0, draw::Fastforwarding{enabled: false, scale: 1.0});
                        katakana::so(logic::Point{x: 40, y: 40}, 1.2, 6, theme.secondary_accent, 10000, draw::Fastforwarding{enabled: true, scale: 3.0});
                        difficulty = 1;
                    }
                    _ => {}
                }
            } else {
                match selected_character {
                    0 => {
                        logic::display::draw_string("A\0", logic::Point{x: 260, y: 40}, true, theme.text, theme.accent);
                        katakana::a(logic::Point{x: 235, y: 155}, 0.5, 4, theme.overlay, 0, draw::Fastforwarding{enabled: false, scale: 1.0});
                        katakana::a(logic::Point{x: 235, y: 150}, 0.5, 4, theme.secondary_accent, 0, draw::Fastforwarding{enabled: false, scale: 1.0});
                        hiragana::a(logic::Point{x: 40, y: 40}, 1.2, 6, theme.secondary_accent, 10000, draw::Fastforwarding{enabled: true, scale: 4.0});
                        difficulty = 3;
                    }
                    1 => {
                        logic::display::draw_string("I\0", logic::Point{x: 260, y: 40}, true, theme.text, theme.accent);
                        katakana::i(logic::Point{x: 240, y: 155}, 0.5, 4, theme.overlay, 0, draw::Fastforwarding{enabled: false, scale: 1.0});
                        katakana::i(logic::Point{x: 240, y: 150}, 0.5, 4, theme.secondary_accent, 0, draw::Fastforwarding{enabled: false, scale: 1.0});
                        hiragana::i(logic::Point{x: 40, y: 40}, 1.2, 3, theme.secondary_accent, 10000, draw::Fastforwarding{enabled: true, scale: 4.0});
                        difficulty = 2;
                    }
                    2 => {
                        logic::display::draw_string("U\0", logic::Point{x: 260, y: 40}, true, theme.text, theme.accent);
                        katakana::u(logic::Point{x: 240, y: 155}, 0.5, 4, theme.overlay, 0, draw::Fastforwarding{enabled: false, scale: 1.0});
                        katakana::u(logic::Point{x: 240, y: 150}, 0.5, 4, theme.secondary_accent, 0, draw::Fastforwarding{enabled: false, scale: 1.0});
                        hiragana::u(logic::Point{x: 40, y: 40}, 1.2, 3, theme.secondary_accent, 10000, draw::Fastforwarding{enabled: true, scale: 4.0});
                        difficulty = 3;
                    }
                    3 => {
                        logic::display::draw_string("E\0", logic::Point{x: 260, y: 40}, true, theme.text, theme.accent);
                        katakana::e(logic::Point{x: 240, y: 155}, 0.5, 4, theme.overlay, 0, draw::Fastforwarding{enabled: false, scale: 1.0});
                        katakana::e(logic::Point{x: 240, y: 150}, 0.5, 4, theme.secondary_accent, 0, draw::Fastforwarding{enabled: false, scale: 1.0});
                        hiragana::e(logic::Point{x: 40, y: 40}, 1.2, 3, theme.secondary_accent, 10000, draw::Fastforwarding{enabled: true, scale: 4.0});
                        difficulty = 4;
                    }
                    4 => {
                        logic::display::draw_string("O\0", logic::Point{x: 260, y: 40}, true, theme.text, theme.accent);
                        katakana::o(logic::Point{x: 240, y: 160}, 0.5, 4, theme.overlay, 0, draw::Fastforwarding{enabled: false, scale: 1.0});
                        katakana::o(logic::Point{x: 240, y: 155}, 0.5, 4, theme.secondary_accent, 0, draw::Fastforwarding{enabled: false, scale: 1.0});
                        hiragana::o(logic::Point{x: 40, y: 40}, 1.2, 6, theme.secondary_accent, 10000, draw::Fastforwarding{enabled: true, scale: 3.0});
                        difficulty = 1;
                    }
                    5 => {
                        logic::display::draw_string("Ka\0", logic::Point{x: 255, y: 40}, true, theme.text, theme.accent);
                        katakana::ka(logic::Point{x: 240, y: 160}, 0.5, 4, theme.overlay, 0, draw::Fastforwarding{enabled: false, scale: 1.0});
                        katakana::ka(logic::Point{x: 240, y: 155}, 0.5, 4, theme.secondary_accent, 0, draw::Fastforwarding{enabled: false, scale: 1.0});
                        hiragana::ka(logic::Point{x: 40, y: 40}, 1.2, 6, theme.secondary_accent, 10000, draw::Fastforwarding{enabled: true, scale: 3.0});
                        difficulty = 1;
                    }
                    6 => {
                        logic::display::draw_string("Ki\0", logic::Point{x: 255, y: 40}, true, theme.text, theme.accent);
                        katakana::ki(logic::Point{x: 240, y: 160}, 0.5, 4, theme.overlay, 0, draw::Fastforwarding{enabled: false, scale: 1.0});
                        katakana::ki(logic::Point{x: 240, y: 155}, 0.5, 4, theme.secondary_accent, 0, draw::Fastforwarding{enabled: false, scale: 1.0});
                        hiragana::ki(logic::Point{x: 40, y: 40}, 1.2, 6, theme.secondary_accent, 10000, draw::Fastforwarding{enabled: true, scale: 3.0});
                        difficulty = 1;
                    }
                    7 => {
                        logic::display::draw_string("Ku\0", logic::Point{x: 255, y: 40}, true, theme.text, theme.accent);
                        katakana::ku(logic::Point{x: 240, y: 160}, 0.5, 4, theme.overlay, 0, draw::Fastforwarding{enabled: false, scale: 1.0});
                        katakana::ku(logic::Point{x: 240, y: 155}, 0.5, 4, theme.secondary_accent, 0, draw::Fastforwarding{enabled: false, scale: 1.0});
                        hiragana::ku(logic::Point{x: 40, y: 40}, 1.2, 6, theme.secondary_accent, 10000, draw::Fastforwarding{enabled: true, scale: 3.0});
                        difficulty = 1;
                    }
                    8 => {
                        logic::display::draw_string("Ke\0", logic::Point{x: 255, y: 40}, true, theme.text, theme.accent);
                        katakana::ke(logic::Point{x: 240, y: 160}, 0.5, 4, theme.overlay, 0, draw::Fastforwarding{enabled: false, scale: 1.0});
                        katakana::ke(logic::Point{x: 240, y: 155}, 0.5, 4, theme.secondary_accent, 0, draw::Fastforwarding{enabled: false, scale: 1.0});
                        hiragana::ke(logic::Point{x: 40, y: 40}, 1.2, 6, theme.secondary_accent, 10000, draw::Fastforwarding{enabled: true, scale: 3.0});
                        difficulty = 1;
                    }
                    9 => {
                        logic::display::draw_string("Ko\0", logic::Point{x: 255, y: 40}, true, theme.text, theme.accent);
                        katakana::ko(logic::Point{x: 240, y: 160}, 0.5, 4, theme.overlay, 0, draw::Fastforwarding{enabled: false, scale: 1.0});
                        katakana::ko(logic::Point{x: 240, y: 155}, 0.5, 4, theme.secondary_accent, 0, draw::Fastforwarding{enabled: false, scale: 1.0});
                        hiragana::ko(logic::Point{x: 40, y: 40}, 1.2, 6, theme.secondary_accent, 10000, draw::Fastforwarding{enabled: true, scale: 3.0});
                        difficulty = 1;
                    }
                    10 => {
                        logic::display::draw_string("Sa\0", logic::Point{x: 255, y: 40}, true, theme.text, theme.accent);
                        katakana::sa(logic::Point{x: 240, y: 160}, 0.5, 4, theme.overlay, 0, draw::Fastforwarding{enabled: false, scale: 1.0});
                        katakana::sa(logic::Point{x: 240, y: 155}, 0.5, 4, theme.secondary_accent, 0, draw::Fastforwarding{enabled: false, scale: 1.0});
                        hiragana::sa(logic::Point{x: 40, y: 40}, 1.2, 6, theme.secondary_accent, 10000, draw::Fastforwarding{enabled: true, scale: 3.0});
                        difficulty = 1;
                    }
                    11 => {
                        logic::display::draw_string("Shi\0", logic::Point{x: 255, y: 40}, true, theme.text, theme.accent);
                        katakana::shi(logic::Point{x: 240, y: 160}, 0.5, 4, theme.overlay, 0, draw::Fastforwarding{enabled: false, scale: 1.0});
                        katakana::shi(logic::Point{x: 240, y: 155}, 0.5, 4, theme.secondary_accent, 0, draw::Fastforwarding{enabled: false, scale: 1.0});
                        hiragana::shi(logic::Point{x: 40, y: 40}, 1.2, 6, theme.secondary_accent, 10000, draw::Fastforwarding{enabled: true, scale: 3.0});
                        difficulty = 1;
                    }
                    12 => {
                        logic::display::draw_string("Su\0", logic::Point{x: 255, y: 40}, true, theme.text, theme.accent);
                        katakana::su(logic::Point{x: 240, y: 160}, 0.5, 4, theme.overlay, 0, draw::Fastforwarding{enabled: false, scale: 1.0});
                        katakana::su(logic::Point{x: 240, y: 155}, 0.5, 4, theme.secondary_accent, 0, draw::Fastforwarding{enabled: false, scale: 1.0});
                        hiragana::su(logic::Point{x: 40, y: 40}, 1.2, 6, theme.secondary_accent, 10000, draw::Fastforwarding{enabled: true, scale: 3.0});
                        difficulty = 1;
                    }
                    13 => {
                        logic::display::draw_string("Se\0", logic::Point{x: 255, y: 40}, true, theme.text, theme.accent);
                        katakana::se(logic::Point{x: 240, y: 160}, 0.5, 4, theme.overlay, 0, draw::Fastforwarding{enabled: false, scale: 1.0});
                        katakana::se(logic::Point{x: 240, y: 155}, 0.5, 4, theme.secondary_accent, 0, draw::Fastforwarding{enabled: false, scale: 1.0});
                        hiragana::se(logic::Point{x: 40, y: 40}, 1.2, 6, theme.secondary_accent, 10000, draw::Fastforwarding{enabled: true, scale: 3.0});
                        difficulty = 1;
                    }
                    14 => {
                        logic::display::draw_string("So\0", logic::Point{x: 255, y: 40}, true, theme.text, theme.accent);
                        katakana::so(logic::Point{x: 240, y: 160}, 0.5, 4, theme.overlay, 0, draw::Fastforwarding{enabled: false, scale: 1.0});
                        katakana::so(logic::Point{x: 240, y: 155}, 0.5, 4, theme.secondary_accent, 0, draw::Fastforwarding{enabled: false, scale: 1.0});
                        hiragana::so(logic::Point{x: 40, y: 40}, 1.2, 6, theme.secondary_accent, 10000, draw::Fastforwarding{enabled: true, scale: 3.0});
                        difficulty = 1;
                    }
                    _ => {}
                }
            }
            logic::display::draw_string("< / >\0", logic::Point{x: 88, y: 9}, false, theme.text, theme.bg);
            for i in 1..(14*difficulty).into() {
                draw::dot(logic::Point{x: 230+i, y: 90}, 10, theme.tertiary_accent, 1500, draw::Fastforwarding{enabled: false, scale: 1.0});
            }
            logic::timing::msleep(200);
            shift_pressed = false;
            needs_redraw = false;
        }
        if logic::keypress(logic::keyboard::Key::Left) && shift_pressed {
            if theme.id == 0 {
                theme = theme::theme(Some(themecount-1));
            } else {
                theme = theme::theme(Some(theme.id.saturating_sub(1)));
            }
            theme_preview(theme);
            logic::timing::msleep(100);
        } else if logic::keypress(logic::keyboard::Key::Right) && shift_pressed {
            if theme.id == themecount-1 {
                theme = theme::theme(Some(0));
            } else {
                theme = theme::theme(Some(theme.id.saturating_add(1)));
            }
            theme_preview(theme);
            logic::timing::msleep(100);
        } else if logic::keypress(logic::keyboard::Key::Left) && !shift_pressed {
            if selected_character > 0 {
                selected_character -= 1;
                needs_redraw = true;
                logic::timing::msleep(100);
            }
        } else if logic::keypress(logic::keyboard::Key::Right) && !shift_pressed {
            if selected_character <= 14 {
                selected_character += 1;
                needs_redraw = true;
                logic::timing::msleep(100);
            }
        } else if logic::keypress(logic::keyboard::Key::Backspace) && !shift_pressed {
            characters_switched = !characters_switched;
            needs_redraw = true;
        } else if logic::keypress(logic::keyboard::Key::Back) {
            if shift_pressed {
                canva(theme);
            };
            needs_redraw = true;
        } else if logic::keypress(logic::keyboard::Key::Ok) {
            if shift_pressed {
                shift_pressed = false;
                needs_redraw = true;
                canva(theme);
            }
        } else if logic::keypress(logic::keyboard::Key::Shift) && !shift_pressed {
            shift_pressed = true;
            theme_preview(theme);
            logic::timing::msleep(200);
        } else if logic::keypress(logic::keyboard::Key::Home) {
            break 'page;
        };
        logic::timing::msleep(10);
    }
}