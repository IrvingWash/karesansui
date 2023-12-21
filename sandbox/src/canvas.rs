use karesansui::vector3::Vector3;
use sdl2::{
    gfx::primitives::DrawRenderer,
    pixels::Color,
    rect::Rect,
    render::{Texture, WindowCanvas},
    EventPump, TimerSubsystem,
};

pub struct Canvas {
    pub timer: TimerSubsystem,
    pub event_pump: EventPump,
    window_width: i32,
    window_height: i32,
    renderer: WindowCanvas,
}

impl Canvas {
    pub fn open_window() -> Self {
        let ctx = sdl2::init().unwrap();

        let video_subsystem = ctx.video().unwrap();

        let display_mode = video_subsystem.current_display_mode(0).unwrap();

        let window_width = display_mode.w;
        let window_height = display_mode.h;

        let window = video_subsystem
            .window(
                "Sandbox",
                window_width.try_into().unwrap(),
                window_height.try_into().unwrap(),
            )
            .metal_view()
            .fullscreen_desktop()
            .build()
            .unwrap();

        Canvas {
            renderer: window.into_canvas().build().unwrap(),
            timer: ctx.timer().unwrap(),
            event_pump: ctx.event_pump().unwrap(),
            window_height,
            window_width,
        }
    }

    pub fn width(&self) -> i32 {
        self.window_width
    }

    pub fn height(&self) -> i32 {
        self.window_height
    }

    pub fn clear_screen(&mut self, color: Color) {
        self.renderer.set_draw_color(color);
    }

    pub fn render_frame(&mut self) {
        self.renderer.present()
    }

    pub fn draw_line(&self, x1: i16, y1: i16, x2: i16, y2: i16, color: Color) {
        self.renderer.line(x1, y1, x2, y2, color).unwrap();
    }

    pub fn draw_circle(&self, x: i16, y: i16, radius: i16, angle: f64, color: Color) {
        self.renderer.circle(x, y, radius, color).unwrap();

        self.draw_line(
            x,
            y,
            x + ((angle.cos() * f64::from(radius)) as i16),
            y + ((angle.sin() * f64::from(radius)) as i16),
            color,
        );
    }

    pub fn draw_filled_circle(&self, x: i16, y: i16, radius: i16, color: Color) {
        self.renderer.filled_circle(x, y, radius, color).unwrap();
    }

    pub fn draw_rectangle(&self, x: i16, y: i16, width: i16, height: i16, color: Color) {
        self.draw_line(
            x - width / 2,
            y - height / 2,
            x + width / 2,
            y - height / 2,
            color,
        );
        self.draw_line(
            x + width / 2,
            y - height / 2,
            x + width / 2,
            y + height / 2,
            color,
        );
        self.draw_line(
            x + width / 2,
            y + height / 2,
            x - width / 2,
            y + height / 2,
            color,
        );
        self.draw_line(
            x - width / 2,
            y + height / 2,
            x - width / 2,
            y - height / 2,
            color,
        );
    }

    pub fn draw_filled_rectangle(&self, x: i16, y: i16, width: i16, height: i16, color: Color) {
        self.renderer
            .box_(
                x - width / 2,
                y - height / 2,
                x + width / 2,
                y + height / 2,
                color,
            )
            .unwrap();
    }

    pub fn draw_polygon(&self, x: i16, y: i16, vertices: Vec<Vector3>, color: Color) {
        for (i, vertex) in vertices.iter().enumerate() {
            let next_index = (i + 1) % vertices.len();

            self.draw_line(
                vertex.x as i16,
                vertex.y as i16,
                vertices[next_index].x as i16,
                vertices[next_index].y as i16,
                color,
            );
        }

        self.draw_filled_circle(x, y, 1, color);
    }

    pub fn draw_filled_polygon(&self, x: i16, y: i16, vertices: Vec<Vector3>, color: Color) {
        let mut vx: [i16; 2] = [0, 0];
        let mut vy: [i16; 2] = [0, 0];

        for (i, vertex) in vertices.iter().enumerate() {
            vx[i] = vertex.x as i16;
            vy[i] = vertex.y as i16;
        }

        self.renderer.filled_polygon(&vx, &vy, color).unwrap();
    }

    pub fn draw_texture(
        &mut self,
        x: i16,
        y: i16,
        width: i16,
        height: i16,
        angle: f64,
        texture: &Texture,
    ) {
        let destination_rectangle = Rect::new(
            (x - (width / 2)) as i32,
            (y - (height / 2)) as i32,
            width as u32,
            height as u32,
        );

        let degrees = angle * 57.2958;

        self.renderer
            .copy_ex(
                texture,
                None,
                destination_rectangle,
                degrees,
                None,
                false,
                false,
            )
            .unwrap();
    }
}
