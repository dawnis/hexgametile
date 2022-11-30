use nannou::color::encoding::Srgb;
use nannou::color::rgb::Rgb;
use hex2d::Coordinate;
use nannou::prelude::*;

pub mod hexagon;
pub mod terrain;

///Type alias for nannou color type
pub type Mrgb = Rgb<Srgb, u8>;

/// Hexagonal tiles implement a trait specifying their axial location and how to draw them. 
pub trait Rawr {
    fn rawr(&self, c: Coordinate, d: &Draw, bounds: Rect);
}
