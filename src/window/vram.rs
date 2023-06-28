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
        if x >= self.size_x || y >= self.size_y {return;}
        let index = self.calc_index(x, y);
        self.data[index] = color;
    }
    pub fn get_pixel(&self, x: u32, y: u32) -> Color {
        if x >= self.size_x || y >= self.size_y {return Color::new(0,0,0,0);}
        let index = self.calc_index(x, y);
        self.data[index]
    }
    pub fn clear(&mut self) {
        for i in 0..self.data.len() {
            self.data[i] = Color::new(0,0,0,255);
        }
    }

    pub fn rect(&mut self, x: u32, y: u32, w: u32, h: u32, color: Color) {
        for i in x..x+w {
            for j in y..y+h {
                self.set_pixel(i, j, color);
            }
        }
    }
    pub fn circle(&mut self, x: u32, y: u32, r: u32, color: Color) {
        for i in x-r..x+r {
            for j in y-r..y+r {
                if (i-x)*(i-x) + (j-y)*(j-y) <= r*r {
                    self.set_pixel(i, j, color);
                }
            }
        }
    }
    pub fn line(&mut self, x1: u32, y1: u32, x2: u32, y2: u32, color: Color) {
        let mut x = x1 as i32;
        let mut y = y1 as i32;
        let dx = (x2 as i32 - x1 as i32).abs();
        let dy = (y2 as i32 - y1 as i32).abs();
        let sx = if x1 < x2 {1} else {-1};
        let sy = if y1 < y2 {1} else {-1};
        let mut err = dx - dy;
        loop {
            self.set_pixel(x as u32, y as u32, color);
            if x == x2 as i32 && y == y2 as i32 {break;}
            let e2 = 2 * err;
            if e2 > -dy {
                err -= dy;
                x += sx;
            }
            if e2 < dx {
                err += dx;
                y += sy;
            }
        }
    }
}

