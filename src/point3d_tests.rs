use std::f64;

#[cfg(test)]
mod point3d_tests {
    use super::Point3D;

    #[test]
    fn add() {
        let p1 = Point3D { x: 1.0, y: 2.0, z: 3.0 };
        let p2 = Point3D { x: 4.0, y: 5.0, z: 6.0 };
        let p3 = p1 + p2;
        assert_eq!(p3, Point3D { x: 5.0, y: 7.0, z: 9.0 });
    }

    #[test]
    fn sub() {
        let p1 = Point3D { x: 1.0, y: 2.0, z: 3.0 };
        let p2 = Point3D { x: 4.0, y: 5.0, z: 6.0 };
        let p3 = p1 - p2;
        assert_eq!(p3, Point3D { x: -3.0, y: -3.0, z: -3.0 });
    }

    #[test]
    fn multiply_by_scalar() {
        let p = Point3D { x: 2.0, y: 3.0, z: 4.0 };
        let p2 = p * 2.0;
        assert_eq!(p2, Point3D { x: 4.0, y: 6.0, z: 8.0 });
    }

    #[test]
    fn dot() {
        let p1 = Point3D { x: 1.0, y: 2.0, z: 3.0 };
        let p2 = Point3D { x: 4.0, y: 5.0, z: 6.0 };
        assert_eq!(p1.dot(p2), 32.0);
    }

    #[test]
    fn distance() {
        let p1 = Point3D { x: 1.0, y: 2.0, z: 3.0 };
        let p2 = Point3D { x: 4.0, y: 5.0, z: 6.0 };
        assert_eq!(p1.distance(p2), 5.0);
    }
}
