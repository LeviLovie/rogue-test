use std::{sync::{Arc}, thread};
use std::time::{Duration, Instant};

use crate::window;

pub fn run(size_x: i32, size_y: i32, scale: i32, tps: i32) {
    let _main_window = window::Display::new("Rogue".to_string(), size_x as u32, size_y as u32, scale as u32);
    let vram_mut = Arc::clone(&_main_window.VRAM);
    thread::spawn(move || {
        let     _tick_duration = Duration::from_secs(1) / tps as u32;
        let mut _last_tick     = Instant::now();
        let mut _start         = Instant::now();
        let mut _end           = Instant::now();
        let mut _elapsed       = _end - _start;
        let mut _iteration     = 0;
        loop {
            _start = Instant::now();
            let mut vram = vram_mut.lock().unwrap();
           
            vram.rect(0, 0, size_x as u32, size_y as u32, window::vram::Color::new(30, 30, 30, 255));
            vram.rect(100+_iteration, 100, 100, 100, window::vram::Color::new(225, 225, 225, 255));
            
            drop(vram); _iteration += 1;
            _end = Instant::now(); _elapsed = _end - _start;
            if _elapsed < _tick_duration {std::thread::sleep(_tick_duration - _elapsed);} _last_tick = Instant::now();
            println!("ENG: {:>3}:{:<3}ms", _elapsed.as_millis(), _tick_duration.as_millis());
        };
    });
    _main_window.run();
}

