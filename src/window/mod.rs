pub mod vram;
pub mod shader;
pub mod sprite;

use gtk::prelude::*;
use gtk::{DrawingArea, Window, WindowType};
use std::cell::RefCell;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use std::time::{Instant};

thread_local!(static GLOBAL: RefCell<Option<Window>> = RefCell::new(None););
fn check_update_display() {GLOBAL.with(|global|{if let Some(win) = &*global.borrow() {win.queue_draw();}})}

#[derive(Clone)]
pub struct Display {
    pub title: String,
    pub VRAM: Arc<Mutex<vram::VRAM>>,
    pub size_x: u32,
    pub size_y: u32,
    pub scale: u32,
}
impl Display {
    pub fn new(title: String, size_x: u32, size_y: u32, scale: u32) -> Display {
        Display {
            title,
            VRAM: Arc::new(Mutex::new(vram::VRAM::new(size_x, size_y))),
            size_x,
            size_y,
            scale,
        }
    }

    pub fn run(&self) {
        gtk::init().expect("Failed to initialize GTK.");
        let window = Window::new(WindowType::Toplevel);
        window.set_title(&self.title);
        window.set_default_size(
            self.size_x as i32 * self.scale as i32,
            self.size_y as i32 * self.scale as i32,
        );
        window.connect_delete_event(|_, _| {gtk::main_quit(); Inhibit(false)});
        window.set_resizable(false);
        let drawing_area = DrawingArea::new();
        window.add(&drawing_area);
        window.show_all();
        let vram_mut = Arc::clone(&self.VRAM);

        let scale = self.scale as usize;
        let size_x = self.size_x as usize;
        drawing_area.connect_draw(move |_, cr| {
            let mut vram = vram_mut.lock().unwrap();
            if vram.redraw {
                let mut _start = Instant::now();
                for i in 0..vram.data.len() {
                    let (r, g, b, a) = vram.data[i].calc_gtk_color(); cr.set_source_rgba(r, g, b, a);
                    cr.rectangle(
                        (i % size_x * scale) as f64,
                        (i / size_x * scale) as f64,
                        scale as f64, scale as f64,
                    );
                    let err = cr.fill(); if err != Ok(()) {println!("Failed to draw pixel, GTK error;");}
                }
                vram.redraw = false;
                println!("WIN: {:>3}", (Instant::now() - _start).as_micros());
            }
            Inhibit(false)
        });

        window.connect_event(|w, _| {w.queue_draw(); Inhibit(false)});
        window.present();

        GLOBAL.with(|global|{*global.borrow_mut() = Some(window);});
        glib::timeout_add(Duration::from_millis(100), move || {check_update_display(); glib::Continue(true)});
        gtk::main();
    }
}

