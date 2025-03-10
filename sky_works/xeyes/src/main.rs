
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderArgs, RenderEvent, UpdateArgs, UpdateEvent, MouseCursorEvent};
use piston::window::WindowSettings;
use glutin_window::GlutinWindow;
use opengl_graphics::{GlGraphics, OpenGL};
use graphics::*;

struct App {
    gl: GlGraphics, // OpenGL drawing backend
    eyes: Vec<(f64, f64)>, // Positions of the eyes (x, y)
    mouse_pos: [f64; 2], // Current mouse position (x, y)
}

impl App {
    fn new(opengl: OpenGL) -> Self {
        App {
            gl: GlGraphics::new(opengl),
            eyes: vec![(100.0, 100.0), (200.0, 100.0)], // Initial positions of the eyes
            mouse_pos: [0.0, 0.0], // Initialize mouse position
        }
    }

    fn render(&mut self, args: &RenderArgs) {
        const EYE_RADIUS: f64 = 30.0;
        const PUPIL_RADIUS: f64 = 10.0;

        self.gl.draw(args.viewport(), |c, gl| {
            // Clear the screen
            clear([1.0, 1.0, 1.0, 1.0], gl);

            for &(eye_x, eye_y) in &self.eyes {
                // Draw the eye (white circle)
                ellipse(
                    [1.0, 1.0, 1.0, 1.0], // White color
                    [eye_x - EYE_RADIUS, eye_y - EYE_RADIUS, EYE_RADIUS * 2.0, EYE_RADIUS * 2.0],
                    c.transform,
                    gl,
                );

                // Calculate pupil position based on mouse position
                let dx = self.mouse_pos[0] - eye_x;
                let dy = self.mouse_pos[1] - eye_y;
                let distance = (dx * dx + dy * dy).sqrt();
                let angle = dy.atan2(dx);

                let pupil_x = eye_x + dx.min(EYE_RADIUS - PUPIL_RADIUS) * angle.cos();
                let pupil_y = eye_y + dy.min(EYE_RADIUS - PUPIL_RADIUS) * angle.sin();

                // Draw the pupil (black circle)
                ellipse(
                    [0.0, 0.0, 0.0, 1.0], // Black color
                    [pupil_x - PUPIL_RADIUS, pupil_y - PUPIL_RADIUS, PUPIL_RADIUS * 2.0, PUPIL_RADIUS * 2.0],
                    c.transform,
                    gl,
                );
            }
        });
    }

    fn update(&mut self, _args: &UpdateArgs) {
        // No updates needed for this example
    }

    fn mouse_cursor(&mut self, pos: [f64; 2]) {
        self.mouse_pos = pos; // Update mouse position
    }
}

fn main() {
    let opengl = OpenGL::V3_2;
    let mut window: GlutinWindow = WindowSettings::new("xeyes in Rust", [400, 300])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut app = App::new(opengl);
    let mut events = Events::new(EventSettings::new());

    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.render_args() {
            app.render(&args);
        }

        if let Some(args) = e.update_args() {
            app.update(&args);
        }

        if let Some(pos) = e.mouse_cursor_args() {
            app.mouse_cursor(pos);
        }
    }
}