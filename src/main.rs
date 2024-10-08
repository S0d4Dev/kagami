#![no_std]
#![no_main]

#[used]
#[link_section = ".rodata.eadk_app_name"]
pub static EADK_APP_NAME: [u8; 7] = *b"Kagami\0";

#[used]
#[link_section = ".rodata.eadk_api_level"]
pub static EADK_APP_API_LEVEL: u32 = 0;

#[used]
#[link_section = ".rodata.eadk_app_icon"]
pub static EADK_APP_ICON: [u8; 4292] = *include_bytes!("../target/icon.nwi");

pub mod characters {
    pub mod hiragana;
    pub mod katakana;
    pub mod other;
}
pub mod widget;
pub mod draw;
pub mod theme;
pub mod logic;
pub mod menu;

#[no_mangle]
pub fn main() {
    menu::page();
}