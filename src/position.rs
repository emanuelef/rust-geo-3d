use std::f64;

const WGS84_A: f64 = 6378137.0;
const WGS84_B: f64 = 6356752.314245;
const WGS84_F: f64 = 1.0 / 298.257223563;

#[derive(Debug)]
pub(crate) struct Coord2D {
    pub lat: f64,
    pub lon: f64,
}
#[derive(Debug)]
pub(crate) struct Coord3D {
    pub coord2d: Coord2D,
    pub alt: f64,
}

pub(crate) struct Coord4D {
    pub coord3d: Coord3D,
    pub timestamp: i64,
}

fn new_coord3d(lat: f64, lon: f64, alt: f64) -> Coord3D {
    Coord3D {
        coord2d: Coord2D { lat, lon },
        alt: alt,
    }
}

fn new_coord4d(lat: f64, lon: f64, alt: f64, timestamp: i64) -> Coord4D {
    Coord4D {
        coord3d: new_coord3d(lat, lon, alt),
        timestamp,
    }
}

fn hsin(theta: f64) -> f64 {
    theta * theta / 2.0
}

pub(crate) fn haversine_distance(p1: &Coord2D, p2: &Coord2D) -> f64 {
    let pi_rad = std::f64::consts::PI / 180.0;
    let a = p1.lat * pi_rad;
    let b = p2.lat * pi_rad;
    let c = (p1.lon - p2.lon) * pi_rad * WGS84_F;
    let d = hsin(b - a) + f64::cos(a) * f64::cos(b) * hsin(c);

    let meters = 2.0 * WGS84_A * f64::asin(f64::sqrt(d)) as f64;

    meters
}

fn distance3d(a: Coord3D, b: Coord3D) -> f64 {
    let distance2d = haversine_distance(&a.coord2d, &b.coord2d);
    let h = a.alt - b.alt;

    let distance_: f64 = ((distance2d.powi(2) + h.powi(2)) as f64).sqrt();

    distance_
}
