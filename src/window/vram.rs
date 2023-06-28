#[derive(Copy, Clone)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}
impl Color {
    pub fn new(r: u8, g: u8, b: u8, a: u8) -> Color {
        Color { r, g, b, a }
    }
    // Function calculates every color value as 0.0 to 1.0 (GTK issue)
    pub fn calc_gtk_color(&self) -> (f64, f64, f64, f64) {
        return (
            self.r as f64 / 255.0,
            self.g as f64 / 255.0,
            self.b as f64 / 255.0,
            self.a as f64 / 255.0,
        )
    }
}

pub struct VRAM {
    pub size_x: u32,
    pub size_y: u32,
    pub data: Vec<Color>,
}
impl VRAM {
    pub fn new(size_x: u32, size_y: u32) -> VRAM {
        VRAM {
            size_x,
            size_y,
            data: vec![Color::new(0,0,0,255); (size_x * size_y) as usize],
        }
    }
    pub fn calc_index(&self, x: u32, y: u32) -> usize {
        (y * self.size_x + x) as usize
    }

    pub fn set_pixel(&mut self, x: u32, y: u32, color: Color) {
        let index = self.calc_index(x, y);
        self.data[index] = color;
    }
    pub fn get_pixel(&self, x: u32, y: u32) -> Color {
        let index = self.calc_index(x, y);
        self.data[index]
    }
    pub fn clear(&mut self) {
        for i in 0..self.data.len() {
            self.data[i] = Color::new(0,0,0,255);
        }
    }
}

