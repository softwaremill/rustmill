extern crate piston_window;
use piston_window::*;

extern crate nalgebra as na;
extern crate ncollide as nc;
extern crate gfx_device_gl;
extern crate gfx_graphics;
extern crate gfx;

extern crate image;
use image::GenericImage;

use std::env;
use std::path::Path;
use std::thread;
use std::sync::mpsc;

mod recognition;
mod boundaries;
mod polygon;
mod app;
use app::App;

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
