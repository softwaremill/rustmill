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
        
        let mut polygons: Vec<Polygon> = vec![];
        for coord in &vector {
            let p1 = polygons.clone();
            let matching_polygons: Vec<usize> = p1
                .iter()
                .enumerate()
                .filter_map(|(idx, p)| {
                    if (p.contains_neighbour(&coord)) {
                        Some(idx)
                    } else {
                        None
                    }
                })
                .collect();
            let len: usize = matching_polygons.len();

            if (len == 0) {
                polygons.push(Polygon::new(&coord));
            } else if (len == 1) {
                polygons.get_mut(matching_polygons[0]).unwrap().add(&coord);
            } else if (len == 2) {
                add_and_merge(polygons, &coord, matching_polygons[0], matching_polygons[1]);
            }
        }

        println!("{:?}", vector);
    }

}

pub fn add_and_merge(polygons: Vec<Polygon>, coord: &Coord, first: usize, second: usize) -> Vec<Polygon> {
    let mut newVec: Vec<Polygon> = vec!();
    let mut first_polygon: Option<Polygon> = None; //= &polygons[first];
    let mut second_polygon: Option<Polygon> = None; //= &polygons[second];

    for (idx, polygon) in polygons.into_iter().enumerate() {
        if idx == first {
            first_polygon = Some(polygon);
        } else if idx == second {
            second_polygon = Some(polygon);
        } else {
            newVec.push(polygon);
        }
    }
    
    let merged = first_polygon.unwrap().add(coord).merge(&second_polygon.unwrap());
    newVec.push(merged);
    newVec
    // first_polygon.add(coord);
    // first_polygon.merge(second_polygon);
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
pub struct Polygon {
    pixels: Vec<Coord>
}

impl Polygon {
    
    pub fn new(coord: &Coord) -> Polygon {
        let mut pixels: Vec<Coord> = Vec::new();
        pixels.push(*coord);
        Polygon { pixels }
    }

    pub fn contains_neighbour(&self, coord: &Coord) -> bool {
        let &(x, y) = coord;
        self.pixels.iter().find(|&&c| {
            c == (x, y - 1) || c == (x - 1, y)
        }).is_some()
    }

    pub fn add(&self, coord: &Coord) -> Polygon {
        let mut newVector: Vec<Coord> = vec![];
        for coord in self.pixels.iter() {
            newVector.push(*coord);
        }
        newVector.push(*coord);
        Polygon {
            pixels: newVector
        }
    }

    pub fn merge(&self, oth: &Polygon) -> Polygon {
        let mut newVector: Vec<Coord> = vec![];
        for coord in self.pixels.iter() {
            newVector.push(*coord);
        }
        for coord in oth.pixels.iter() {
            newVector.push(*coord);            
        }
        Polygon {
            pixels: newVector
        }
    }

}