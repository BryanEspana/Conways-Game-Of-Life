mod colors;
mod framebuffer;

use framebuffer::FrameBuffer;
use colors::Color;
use minifb::{Window, WindowOptions, Key};
use std::time::{Duration, Instant};

fn main() {
    let cell_size = 60; // Tamaño de cada célula en píxeles
    let grid_width = 10; // Número de células a lo ancho
    let grid_height = 10; // Número de células a lo alto
    let window_width = grid_width * cell_size;
    let window_height = grid_height * cell_size;

    let mut fb = FrameBuffer::new(window_width, window_height, cell_size);

    let alive_color = Color::white();
    let dead_color = Color::black();
    fb.clear(dead_color);

    // Still Lifes
    let block = [(1, 1), (1, 2), (2, 1), (2, 2)];
    let bee_hive = [(4, 1), (5, 2), (5, 3), (4, 4), (3, 3), (3, 2)];
    let loaf = [(7, 1), (8, 2), (8, 3), (7, 4), (6, 3), (6, 2), (7, 2)];
    let boat = [(1, 5), (2, 5), (2, 6), (1, 6), (3, 6)];
    let tub = [(4, 5), (5, 6), (4, 7), (3, 6)];

    // Oscillators
    let blinker = [(7, 5), (8, 5), (9, 5)];
    let toad = [(1, 8), (2, 8), (2, 9), (3, 9), (3, 10), (4, 10)];
    let beacon = [(5, 8), (6, 8), (5, 9), (6, 9), (7, 10), (8, 10), (7, 11), (8, 11)];
    let pulsar = [
        (1, 13), (2, 13), (3, 13), (5, 13), (6, 13), (7, 13), 
        (1, 15), (2, 15), (3, 15), (5, 15), (6, 15), (7, 15), 
        (4, 12), (4, 17), 
        (9, 12), (9, 17), 
        (10, 13), (11, 13), (12, 13), (10, 15), (11, 15), (12, 15),
    ];
    let pentadecathlon = [
        (6, 8), (6, 9), (6, 10), (7, 9), (5, 9), (6, 11),
        (6, 12), (6, 13), (7, 12), (5, 12), (6, 14), (6, 15)
    ];

    // Spaceships
    let glider = [(8, 8), (9, 9), (7, 10), (8, 10), (9, 10)];
    let lwss = [
        (1, 10), (2, 10), (3, 10), (4, 10), (0, 11), (4, 11),
        (4, 12), (0, 13), (3, 13)
    ];
    let mwss = [
        (5, 10), (6, 10), (7, 10), (8, 10), (9, 10), (5, 11), 
        (9, 11), (9, 12), (5, 13), (8, 13)
    ];
    let hwss = [
        (1, 13), (2, 13), (3, 13), (4, 13), (5, 13), (1, 14), 
        (5, 14), (5, 15), (1, 16), (4, 16)
    ];

    fb.set_pattern(&block, alive_color);
    fb.set_pattern(&bee_hive, alive_color);
    fb.set_pattern(&loaf, alive_color);
    fb.set_pattern(&boat, alive_color);
    fb.set_pattern(&tub, alive_color);
    fb.set_pattern(&blinker, alive_color);
    fb.set_pattern(&toad, alive_color);
    fb.set_pattern(&beacon, alive_color);
    fb.set_pattern(&pulsar, alive_color);
    fb.set_pattern(&pentadecathlon, alive_color);
    fb.set_pattern(&glider, alive_color);
    fb.set_pattern(&lwss, alive_color);
    fb.set_pattern(&mwss, alive_color);
    fb.set_pattern(&hwss, alive_color);

    let mut window = Window::new(
        "Conway's Game of Life",
        window_width,
        window_height,
        WindowOptions::default(),
    ).unwrap_or_else(|e| {
        panic!("{}", e);
    });

    let mut last_update = Instant::now();
    while window.is_open() && !window.is_key_down(Key::Escape) {
        if last_update.elapsed() >= Duration::from_millis(100) {
            fb.update(alive_color, dead_color);
            last_update = Instant::now();
        }
        window.update_with_buffer(&fb.buffer, window_width, window_height).unwrap();
    }
}
