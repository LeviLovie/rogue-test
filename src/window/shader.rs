use std::sync::{MutexGuard};
use crate::window;

fn process_pixel(_x: u32, _y: u32, _size_x: u32, _size_y: u32, _color: window::vram::Color) -> window::vram::Color {
    // if _x % 10 == 0 {
    //     return window::vram::Color::new(10, 10, 10, 255);
    // }
    return _color;
}

pub fn process_shader_to_vram(vram: &MutexGuard<'_, window::vram::VRAM>, size_x: u32, size_y: u32) -> window::vram::VRAM {
    let mut result = window::vram::VRAM::new(size_x, size_y);
    for i in 0..size_y {
        for j in 0..size_x {
            let color = vram.get_pixel(i, j);
            result.set_pixel(i, j, process_pixel(j, i, size_x, size_y, color));
        }
    }
    return result;
}
