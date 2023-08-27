mod adb;
use crate::error::{AGError, AGResult};
pub use adb::{AdbBuilder, ADB};
use image::RgbaImage;

pub trait Controller {
    fn screenshot(&mut self) -> AGResult<RgbaImage>;
    fn click(&mut self, x: u32, y: u32) -> AGResult<()>;
    fn swipe<'a>(&mut self, x1: u32, y1: u32, x2: u32, y2: u32) -> AGResult<()>;
    fn press_key(&mut self, keycode: u32) -> AGResult<()>;
    fn get_resolution(&mut self) -> AGResult<(u32, u32)>;
    fn input_text(&mut self, text: &str) -> AGResult<()>;
}
