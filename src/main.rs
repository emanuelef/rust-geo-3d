use std::time::Instant;
use rayon::prelude::*;
use crate::position::haversine_distance;
mod point3d;
mod position;

fn main() {
    let p1 = point3d::Point3D { x: 1.0, y: 2.0, z: 3.0 };
    let p2 = point3d::Point3D { x: 4.0, y: 5.0, z: 6.0 };
    let p3 = p1.add(p2);
    println!("p3 = {:?}", p3);

    let position1 = position::Coord3D {
        coord2d: position::Coord2D { lat: 1.0, lon: 2.0 },
        alt: 3.0,
    };

    let position2 = position::Coord3D {
        coord2d: position::Coord2D { lat: 3.0, lon: 7.0 },
        alt: 6.0,
    };

    println!("position1 = {:?}", position1);

    let start_time = Instant::now();

    // Use Rayon's parallel iterator to parallelize the computation
    (0..250_000_000).into_par_iter().for_each(|_| {
        let _distance = haversine_distance(&position1.coord2d, &position2.coord2d);
    });

    let end_time = Instant::now();
    let duration = end_time - start_time;
    println!("Total time taken: {:?}", duration);
}
