extern crate piston_window;
extern crate nalgebra as na;
extern crate ncollide as nc;
extern crate gfx_device_gl;
extern crate gfx_graphics;
extern crate gfx;

extern crate image;

use std::env;
use std::path::Path;

use image::GenericImage;

mod boundaries;
use boundaries::Boundaries;

mod polygon;

use piston_window::*;

use std::thread;
use std::sync::mpsc;
use std::sync::mpsc::*;

mod recognition;

type Coord = (u32, u32);

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

    fn on_draw<E: GenericEvent>(&mut self, e: &E, w: &mut PistonWindow) {
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

    fn on_input<E: GenericEvent>(&mut self, _e: &E) {
    }

    fn on_update(&mut self, _upd: &UpdateArgs, rx: &Receiver<Vec<Boundaries>>) {
        match rx.try_recv() {
            Err(_)              => (),
            Ok(boundaries_vec)  => {
                println!("Found {} polygons", boundaries_vec.len());
                self.found_objects = boundaries_vec;
            }
        };
    }
}

pub fn run() {
    let file = if env::args().count() == 2 {
        env::args().nth(1).unwrap()
    } else {
        panic!("Please enter a file")
    };
    let path = Path::new(&file);

    let opengl = OpenGL::V3_2;

    let img = image::open(&path).unwrap();
    let (img_w, img_h) = img.dimensions();
    println!("dimensions {:?}", img.dimensions());
    println!("{:?}", img.color());

    let mut window: PistonWindow = WindowSettings::new(
            "rustmill",
            [img_w, img_h]
        )
        .exit_on_esc(true)
        .opengl(opengl)
        .build()
        .unwrap();

    let image_to_draw: G2dTexture = Texture::from_path(
        &mut window.factory,
        &path,
        Flip::None,
        &TextureSettings::new()
    ).unwrap();

    let mut app = App::new(image_to_draw);
    let treshold = 140;
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        let found_objects = recognition::find_objects(img, treshold);
        tx.send(found_objects).unwrap();
    });

    while let Some(e) = window.next() {
        app.on_input(&e);
        if let Some(upd) = e.update_args() {
            app.on_update(&upd, &rx);
        }
        app.on_draw(&e, &mut window);
    }
}
