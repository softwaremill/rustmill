use ::image::Pixel;
use ::image::GenericImage;
use ::image::DynamicImage;
use ::boundaries::*;
use ::polygon::*;

pub fn find_objects(img: DynamicImage, treshold: u8) -> Vec<Boundaries> {

    img.pixels()
        .filter_map(|(x, y, pixel)| {
            let luma_value = pixel.to_luma().data[0];
            if luma_value > treshold {
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
        })
        .into_iter()
        .map(|p| p.boundaries())
        .collect()
        
}