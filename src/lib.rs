extern crate piston_window;
extern crate nalgebra as na;
extern crate ncollide as nc;
extern crate gfx_device_gl;
extern crate gfx_graphics;
extern crate gfx;

extern crate image;

use std::env;
use std::path::Path;

use image::Pixel;
use image::GenericImage;
use image::DynamicImage;

mod boundaries;
use boundaries::Boundaries;

mod polygon;
use polygon::Polygon;

use piston_window::*;

type Coord = (u32, u32);

pub struct App {
    img: DynamicImage,
    treshold: u8,
    image_to_draw: G2dTexture,
    found_objects: Vec<(Boundaries)>
}

impl App {
    pub fn new(img: DynamicImage, treshold: u8, image_to_draw: G2dTexture) -> App {  
        App {
            img,
            treshold,
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

    fn on_update(&mut self, _upd: &UpdateArgs) {
    }

    fn find_objects(&mut self) {
        let polygons: Vec<Polygon> = self.img
            .pixels()
            .filter_map(|(x, y, pixel)| {
                let luma_value = pixel.to_luma().data[0];
                if luma_value > self.treshold {
                    Some((x, y))
                } else {
                    None
                }
            })
            .fold(Vec::<Polygon>::new(), |found_polygons, coord| {
                let (matching, mut rest): (Vec<Polygon>, Vec<Polygon>) = found_polygons
                    .into_iter()
                    .partition(|p| p.contains_neighbour(&coord));
                
                let new_poly: Polygon = matching
                    .into_iter()
                    .fold(Polygon::new(coord), |p1, p2| p1 + p2);

                rest.push(new_poly);
                rest
            });

        println!("Found {} polygons", polygons.len());

        self.found_objects = polygons
            .into_iter()
            .map(|p| p.boundaries())
            .collect()
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

    let mut app = App::new(img, 100, image_to_draw);

    app.find_objects();

    while let Some(e) = window.next() {
        app.on_input(&e);
        if let Some(upd) = e.update_args() {
            app.on_update(&upd);
        }
        app.on_draw(&e, &mut window);
    }
}
