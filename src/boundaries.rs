#[derive(Debug)]
pub struct Boundaries {
    top_left_corner: (f64, f64),
    bottom_right_corner: (f64, f64)
}

impl Boundaries {
    pub fn new(top_left_corner: (f64, f64), bottom_right_corner: (f64, f64)) -> Boundaries {
        Boundaries {
            top_left_corner,
            bottom_right_corner
        }
    }

    pub fn as_rectangle(&self) -> [f64; 4] {
        let (x, y) = self.top_left_corner;
        let (x2, y2) = self.bottom_right_corner;
        [x, y, (x2 - x), (y2 - y)]
    }
}