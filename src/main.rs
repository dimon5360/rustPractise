extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;
extern crate piston_window;

use glutin_window::GlutinWindow as WindowSettings;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderArgs, RenderEvent, UpdateArgs, UpdateEvent};
// use piston_window::*;

pub struct App {
    gl: GlGraphics, // OpenGL drawing backend.
    rotation: f64,  // Rotation for the square.
}

impl App {
    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
    }
}

// Application version v.0.0.2 from 09.06.2020
const APP_VERSION: &'static str = "0.0.2";

/***************************************
 * @brief Main method in application
 */
fn main() {
    
    use piston_window::*;
    println!("Hello, %username%! Application version v.{}.", APP_VERSION);
    println!("Application started.");

    let mut window: PistonWindow = WindowSettings::new(
        "piston: hello_world",
        [640, 480]
    )
    .exit_on_esc(true).build().unwrap();

    while let Some(event) = window.next() {
        window.draw_2d(&event, |context, graphics, _device| {
            clear([1.0; 4], graphics);
            rectangle(  [1.0, 0.0, 0.0, 1.0],
                        [0.0, 0.0, 100.0, 100.0], 
                        context.transform, graphics);
        });
    }
 }