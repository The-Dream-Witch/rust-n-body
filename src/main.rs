extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use glutin_window::GlutinWindow as Window;
use n_body_sim::*;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderArgs, RenderEvent};
use piston::window::WindowSettings;

pub struct Sim {
    gl: GlGraphics, // OpenGL drawing backend.
}

impl Sim {
    fn render(&mut self, args: &RenderArgs, nbodies: &Nbodies) {
        use graphics::color::*;
        use graphics::*;

        self.gl.draw(args.viewport(), |c, gl| {
            // Clear the screen.
            clear(BLACK, gl);
            //Draw each body to the screen in its updated position
            for body in &nbodies.bodies {
                let transform = c.transform.trans(body.pos.0, body.pos.1);

                let size = (body.pos.2 / 80.).ceil();
                let circle = ellipse::circle(0.0, 0.0, size);
                let colors = [
                    hex("5c5c5c"),
                    hex("747575"),
                    hex("949494"),
                    hex("aba9a9"),
                    hex("c2c0c0"),
                    hex("dbd9d9"),
                    hex("edebeb"),
                    hex("ffffff"),
                ];
                let mut size = (body.pos.2 / 100.).ceil() as usize;
                if size == 0 {
                    size = 1;
                }
                ellipse(colors[size - 1], circle, transform, gl);
            }
        });
    }
}

fn main() {
    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V3_2;

    // Create an Glutin window.
    let mut window: Window = WindowSettings::new("N-Body-Sim", [XMAX, YMAX])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    // Create a new game and run it.
    let mut sim = Sim {
        gl: GlGraphics::new(opengl),
    };

    let mut events = Events::new(EventSettings::new());
    let mut nbodies = Nbodies::new(1000);

    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.render_args() {
            sim.render(&args, &nbodies);
            nbodies.update_with_tree();
        }
    }
}
