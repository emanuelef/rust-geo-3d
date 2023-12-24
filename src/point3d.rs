#[derive(Debug)]

pub struct Point3D {
	pub(crate) x: f64,
	pub(crate) y: f64,
	pub(crate) z: f64,
}

impl Point3D {
	pub fn add(&self, other: Point3D) -> Point3D {
		Point3D {
			x: self.x + other.x,
			y: self.y + other.y,
			z: self.z + other.z,
		}
	}

	pub fn sub(&self, other: Point3D) -> Point3D {
		Point3D {
			x: self.x - other.x,
			y: self.y - other.y,
			z: self.z - other.z,
		}
	}

	pub fn multiply_by_scalar(&self, scalar: f64) -> Point3D {
		Point3D {
			x: self.x * scalar,
			y: self.y * scalar,
			z: self.z * scalar,
		}
	}

	pub fn dot(&self, other: Point3D) -> f64 {
		self.x * other.x + self.y * other.y + self.z * other.z
	}

	pub fn distance(&self, other: Point3D) -> f64 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        let dz = self.z - other.z;
        let len = f64::sqrt(dx * dx + dy * dy + dz * dz);
        if len == 0.0 {
            return 0.0;
        } else {
            return len;
        }	}

}
