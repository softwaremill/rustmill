extern crate piston_window;
extern crate nalgebra as na;
extern crate ncollide as nc;
extern crate gfx_device_gl;
extern crate gfx_graphics;
extern crate gfx;

extern crate image;

use std::env;
use std::fs::File;
use std::path::Path;

use image::GenericImage;
use image::DynamicImage;

use piston_window::*;

pub struct App {
    img: DynamicImage,
    image_to_draw: G2dTexture
}

impl App {
    pub fn new(img: DynamicImage, image_to_draw: G2dTexture) -> App {  
        App {
            img,
            image_to_draw
        }
    }

    fn on_draw<E: GenericEvent>(&mut self, e: &E, w: &mut PistonWindow) {
        let size = w.size();
        w.draw_2d(e, |c, g| {
            clear([0.0, 0.0, 0.0, 1.0], g);
            image(&self.image_to_draw, c.transform, g);
        });
    }

    fn on_input<E: GenericEvent>(&mut self, e: &E) {
    }

    fn on_update(&mut self, upd: &UpdateArgs) {
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

    let mut app = App::new(img, image_to_draw);
    while let Some(e) = window.next() {
        app.on_input(&e);
        if let Some(upd) = e.update_args() {
            app.on_update(&upd);
        }
        app.on_draw(&e, &mut window);
    }
}