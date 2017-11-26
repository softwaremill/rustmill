use ::std;
use ::boundaries::*;

type Coord = (u32, u32);

#[derive(Debug)]
pub struct Polygon {
    pixels: Vec<Coord>
}

impl Polygon {
    
    pub fn new(coord: Coord) -> Polygon {
        Polygon {
            pixels: vec![coord]
        }
    }

    pub fn contains_neighbour(&self, coord: &Coord) -> bool {
        let &(x, y) = coord;
        self.pixels.iter().find(|&&c| {
            (y > 0 && c == (x, y - 1)) || ( x > 0 && c == (x - 1, y))
        }).is_some()
    }

    pub fn boundaries(&self) -> Boundaries {
        let xs: Vec<u32> = self.pixels.iter().map(|&(x, _)| x).collect();
        let min_x = *xs.iter().min().unwrap() as f64;
        let max_x = *xs.iter().max().unwrap() as f64;

        let ys: Vec<u32> = self.pixels.iter().map(|&(_, y)| y).collect();
        let min_y = *ys.iter().min().unwrap() as f64;
        let max_y = *ys.iter().max().unwrap() as f64;

        Boundaries::new(
            (min_x, min_y), 
            (max_x, max_y)
        )
    }

}

impl std::ops::Add for Polygon {
    type Output = Polygon;

    fn add(mut self, mut oth: Polygon) -> Polygon {
        self.pixels.append(&mut oth.pixels);
        self
    }
}
