mod point3d;

fn main() {
    let p1 = point3d::Point3D { x: 1.0, y: 2.0, z: 3.0 };
    let p2 = point3d::Point3D { x: 4.0, y: 5.0, z: 6.0 };
    let p3 = p1.add(p2);
    print!("p3 = {:?}\n", p3);
}
