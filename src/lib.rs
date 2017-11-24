extern crate piston_window;
extern crate nalgebra as na;
extern crate ncollide as nc;
extern crate gfx_device_gl;
extern crate gfx_graphics;
extern crate gfx;

use piston_window::*;

const WINDOW_WIDTH: f64 = 600.0;
const WINDOW_HEIGHT: f64 = 600.0;

pub struct App {
    
}

impl App {
    pub fn new() -> App {
        App {}
    }

    fn on_draw<E: GenericEvent>(&mut self, e: &E, w: &mut PistonWindow) {
        let size = w.size();
        w.draw_2d(e, |c, g| {
            clear([0.0, 0.0, 0.0, 1.0], g);
        });
    }

    fn on_input<E: GenericEvent>(&mut self, e: &E) {
    }

    fn on_update(&mut self, upd: &UpdateArgs) {
    }

}

pub fn run() {
    let mut window: PistonWindow = WindowSettings::new(
        "rustmill",
        [600, 600]
    )
    .exit_on_esc(true)
    .build()
    .unwrap();

    let x = 3;
    let y = &x;

    let mut app = App::new();
    
    while let Some(e) = window.next() {
        app.on_input(&e);
        if let Some(upd) = e.update_args() {
            app.on_update(&upd);
        }
        app.on_draw(&e, &mut window);
    }
}