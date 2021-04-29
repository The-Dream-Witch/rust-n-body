extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderArgs, RenderEvent, UpdateArgs, UpdateEvent};
use piston::window::WindowSettings;

pub struct App {
    gl: GlGraphics, // OpenGL drawing backend.
    rotation: f64,  // Rotation for the square.
}

impl App {
    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 0.0];
        const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];
        const RED:   [f32; 4] = [1.0, 0.0,0.0,1.0];

        let square1 = rectangle::square(0.0, 0.0, 50.0);
        let square2 = rectangle::square(0.0,0.0,100.0);
        
        let rotation = self.rotation;
        let (x, y) = (args.window_size[0] / 2.0, args.window_size[1] / 2.0);

        self.gl.draw(args.viewport(), |c, gl| {
            // Clear the screen.
            clear(BLACK, gl);

            let transform = c
                .transform
                .trans(x, y)
                .rot_rad(rotation)
                .trans(-25.0, -25.0);
            
                let transform2 = c
                .transform
                .trans(x,y)
                .rot_rad(rotation)
                .trans(25.0,25.0);

                let transform3 = c
                .transform
                .trans(x,y)
                .rot_rad(rotation)
                .trans(-125.0,-125.0);

                let transform4 = c
                .transform
                .trans(x,y)
                .rot_rad(rotation)
                .trans(25.0,-125.0);

                let transform5 = c
                .transform
                .trans(x,y)
                .rot_rad(rotation)
                .trans(-125.0,25.0);

            // Draw a box rotating around the middle of the screen.
            rectangle(WHITE, square1, transform, gl);
            rectangle(RED, square2, transform3, gl);
            rectangle(RED, square2, transform4, gl);
            rectangle(RED, square2, transform2, gl);
            rectangle(RED, square2, transform5, gl);
        });
    }

    fn update(&mut self, args: &UpdateArgs) {
        // Rotate 2 radians per second.
        self.rotation += 2.0 * args.dt;
    }
}

fn main() {
    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V3_2;

    // Create an Glutin window.
    let mut window: Window = WindowSettings::new("SquareSpin", [200, 200])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    // Create a new game and run it.
    let mut app = App {
        gl: GlGraphics::new(opengl),
        rotation: 0.0,
    };

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.render_args() {
            app.render(&args);
        }

        if let Some(args) = e.update_args() {
            app.update(&args);
        }
    }
}