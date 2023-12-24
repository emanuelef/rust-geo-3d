use crate::position::haversine_distance;
mod point3d;
mod position;

fn main() {
    let p1 = point3d::Point3D { x: 1.0, y: 2.0, z: 3.0 };
    let p2 = point3d::Point3D { x: 4.0, y: 5.0, z: 6.0 };
    let p3 = p1.add(p2);
    print!("p3 = {:?}\n", p3);

    let position1 = position::Coord3D {
        coord2d: position::Coord2D { lat: 1.0, lon: 2.0 },
        alt: 3.0,
    };  

    let position2 = position::Coord3D {
        coord2d: position::Coord2D { lat: 3.0, lon: 7.0 },
        alt: 6.0,
    };  

    print!("position1 = {:?}\n", position1);

    let distance = haversine_distance(&position1.coord2d, &position2.coord2d);

    print!("distance = {:?}\n", distance);
}
