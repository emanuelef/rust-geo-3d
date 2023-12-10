use std::f64;

const WGS84_a: f64 = 6378137.0;
const WGS84_b: f64 = 6356752.314245;
const WGS84_e2: f64 = 1 - (math::powf(WGS84_b, 2) / math::powf(WGS84_a, 2));

struct Point3D {
    X: f64,
    Y: f64,
    Z: f64,
}

struct Coord3D {
    coord2d: Coord2D,
    alt: f64,
}

fn lat_lon_alt_to_xyz_wgs84(pos: Coord3D) -> Point3D {
    let latr: f64 = pos.lat / 90.0 * 0.5 * std::f64::consts::PI;
    let lonr: f64 = pos.lon / 180.0 * std::f64::consts::PI;

    let Nphi = WGS84_a / math::sqrt(1.0 - WGS84_e2 * math::powf(math::sin(latr), 2));

    let x: f64 = (Nphi + pos.alt) * math::cos(latr) * math::cos(lonr);
    let y: f64 = (Nphi + pos.alt) * math::cos(latr) * math::sin(lonr);
    let z: f64 = (math::powf(WGS84_b, 2) / math::powf(WGS84_a, 2) * Nphi + pos.alt) * math::sin(latr);

    return Point3D { X: x, Y: y, Z: z };
}

fn xyz_wgs84_to_lat_lon_alt(point: Point3D) -> Coord3D {
    let a = WGS84_a;
    let b = WGS84_b;
    let f = (a - b) / a;
    let e_sq = f * (2.0 - f);
    let eps = e_sq / (1.0 - e_sq);

    let p = math::sqrt(math::powf(point.X, 2) + math::powf(point.Y, 2));
    let q = math::atan2((point.Z * a), (p * b));

    let sin_q = math::sin(q);
    let cos_q = math::cos(q);

    let sin_q_3: f64 = sin_q * sin_q * sin_q;
    let cos_q_3: f64 = cos_q * cos_q * cos_q;

    let phi = math::atan2((point.Z + eps * b * sin_q_3), (p - e_sq * a * cos_q_3));
    let lam = math::atan2(point.Y, point.X);

    let v = a / math::sqrt(1.0 - e_sq * math::powf(math::sin(phi), 2));
    let h = (p / math::cos(phi)) - v;

    let lat: f64 = RadiansToDegrees(phi);
    let lon: f64 = RadiansToDegrees(lam);

    return Coord3D {
        coord2d: Coord2D { lat, lon },
        alt: h,
    };
}

fn radians_to_degrees(radians: f64) -> f64 {
    (radians * 180.0) / std::f64::consts::PI
}

