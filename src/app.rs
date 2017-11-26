use ::piston_window::*;
use ::boundaries::Boundaries;
use std::sync::mpsc::Receiver;

pub struct App {
    image_to_draw: G2dTexture,
    found_objects: Vec<Boundaries>
}

impl App {
    pub fn new(image_to_draw: G2dTexture) -> App {  
        App {
            image_to_draw,
            found_objects: Vec::new()
        }
    }

    pub fn on_draw<E: GenericEvent>(&mut self, e: &E, w: &mut PistonWindow) {
        w.draw_2d(e, |c, g| {
            clear([0.0, 0.0, 0.0, 1.0], g);
            image(&self.image_to_draw, c.transform, g);
            for boundaries in self.found_objects.iter() {
                rectangle(
                    [1.0, 0.0, 0.0, 0.5],
                    boundaries.as_rectangle(),
                    c.transform,
                    g
                );
            }
        });
    }

    pub fn on_input<E: GenericEvent>(&mut self, _e: &E) {
    }

    pub fn on_update(&mut self, _upd: &UpdateArgs, rx: &Receiver<Vec<Boundaries>>) {
        match rx.try_recv() {
            Err(_)              => (),
            Ok(boundaries_vec)  => {
                println!("Found {} polygons", boundaries_vec.len());
                self.found_objects = boundaries_vec;
            }
        };
    }
}