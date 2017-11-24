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

use image::Pixel;
use image::GenericImage;
use image::DynamicImage;
use image::ImageDecoder;
use image::png::PNGDecoder;

use piston_window::*;

pub struct App {
    img: DynamicImage,
    image_to_draw: G2dTexture,
    found_objects: Vec<((f64, f64), (f64, f64))>
}

impl App {
    pub fn new(img: DynamicImage, image_to_draw: G2dTexture) -> App {  
        App {
            img,
            image_to_draw,
            found_objects: Vec::new()
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

    fn find_objects(&mut self) {
        let treshold = 20;
        // let objects: Vec<Object> = Vec::new();

        let vector: Vec<(u32, u32)> = self.img
            .pixels()
            .filter(|&(x, y, pixel)| {
                pixel.to_luma().data[0] > treshold
            })
            .map(|(x, y, pixel)| {
                (x, y)
            })
            .collect();
        
        let zero: Vec<Polygon> = vec![];
        vector.iter().fold(&zero, |polygons, coord| {
            let matching_polygons: Vec<&Polygon> = polygons
                .iter()
                .filter(|p| p.contains_neighbour(coord))
                .collect();

            let len: usize = matching_polygons.len();

            if (len == 1) {
                // matching_polygons.get_mut(0).unwrap().add(coord);
                // let mut poly = matching_polygons.get_mut(0).unwrap();
                // poly.add(coord);
                // let polygon: &Polygon = matching_polygons[0];
                // polygon.add(coord);
                // matching_polygons[0].add(coord);
            } else if (len == 2) {
                // let first_polygon = polygons.get_mut(matching_polygon_ids[0]).unwrap();
                // let second_polygon = polygons.get_mut(matching_polygon_ids[1]).unwrap();
                // first_polygon.add(coord);
                // first_polygon.merge(second_polygon);
                // matching_polygons[0].add(coord);
                // matching_polygons[0].merge(*matching_polygons[1]);
            }
            // match len {
            //     0 => polygons,
            //     1 => {
            //             matching_polygons[0].add(coord);
            //             polygons
            //         },
            //     2 => {
            //         matching_polygons[0].add(coord).merge(*matching_polygons[1]);
            //         polygons
            //     },
            //     _ => panic!("Something went wrong"),
            // };
            polygons
        });

        println!("{:?}", vector);
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

    app.find_objects();

    while let Some(e) = window.next() {
        app.on_input(&e);
        if let Some(upd) = e.update_args() {
            app.on_update(&upd);
        }
        app.on_draw(&e, &mut window);
    }
}

type Coord = (u32, u32);

#[derive(Clone, Debug)]
struct Polygon {
    pixels: Vec<Coord>
}

impl Polygon {

    pub fn new() -> Polygon {
        Polygon { pixels: Vec::new() }
    }

    pub fn contains_neighbour(&self, coord: &Coord) -> bool {
        let &(x, y) = coord;
        self.pixels.iter().find(|&&c| {
            c == (x, y - 1) || c == (x - 1, y)
        }).is_some()
    }

    pub fn add(&mut self, coord: &Coord) {
        self.pixels.push(*coord);
    }

    pub fn merge(&mut self, oth: &mut Polygon) {
        self.pixels.append(&mut oth.pixels);
    }

}