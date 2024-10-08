use crate::draw;
use crate::logic;
use crate::theme;
use crate::widget;
use crate::characters::{hiragana, katakana};

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
            widget::rect_widget(logic::Rect{x: 10, y: 20, width: 190, height: 210}, theme, 6);
            widget::rect_widget(logic::Rect{x: 220, y: 140, width: 90, height: 90}, theme, 6);
            widget::info_widget(logic::Point{x: 240, y: 135}, theme, draw::Text{text: "clear\0", xoffset: 7, big: true, text_color: theme.subtext, bg_color: theme.tertiary_accent});
            widget::info_widget(logic::Point{x: 90, y: 15}, theme, draw::Text{text: "+ / -\0", xoffset: 8, big: true, text_color: theme.subtext, bg_color: theme.tertiary_accent});
            widget::info_widget(logic::Point{x: 145, y: 15}, theme, draw::Text{text: "EXE\0", xoffset: 15, big: true, text_color: theme.subtext, bg_color: theme.tertiary_accent});
            widget::info_widget(logic::Point{x: 222, y: 16}, theme, draw::Text{text: "shift\0", xoffset: 7, big: true, text_color: theme.subtext, bg_color: theme.tertiary_accent});
            widget::info_widget(logic::Point{x: 275, y: 16}, theme, draw::Text{text: "paste\0", xoffset: 7, big: true, text_color: theme.subtext, bg_color: theme.secondary_accent});
            if characters_switched {
                match selected_character {
                    0 => {
                        widget::character_widget(logic::Point{x: 230, y: 50}, theme, draw::Text{text: "A\0", xoffset: 30, big: true, text_color: theme.text, bg_color: theme.accent});
                        widget::difficulty_widget(logic::Point{x: 230, y: 90},theme, 2);
                        hiragana::a(logic::Point{x: 240, y: 155}, 0.5, 4, theme.overlay, 0, draw::Fastforwarding{enabled: false, scale: 1.0});
                        hiragana::a(logic::Point{x: 240, y: 150}, 0.5, 4, theme.secondary_accent, 0, draw::Fastforwarding{enabled: false, scale: 1.0});
                        katakana::a(logic::Point{x: 40, y: 40}, 1.2, 6, theme.secondary_accent, 10000, draw::Fastforwarding{enabled: true, scale: 3.0});
                        widget::info_widget(logic::Point{x: 90, y: 15}, theme, draw::Text{text: "< / >\0", xoffset: 8, big: true, text_color: theme.text, bg_color: theme.tertiary_accent});
                    }
                    1 => {
                        widget::character_widget(logic::Point{x: 230, y: 50}, theme, draw::Text{text: "I\0", xoffset: 30, big: true, text_color: theme.text, bg_color: theme.accent});
                        widget::difficulty_widget(logic::Point{x: 230, y: 90},theme, 1);
                        hiragana::i(logic::Point{x: 240, y: 155}, 0.5, 4, theme.overlay, 0, draw::Fastforwarding{enabled: false, scale: 1.0});
                        hiragana::i(logic::Point{x: 240, y: 150}, 0.5, 4, theme.secondary_accent, 0, draw::Fastforwarding{enabled: false, scale: 1.0});
                        katakana::i(logic::Point{x: 40, y: 40}, 1.2, 6, theme.secondary_accent, 10000, draw::Fastforwarding{enabled: true, scale: 3.0});
                        widget::info_widget(logic::Point{x: 90, y: 15}, theme, draw::Text{text: "< / >\0", xoffset: 8, big: true, text_color: theme.text, bg_color: theme.tertiary_accent});
                    }
                    2 => {
                        widget::character_widget(logic::Point{x: 230, y: 50}, theme, draw::Text{text: "U\0", xoffset: 30, big: true, text_color: theme.text, bg_color: theme.accent});
                        widget::difficulty_widget(logic::Point{x: 230, y: 90},theme, 2);
                        hiragana::u(logic::Point{x: 240, y: 155}, 0.5, 4, theme.overlay, 0, draw::Fastforwarding{enabled: false, scale: 1.0});
                        hiragana::u(logic::Point{x: 240, y: 150}, 0.5, 4, theme.secondary_accent, 0, draw::Fastforwarding{enabled: false, scale: 1.0});
                        katakana::u(logic::Point{x: 40, y: 40}, 1.2, 6, theme.secondary_accent, 10000, draw::Fastforwarding{enabled: true, scale: 3.0});
                        widget::info_widget(logic::Point{x: 90, y: 15}, theme, draw::Text{text: "< / >\0", xoffset: 8, big: true, text_color: theme.text, bg_color: theme.tertiary_accent});
                    }
                    3 => {
                        widget::character_widget(logic::Point{x: 230, y: 50}, theme, draw::Text{text: "E\0", xoffset: 30, big: true, text_color: theme.text, bg_color: theme.accent});
                        widget::difficulty_widget(logic::Point{x: 230, y: 90},theme, 1);
                        hiragana::e(logic::Point{x: 240, y: 155}, 0.5, 4, theme.overlay, 0, draw::Fastforwarding{enabled: false, scale: 1.0});
                        hiragana::e(logic::Point{x: 240, y: 150}, 0.5, 4, theme.secondary_accent, 0, draw::Fastforwarding{enabled: false, scale: 1.0});
                        katakana::e(logic::Point{x: 40, y: 40}, 1.2, 6, theme.secondary_accent, 10000, draw::Fastforwarding{enabled: true, scale: 3.0});
                        widget::info_widget(logic::Point{x: 90, y: 15}, theme, draw::Text{text: "< / >\0", xoffset: 8, big: true, text_color: theme.text, bg_color: theme.tertiary_accent});
                    }
                    _ => {}
                }
            } else {
                match selected_character {
                    0 => {
                        widget::character_widget(logic::Point{x: 230, y: 50}, theme, draw::Text{text: "A\0", xoffset: 30, big: true, text_color: theme.text, bg_color: theme.accent});
                        widget::difficulty_widget(logic::Point{x: 230, y: 90},theme, 4);
                        katakana::a(logic::Point{x: 240, y: 155}, 0.5, 4, theme.overlay, 0, draw::Fastforwarding{enabled: false, scale: 1.0});
                        katakana::a(logic::Point{x: 240, y: 150}, 0.5, 4, theme.secondary_accent, 0, draw::Fastforwarding{enabled: false, scale: 1.0});
                        hiragana::a(logic::Point{x: 40, y: 40}, 1.2, 6, theme.secondary_accent, 10000, draw::Fastforwarding{enabled: true, scale: 4.0});
                        widget::info_widget(logic::Point{x: 90, y: 15}, theme, draw::Text{text: "< / >\0", xoffset: 8, big: true, text_color: theme.text, bg_color: theme.tertiary_accent});
                    }
                    1 => {
                        widget::character_widget(logic::Point{x: 230, y: 50}, theme, draw::Text{text: "I\0", xoffset: 30, big: true, text_color: theme.text, bg_color: theme.accent});
                        widget::difficulty_widget(logic::Point{x: 230, y: 90},theme, 2);
                        katakana::i(logic::Point{x: 240, y: 155}, 0.5, 4, theme.overlay, 0, draw::Fastforwarding{enabled: false, scale: 1.0});
                        katakana::i(logic::Point{x: 240, y: 150}, 0.5, 4, theme.secondary_accent, 0, draw::Fastforwarding{enabled: false, scale: 1.0});
                        hiragana::i(logic::Point{x: 40, y: 40}, 1.2, 6, theme.secondary_accent, 10000, draw::Fastforwarding{enabled: true, scale: 4.0});
                        widget::info_widget(logic::Point{x: 90, y: 15}, theme, draw::Text{text: "< / >\0", xoffset: 8, big: true, text_color: theme.text, bg_color: theme.tertiary_accent});
                    }
                    2 => {
                        widget::character_widget(logic::Point{x: 230, y: 50}, theme, draw::Text{text: "U\0", xoffset: 30, big: true, text_color: theme.text, bg_color: theme.accent});
                        widget::difficulty_widget(logic::Point{x: 230, y: 90},theme, 3);
                        katakana::u(logic::Point{x: 240, y: 155}, 0.5, 4, theme.overlay, 0, draw::Fastforwarding{enabled: false, scale: 1.0});
                        katakana::u(logic::Point{x: 240, y: 150}, 0.5, 4, theme.secondary_accent, 0, draw::Fastforwarding{enabled: false, scale: 1.0});
                        hiragana::u(logic::Point{x: 40, y: 40}, 1.2, 6, theme.secondary_accent, 10000, draw::Fastforwarding{enabled: true, scale: 4.0});
                        widget::info_widget(logic::Point{x: 90, y: 15}, theme, draw::Text{text: "< / >\0", xoffset: 8, big: true, text_color: theme.text, bg_color: theme.tertiary_accent});
                    }
                    3 => {
                        widget::character_widget(logic::Point{x: 230, y: 50}, theme, draw::Text{text: "E\0", xoffset: 30, big: true, text_color: theme.text, bg_color: theme.accent});
                        widget::difficulty_widget(logic::Point{x: 230, y: 90},theme, 4);
                        katakana::e(logic::Point{x: 240, y: 155}, 0.5, 4, theme.overlay, 0, draw::Fastforwarding{enabled: false, scale: 1.0});
                        katakana::e(logic::Point{x: 240, y: 150}, 0.5, 4, theme.secondary_accent, 0, draw::Fastforwarding{enabled: false, scale: 1.0});
                        hiragana::e(logic::Point{x: 40, y: 40}, 1.2, 6, theme.secondary_accent, 10000, draw::Fastforwarding{enabled: true, scale: 4.0});
                        widget::info_widget(logic::Point{x: 90, y: 15}, theme, draw::Text{text: "< / >\0", xoffset: 8, big: true, text_color: theme.text, bg_color: theme.tertiary_accent});
                    }
                    _ => {}
                }
            }
            shift_pressed = false;
            needs_redraw = false;
        }
        if logic::keypress(logic::keyboard::Key::Left) && shift_pressed {
            if theme.id == 0 {
                theme = theme::theme(Some(4));
            } else {
                theme = theme::theme(Some(theme.id.saturating_sub(1)));
            }
            widget::theme_widget(theme);
            logic::timing::msleep(100);
        } else if logic::keypress(logic::keyboard::Key::Right) && shift_pressed {
            if theme.id == 4 {
                theme = theme::theme(Some(0));
            } else {
                theme = theme::theme(Some(theme.id.saturating_add(1)));
            }
            widget::theme_widget(theme);
            logic::timing::msleep(100);
        } else if logic::keypress(logic::keyboard::Key::Left) && !shift_pressed {
            if selected_character > 0 {
                selected_character -= 1;
                needs_redraw = true;
                logic::timing::msleep(500);
            }
        } else if logic::keypress(logic::keyboard::Key::Right) && !shift_pressed {
            if selected_character < 3 {
                selected_character += 1;
                needs_redraw = true;
                logic::timing::msleep(500);
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
            widget::theme_widget(theme);
            logic::timing::msleep(200);
        } else if logic::keypress(logic::keyboard::Key::Home) {
            break 'page;
        };
        logic::timing::msleep(10);
    }
}