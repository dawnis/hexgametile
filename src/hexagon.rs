use nannou::prelude::*;
use crate::Rawr;
use crate::terrain::Terrain;
use hex2d::{Coordinate, Spacing};

#[derive(Debug)]
pub struct HexagonalTile {
    edge: f32,
    pub terrain: Terrain,
}

impl HexagonalTile {
    pub fn new(edge: f32, terrain: Terrain) -> Self {
        HexagonalTile {
            edge,
            terrain,
        }
    }

}

impl Rawr for HexagonalTile {
    fn rawr(&self, axial: Coordinate, draw: &Draw, bounding_coordinates: Rect) {
        let hexagon_pixel_ctr = axial.to_pixel(Spacing::FlatTop(self.edge));

        if bounding_coordinates.left() < hexagon_pixel_ctr.0 && bounding_coordinates.right() > hexagon_pixel_ctr.0 
           && bounding_coordinates.bottom() < hexagon_pixel_ctr.1  && bounding_coordinates.top() >  hexagon_pixel_ctr.1 {
            let step = 60;
            let points = (0..=360).step_by(step).map(|i| {
                let radian = deg_to_rad(i as f32);
                let x = radian.cos() * self.edge + hexagon_pixel_ctr.0;
                let y = radian.sin() * self.edge + hexagon_pixel_ctr.1;
                (pt2(x, y), self.terrain.color())
            });
            draw.polygon().points_colored(points);
            let points = (0..=360).step_by(step).map(|i| {
                let radian = deg_to_rad(i as f32);
                let x = radian.cos() * self.edge + hexagon_pixel_ctr.0;
                let y = radian.sin() * self.edge + hexagon_pixel_ctr.1;
                (pt2(x, y), BLACK)
            });
            draw.polyline().weight(1.0).points_colored(points);
        }
    }
}
