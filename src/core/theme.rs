#[derive(Copy, Clone)]
pub struct Theme<'a> {
    pub name : &'a str,
    pub id : u8,
    pub bg : u16,
    pub text : u16,
    pub subtext : u16,
    pub overlay : u16,
    pub accent : u16,
    pub secondary_accent : u16,
    pub tertiary_accent : u16,
}

pub fn themecount() -> u8 {
    5
}

pub fn theme<'a>(theme: Option<u8>) -> Theme<'a> {
    match theme {
        Some(0) => Theme  {
            name : "Numworks Light\0",
            id : 0,
            bg : 0xffff,
            text : 0x0000,
            subtext: 0x1082,
            overlay : 0xe71c,
            accent : 0xfde9,
            secondary_accent : 0xb46d,
            tertiary_accent : 0xfed3,
        },
        Some(1) => Theme {
            name : "Numworks Dark\0",
            id : 1,
            bg : 0x18c3,
            text : 0xffff,
            subtext: 0xef5d,
            overlay : 0x4a69,
            accent : 0xfde9,
            secondary_accent : 0xfed3,
            tertiary_accent : 0xb46d,
        },
        Some(2) => Theme {
            name : "Omega Light\0",
            id : 2,
            bg : 0xffff,
            text : 0x0000,
            subtext: 0x4a69,
            overlay : 0xe71c,
            accent : 0xe1c7,
            secondary_accent : 0x9229,
            tertiary_accent : 0xfd75,
        },
        Some(3) => Theme {
            name : "Omega Dark\0",
            id : 3,
            bg : 0x18c3,
            text : 0xffff,
            subtext: 0xe71c,
            overlay : 0x4a69,
            accent : 0xe1c7,
            secondary_accent : 0xfd75,
            tertiary_accent : 0x9229,
        },
        Some(4) => Theme {
            name : "Candy\0",
            id : 4,
            bg : 0xe7ff,
            text : 0x1083,
            subtext : 0x2126,
            overlay : 0x8514,
            accent : 0x5e97,
            secondary_accent : 0x4293,
            tertiary_accent : 0xfd57,
        },
        // Other themes
        _ => Theme {
            name : "Numworks Light\0",
            id : 0,
            bg : 0xffff,
            text : 0x0000,
            subtext: 0x4a69,
            overlay : 0xe71c,
            accent : 0xfde9,
            secondary_accent : 0x93ca,
            tertiary_accent : 0xfed3,
        },
    }
}