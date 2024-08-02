use crate::colors::Color;

pub struct FrameBuffer {
    pub width: usize,
    pub height: usize,
    pub buffer: Vec<u32>,
    pub previous_buffer: Vec<u32>,
    pixel_size: usize,
}

impl FrameBuffer {
    pub fn new(width: usize, height: usize, pixel_size: usize) -> Self {
        let black = Color::black().to_u32();
        Self {
            width,
            height,
            buffer: vec![black; width * height],
            previous_buffer: vec![black; width * height],
            pixel_size,
        }
    }

    pub fn draw_pixel(&mut self, x: isize, y: isize, color: Color) {
        if x >= 0 && y >= 0 && x < (self.width as isize) / (self.pixel_size as isize) && y < (self.height as isize) / (self.pixel_size as isize) {
            self.draw_large_pixel(x, y, color);
        }
    }

    fn draw_large_pixel(&mut self, x: isize, y: isize, color: Color) {
        let base_x = x * self.pixel_size as isize;
        let base_y = y * self.pixel_size as isize;
        let color_value = color.to_u32();

        for offset_y in 0..self.pixel_size {
            for offset_x in 0..self.pixel_size {
                let draw_x = base_x + offset_x as isize;
                let draw_y = base_y + offset_y as isize;
                let index = (draw_x + draw_y * self.width as isize) as usize;
                if index < self.buffer.len() && self.buffer[index] != color_value {
                    self.buffer[index] = color_value;
                }
            }
        }
    }

    pub fn clear(&mut self, color: Color) {
        let color_value = color.to_u32();
        for pixel in self.buffer.iter_mut() {
            *pixel = color_value;
        }
    }

    pub fn set_pattern(&mut self, pattern: &[(usize, usize)], color: Color) {
        for &(x, y) in pattern.iter() {
            self.draw_pixel(x as isize, y as isize, color);
        }
    }

    pub fn update(&mut self, alive_color: Color, dead_color: Color) {
        let mut new_buffer = self.buffer.clone();
        let width = self.width;
        let height = self.height;

        for y in 0..height {
            for x in 0..width {
                let idx = y * width + x;
                let current = self.buffer[idx];
                let neighbors = self.count_alive_neighbors(x, y, alive_color.to_u32());
                if current == alive_color.to_u32() {
                    if neighbors < 2 || neighbors > 3 {
                        new_buffer[idx] = dead_color.to_u32();
                    }
                } else {
                    if neighbors == 3 {
                        new_buffer[idx] = alive_color.to_u32();
                    }
                }
            }
        }

        if new_buffer != self.previous_buffer {
            self.buffer = new_buffer.clone();
            self.previous_buffer = new_buffer;
        }
    }

    fn count_alive_neighbors(&self, x: usize, y: usize, alive_color: u32) -> u32 {
        let mut count = 0;
        let width = self.width as isize;
        let height = self.height as isize;

        for dy in -1..=1 {
            for dx in -1..=1 {
                if dx == 0 && dy == 0 {
                    continue;
                }
                let nx = x as isize + dx;
                let ny = y as isize + dy;
                if nx >= 0 && ny >= 0 && nx < width && ny < height {
                    let idx = ny as usize * self.width + nx as usize;
                    if self.buffer[idx] == alive_color {
                        count += 1;
                    }
                }
            }
        }
        count
    }
}
