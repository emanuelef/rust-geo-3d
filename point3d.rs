pub struct Point3D {
	x: f64,
	y: f64,
	z: f64,
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
		use std::f64::sqrt;
		sqrt(square(self.x - other.x) + square(self.y - other.y) + square(self.z - other.z))
	}

	fn square(&self, x: f64) -> f64 {
		x * x
	}
}
