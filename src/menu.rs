use crate::draw;
use crate::logic;
use crate::theme;
use crate::characters::hiragana;
use crate::characters::katakana;

pub fn rect(rect: logic::Rect, theme: theme::Theme, thickness: u16) {
    logic::display::wait_for_vblank();
    logic::display::push_rect_uniform(logic::Rect{x : rect.x - 1, y: rect.y, width: rect.width, height: rect.height + 1}, logic::Color{rgb565: theme.bg});
    draw::rounded_rect(logic::Point{x: rect.x, y: rect.y + 1}, rect.width, rect.height, thickness-2, theme.text);
    draw::rounded_rect(logic::Point{x: rect.x, y: rect.y - 4}, rect.width, rect.height, thickness, theme.accent);
}

pub fn rect_selected(rect: logic::Rect, theme: theme::Theme, thickness: u16) {
    logic::display::wait_for_vblank();
    logic::display::push_rect_uniform(logic::Rect{x : rect.x - 5, y: rect.y, width: rect.width, height: rect.height + 5}, logic::Color{rgb565: theme.bg});
    draw::rounded_rect(logic::Point{x: rect.x, y: rect.y + 1}, rect.width, rect.height, thickness-2, theme.text);
    draw::rounded_rect(logic::Point{x: rect.x, y: rect.y - 10}, rect.width, rect.height, thickness, theme.accent);
}

pub fn rect_pressed(rect: logic::Rect, theme: theme::Theme, thickness: u16) {
    logic::display::wait_for_vblank();
    logic::display::push_rect_uniform(logic::Rect{x : rect.x - 5, y: rect.y, width: rect.width, height: rect.height + 3}, logic::Color{rgb565: theme.bg});
    draw::rounded_rect(logic::Point{x: rect.x, y: rect.y + 1}, rect.width, rect.height, thickness-2, theme.text);
    draw::rounded_rect(logic::Point{x: rect.x, y: rect.y - 6}, rect.width, rect.height, thickness, theme.accent);
}

pub fn chracter_widget(pos: logic::Point, theme: theme::Theme, text: draw::Text) {
    draw::line(logic::Point{x: pos.x, y: pos.y+5}, 40, 14, theme.subtext, false);
    draw::line(logic::Point{x: pos.x, y: pos.y}, 40, 15, theme.accent, false);
    draw::string(text.text, logic::Point{x: pos.x + (15/text.text.chars().count()) as u16, y: pos.y-10}, text.big, text.text_color, text.bg_color, text.delay);
}

pub fn theme_widget(theme: theme::Theme) {
    logic::display::wait_for_vblank();
    draw::line(logic::Point{x: 30, y: 30}, 210, 19, theme.subtext, false);
    draw::line(logic::Point{x: 30, y: 25}, 210, 20, theme.overlay, false);
    logic::display::wait_for_vblank();
    draw::line(logic::Point{x: 90, y: 25}, 150, 14, theme.bg, false);
    draw::string(theme.name, logic::Point{x: 92, y: 17}, true, theme.text, theme.bg, 0);
    draw::dot(logic::Point{x: 30, y: 25}, 14, theme.tertiary_accent, 0, draw::Fastforwarding{enabled: false, scale: 1.0});
    draw::dot(logic::Point{x: 45, y: 25}, 14, theme.secondary_accent, 0, draw::Fastforwarding{enabled: false, scale: 1.0});
    draw::dot(logic::Point{x: 60, y: 25}, 14, theme.accent, 0, draw::Fastforwarding{enabled: false, scale: 1.0});
}

pub fn difficulty_widget(pos: logic::Point, theme: theme::Theme, level: u8) {
    logic::display::wait_for_vblank();
    draw::line(logic::Point{x: pos.x, y: pos.y+5}, 50, 10, theme.subtext, false);
    draw::line(logic::Point{x: pos.x, y: pos.y}, 50, 10, theme.accent, false);
    draw::line(logic::Point{x: pos.x, y: pos.y}, 50, 6, theme.bg, false);
    if level == 1 {
        draw::line(logic::Point{x: pos.x, y: pos.y}, 10, 5, theme.tertiary_accent, false);
    } else if level == 2 {
        draw::line(logic::Point{x: pos.x, y: pos.y}, 20, 5, theme.tertiary_accent, false);
    } else if level == 3 {
        draw::line(logic::Point{x: pos.x, y: pos.y}, 30, 5, theme.tertiary_accent, false);
    } else if level == 4 {
        draw::line(logic::Point{x: pos.x, y: pos.y}, 40, 5, theme.tertiary_accent, false);
    } else if level == 5 {
        draw::line(logic::Point{x: pos.x, y: pos.y}, 50, 5, theme.tertiary_accent, false);
    }
}

pub fn page() {
    let mut theme = theme::theme(None);
    let mut selected_character: u8 = 0;
    let mut characters_switched: bool = false;
    let mut needs_redraw: bool = true;
    let mut shift_pressed: bool = false;
    'page: loop {
        logic::display::wait_for_vblank();
        if needs_redraw {
            draw::clearscreen(theme.bg);
            rect(logic::Rect{x: 10, y: 20, width: 190, height: 210}, theme, 6);
            rect(logic::Rect{x: 220, y: 20, width: 90, height: 210}, theme, 6);
            if characters_switched {
                match selected_character {
                    0 => {
                        chracter_widget(logic::Point{x: 245, y: 50}, theme, draw::Text{text: "A", big: true, text_color: theme.text, bg_color: theme.accent, delay: 0});
                        difficulty_widget(logic::Point{x: 240, y: 90},theme, 2);
                        hiragana::a(logic::Point{x: 240, y: 145}, 0.5, 4, theme.overlay, 0, draw::Fastforwarding{enabled: false, scale: 1.0});
                        hiragana::a(logic::Point{x: 240, y: 140}, 0.5, 4, theme.secondary_accent, 0, draw::Fastforwarding{enabled: false, scale: 1.0});
                        katakana::a(logic::Point{x: 40, y: 40}, 1.2, 6, theme.secondary_accent, 10000, draw::Fastforwarding{enabled: true, scale: 3.0});
                    }
                    1 => {
                        chracter_widget(logic::Point{x: 245, y: 50}, theme, draw::Text{text: "I", big: true, text_color: theme.text, bg_color: theme.accent, delay: 0});
                        difficulty_widget(logic::Point{x: 240, y: 90},theme, 1);
                        hiragana::i(logic::Point{x: 240, y: 145}, 0.5, 4, theme.overlay, 0, draw::Fastforwarding{enabled: false, scale: 1.0});
                        hiragana::i(logic::Point{x: 240, y: 140}, 0.5, 4, theme.secondary_accent, 0, draw::Fastforwarding{enabled: false, scale: 1.0});
                        katakana::i(logic::Point{x: 40, y: 40}, 1.2, 6, theme.secondary_accent, 10000, draw::Fastforwarding{enabled: true, scale: 3.0});
                    }
                    2 => {
                        chracter_widget(logic::Point{x: 245, y: 50}, theme, draw::Text{text: "U", big: true, text_color: theme.text, bg_color: theme.accent, delay: 0});
                        difficulty_widget(logic::Point{x: 240, y: 90},theme, 2);
                        hiragana::u(logic::Point{x: 240, y: 145}, 0.5, 4, theme.overlay, 0, draw::Fastforwarding{enabled: false, scale: 1.0});
                        hiragana::u(logic::Point{x: 240, y: 140}, 0.5, 4, theme.secondary_accent, 0, draw::Fastforwarding{enabled: false, scale: 1.0});
                        katakana::u(logic::Point{x: 40, y: 40}, 1.2, 6, theme.secondary_accent, 10000, draw::Fastforwarding{enabled: true, scale: 3.0});
                    }
                    3 => {
                        chracter_widget(logic::Point{x: 245, y: 50}, theme, draw::Text{text: "E", big: true, text_color: theme.text, bg_color: theme.accent, delay: 0});
                        difficulty_widget(logic::Point{x: 240, y: 90},theme, 1);
                        hiragana::e(logic::Point{x: 240, y: 145}, 0.5, 4, theme.overlay, 0, draw::Fastforwarding{enabled: false, scale: 1.0});
                        hiragana::e(logic::Point{x: 240, y: 140}, 0.5, 4, theme.secondary_accent, 0, draw::Fastforwarding{enabled: false, scale: 1.0});
                        katakana::e(logic::Point{x: 40, y: 40}, 1.2, 6, theme.secondary_accent, 10000, draw::Fastforwarding{enabled: true, scale: 3.0});
                    }
                    _ => {}
                }
            } else {
                match selected_character {
                    0 => {
                        chracter_widget(logic::Point{x: 245, y: 50}, theme, draw::Text{text: "A", big: true, text_color: theme.text, bg_color: theme.accent, delay: 0});
                        difficulty_widget(logic::Point{x: 240, y: 90},theme, 4);
                        katakana::a(logic::Point{x: 240, y: 145}, 0.5, 4, theme.overlay, 0, draw::Fastforwarding{enabled: false, scale: 1.0});
                        katakana::a(logic::Point{x: 240, y: 140}, 0.5, 4, theme.secondary_accent, 0, draw::Fastforwarding{enabled: false, scale: 1.0});
                        hiragana::a(logic::Point{x: 40, y: 40}, 1.2, 6, theme.secondary_accent, 10000, draw::Fastforwarding{enabled: true, scale: 4.0});
                    }
                    1 => {
                        chracter_widget(logic::Point{x: 245, y: 50}, theme, draw::Text{text: "I", big: true, text_color: theme.text, bg_color: theme.accent, delay: 0});
                        difficulty_widget(logic::Point{x: 240, y: 90},theme, 2);
                        katakana::i(logic::Point{x: 240, y: 145}, 0.5, 4, theme.overlay, 0, draw::Fastforwarding{enabled: false, scale: 1.0});
                        katakana::i(logic::Point{x: 240, y: 140}, 0.5, 4, theme.secondary_accent, 0, draw::Fastforwarding{enabled: false, scale: 1.0});
                        hiragana::i(logic::Point{x: 40, y: 40}, 1.2, 6, theme.secondary_accent, 10000, draw::Fastforwarding{enabled: true, scale: 4.0});
                    }
                    2 => {
                        chracter_widget(logic::Point{x: 245, y: 50}, theme, draw::Text{text: "U", big: true, text_color: theme.text, bg_color: theme.accent, delay: 0});
                        difficulty_widget(logic::Point{x: 240, y: 90},theme, 3);
                        katakana::u(logic::Point{x: 240, y: 145}, 0.5, 4, theme.overlay, 0, draw::Fastforwarding{enabled: false, scale: 1.0});
                        katakana::u(logic::Point{x: 240, y: 140}, 0.5, 4, theme.secondary_accent, 0, draw::Fastforwarding{enabled: false, scale: 1.0});
                        hiragana::u(logic::Point{x: 40, y: 40}, 1.2, 6, theme.secondary_accent, 10000, draw::Fastforwarding{enabled: true, scale: 4.0});
                    }
                    3 => {
                        chracter_widget(logic::Point{x: 245, y: 50}, theme, draw::Text{text: "E", big: true, text_color: theme.text, bg_color: theme.accent, delay: 0});
                        difficulty_widget(logic::Point{x: 240, y: 90},theme, 4);
                        katakana::e(logic::Point{x: 240, y: 145}, 0.5, 4, theme.overlay, 0, draw::Fastforwarding{enabled: false, scale: 1.0});
                        katakana::e(logic::Point{x: 240, y: 140}, 0.5, 4, theme.secondary_accent, 0, draw::Fastforwarding{enabled: false, scale: 1.0});
                        hiragana::e(logic::Point{x: 40, y: 40}, 1.2, 6, theme.secondary_accent, 10000, draw::Fastforwarding{enabled: true, scale: 4.0});
                    }
                    _ => {}
                }
            }
            logic::timing::msleep(500);
            shift_pressed = false;
            needs_redraw = false;
        }
        if logic::keypress(logic::keyboard::Key::Left) && shift_pressed {
            if theme.id == 0 {
                theme = theme::theme(Some(4));
            } else {
                theme = theme::theme(Some(theme.id - 1));
            }
            theme_widget(theme);
            logic::timing::msleep(100);
        } else if logic::keypress(logic::keyboard::Key::Right) && shift_pressed {
            if theme.id == 4 {
                theme = theme::theme(Some(0));
            } else {
                theme = theme::theme(Some(theme.id + 1));
            }
            theme_widget(theme);
            logic::timing::msleep(100);
        } else if logic::keypress(logic::keyboard::Key::Left) && !shift_pressed {
            if selected_character > 0 {
                selected_character -= 1;
                needs_redraw = true;
            }
        } else if logic::keypress(logic::keyboard::Key::Right) && !shift_pressed {
            if selected_character < 3 {
                selected_character += 1;
                needs_redraw = true;
            }
        } else if logic::keypress(logic::keyboard::Key::Backspace) && !shift_pressed {
            characters_switched = !characters_switched;
            needs_redraw = true;
        } else if logic::keypress(logic::keyboard::Key::Back) {
            if shift_pressed {
                theme = theme::theme(None);
            };
            needs_redraw = true;
        } else if logic::keypress(logic::keyboard::Key::Ok) {
            shift_pressed = false;
            needs_redraw = true;
        } else if logic::keypress(logic::keyboard::Key::Shift) && !shift_pressed {
            shift_pressed = true;
            theme_widget(theme);
            logic::timing::msleep(200);
        } else if logic::keypress(logic::keyboard::Key::Home) {
            break 'page;
        };
        logic::timing::msleep(10);
    }
}