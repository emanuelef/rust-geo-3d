use std::f64;

const WGS84_a: f64 = 6378137.0;
const WGS84_b: f64 = 6356752.314245;
const WGS84_f: f64 = 1 / 298.257223563;

struct Coord2D {
    lat: f64,
    lon: f64,
}

struct Coord3D {
    coord2d: Coord2D,
    alt: f64,
}

struct Coord4D {
    coord3d: Coord3D,
    timestamp: i64,
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

fn haversine_distance(p1: Coord2D, p2: Coord2D) -> f64 {
    let pi_rad = std::f64::consts::PI / 180.0;
    let a = p1.lat * pi_rad;
    let b = p2.lat * pi_rad;
    let c = (p1.lon - p2.lon) * pi_rad * WGS84_f;
    let d = hsin(b - a) + math::cos(a) * math::cos(b) * hsin(c);

    let meters = 2.0 * WGS84_a * math::asin(math::sqrt(d)) as f64;

    meters
}

fn distance3d(a: Coord3D, b: Coord3D) -> f64 {
    let distance2d = haversine_distance(a.coord2d, b.coord2d);
    let h = a.alt - b.alt;

    let distance_: f64 = ((distance2d.powi(2) + h.powi(2)) as f64).sqrt();

    distance_
}
