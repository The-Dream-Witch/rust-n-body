extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderArgs, RenderEvent, UpdateArgs, UpdateEvent};
use piston::window::WindowSettings;
use n_body_sim::*;

pub struct Sim {
    gl: GlGraphics, // OpenGL drawing backend.
}

impl Sim {
    fn render(&mut self, args: &RenderArgs, nbodies: &Nbodies) {
        use graphics::*;
        use graphics::color::*;
        
        let square = rectangle::square(0.0, 0.0, 2.0);

        self.gl.draw(args.viewport(), |c, gl| {
            // Clear the screen.
            clear(BLACK, gl);
            //Draw each body to the screen in its updated position
            for body in &nbodies.bodies {
                let transform = c
                    .transform
                    .trans(body.pos.0, body.pos.1);

                    if body.pos.2 >= 0. {
                       rectangle(RED, square, transform, gl);
                    } else {
                        rectangle(WHITE, square, transform, gl);
                    }
            }
        });
    }
}

fn main() {
    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V3_2;

    // Create an Glutin window.
    let mut window: Window = WindowSettings::new("N-Body-Sim", [800, 800])
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
            sim.render(&args,&nbodies);
            nbodies.next();
        }
    }
}