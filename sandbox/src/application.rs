use sdl2::{event::Event, keyboard::Keycode, pixels::Color};

use crate::canvas::Canvas;

const FPS: u64 = 60;
const MILLISECONDS_PER_FRAME: u64 = 1000 / FPS;

pub struct Application {
    is_running: bool,
    canvas: Canvas,
    time_previous_frame: u64,
}

impl Application {
    pub fn new() -> Self {
        let canvas = Canvas::open_window();

        Application {
            is_running: false,
            canvas,
            time_previous_frame: 0,
        }
    }

    pub fn init(&mut self) {
        self.is_running = true;
    }

    pub fn is_running(&self) -> bool {
        self.is_running
    }

    pub fn input(&mut self) {
        for event in self.canvas.event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    self.is_running = false;
                }
                _ => {}
            };
        }
    }

    pub fn update(&mut self) {
        let delta_time = self.sleep_and_calculate_delta_time();

        println!("{}", delta_time);
    }

    pub fn render(&mut self) {
        self.canvas.clear_screen(Color::RGB(1, 0, 0));

        self.canvas.draw_filled_circle(100, 100, 100, Color::RGB(0, 0, 180));

        self.canvas.render_frame();
    }

    fn sleep_and_calculate_delta_time(&mut self) -> f64 {
        let time_to_wait =
            MILLISECONDS_PER_FRAME.saturating_sub(self.canvas.timer.ticks64().saturating_sub(self.time_previous_frame));

        if time_to_wait > 0 {
            self.canvas.timer.delay(time_to_wait as u32);
        }

        let delta_time = (self.canvas.timer.ticks64().saturating_sub(self.time_previous_frame)) as f64 / 1000_f64;

        self.time_previous_frame = self.canvas.timer.ticks64();

        delta_time
    }
}
