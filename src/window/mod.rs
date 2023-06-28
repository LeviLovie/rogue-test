pub mod vram;

use gtk::prelude::*;
use gtk::{DrawingArea, Window, WindowType};
use std::cell::RefCell;
use std::sync::{Arc, Mutex};
use std::time::Duration;

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

        let scale = self.scale;
        drawing_area.connect_draw(move |_, cr| {
            let vram = vram_mut.lock().unwrap();
            let mut err;
            for i in 0..vram.data.len() {
                let x = i % vram.size_x as usize;
                let y = i / vram.size_x as usize;
                let color = vram.data[i];
                let (r, g, b, a) = color.calc_gtk_color();
                cr.set_source_rgba(r, g, b, a);
                cr.rectangle(
                    (x * scale as usize) as f64,
                    (y * scale as usize) as f64,
                    scale as f64,
                    scale as f64,
                );
                err = cr.fill();
                if err != Ok(()) {
                    println!("Failed to draw pixel, GTK error;");
                }
            }
            drop(vram);
            Inhibit(false)
        });

        window.connect_event(|w, _| {w.queue_draw(); Inhibit(false)});
        window.present();

        GLOBAL.with(|global|{*global.borrow_mut() = Some(window);});
        glib::timeout_add(Duration::from_millis(100), move || {check_update_display(); glib::Continue(true)});
        gtk::main();
    }
}

