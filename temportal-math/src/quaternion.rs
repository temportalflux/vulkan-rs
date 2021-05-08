use super::*;

pub type Quaternion = Vector<f32, 4>;

impl Quaternion {
	pub fn identity() -> Self {
		Quaternion::new([0.0, 0.0, 0.0, 1.0])
	}

	/// Creates a quaternion from an quantity of radians to rotate around an axis.
	pub fn from_axis_angle(axis: Vector<f32, 3>, radians: f32) -> Quaternion {
		(axis * f32::sin(radians * 0.5)).subvec(None)
			+ (Quaternion::identity() * f32::cos(radians * 0.5))
	}

	/// Returns the euler representation of the quaternion.
	pub fn to_euler(&self) -> Vector<f32, 3> {
		// See https://en.wikipedia.org/wiki/Conversion_between_quaternions_and_Euler_angles#Quaternion_to_Euler_Angles_Conversion
		let sinp = 2.0 * (self.w() * self.z() - self.y() * self.x());
		Vector::new([
			f32::atan2(
				2.0 * (self.w() * self.x() + self.y() * self.z()),
				1.0 - (2.0 * (self.x().powi(2) + self.z().powi(2))),
			),
			f32::atan2(
				2.0 * (self.w() * self.y() + self.x() * self.z()),
				1.0 - (2.0 * (self.y().powi(2) + self.z().powi(2))),
			),
			if sinp.abs() >= 1.0 {
				(std::f32::consts::PI / 2.0).copysign(sinp)
			} else {
				f32::asin(sinp)
			},
		])
	}

	pub fn conjugate(&self) -> Self {
		// <-x, -y, -z, w>
		let xyz: Vector<f32, 3> = -self.subvec(None);
		xyz.subvec(None) + (Quaternion::identity() * self.w())
	}

	pub fn inverse(&self) -> Self {
		self.conjugate().normal()
	}

	/// Returns a quaternion representation a rotation first by `a` then by `b`.
	pub fn concat(a: &Quaternion, b: &Quaternion) -> Quaternion {
		// https://en.wikipedia.org/wiki/Quaternion#Hamilton_product
		// a1a2 - b1b2 - c1c2 - d1d2
		// + (a1b2 + b1a2 + c1d2 - d1c2)i
		// + (a1c2 - b1d2 + c1a2 + d1b2)j
		// + (a1d2 + b1c2 - c1b2 + d1a2)k
		// aka
		// where:
		//   a1+b1i+c1j+d1k = `a<x, y, z, w>`
		//   a2+b2i+c2j+d2k = `b<x, y, z, w>`
		// w: awbw - axbx - ayby - azbz
		// x: awbx + axbw + aybz - azby
		// y: awby - axbz + aybw + azbx
		// z: awbz + axby - aybx + azbw

		let a_real = a.subvec::<3>(None);
		let b_real = b.subvec::<3>(None);
		// aw*bw - (axbw + ayby + azbz)
		let w = a.w() * b.w() - Vector::dot(&a_real, &b_real);
		// <ay*bz - az*by, az*bx - ax*bz, ax*by - ay*bx>
		let cross_real = Vector::cross(&a_real, &b_real);
		// vec: a*bw + b*aw + cross
		//   x: awbx + axbw + (aybz - azby)
		//   y: awby + aybw + (azbx - axbz)
		//   z: awbz + azbw + (axby - aybx)
		let vec = (b_real * a.w()) + (a_real * b.w()) + cross_real;
		vec.subvec::<4>(None) + (Quaternion::identity() * w)
	}

	/// Rotates a vector by the rotation `self`, returning the result.
	/// Does not modify the provided vector.
	pub fn rotate(&self, vec: &Vector<f32, 3>) -> Vector<f32, 3> {
		let real = self.subvec::<3>(None);
		let a = real * 2.0 * Vector::dot(&real, &vec);
		let b = (*vec) * (self.w().powi(2) - real.magnitude_sq());
		let c = Vector::cross(&real, &vec) * 2.0 * self.w();
		a + b + c
	}

	pub fn look_at_2d(
		prev_forward: &Vector<f32, 2>,
		next_forward: &Vector<f32, 2>,
		world_out: &Vector<f32, 3>,
	) -> Quaternion {
		let dot = Vector::dot(prev_forward, next_forward);
		if f32::abs(dot + 1.0) <= f32::EPSILON {
			return Quaternion::from_axis_angle(*world_out, 180_f32.to_radians());
		} else if f32::abs(dot - 1.0) <= f32::EPSILON {
			return Quaternion::identity();
		} else {
			let angle = f32::acos(dot);
			Quaternion::from_axis_angle(*world_out, angle)
		}
	}

	pub fn look_at_3d(
		prev_forward: &Vector<f32, 3>,
		next_forward: &Vector<f32, 3>,
		up: &Vector<f32, 3>,
	) -> Quaternion {
		let dot = Vector::dot(prev_forward, next_forward);
		if f32::abs(dot + 1.0) <= f32::EPSILON {
			return Quaternion::from_axis_angle(*up, 180_f32.to_radians());
		} else if f32::abs(dot - 1.0) <= f32::EPSILON {
			return Quaternion::identity();
		} else {
			let angle = f32::acos(dot);
			let axis = Vector::cross(&prev_forward, &next_forward);
			Quaternion::from_axis_angle(axis, angle)
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	fn right_angle() -> (f32, f32, f32) {
		let angle = f32::to_radians(90.0);
		let half_angle = angle / 2.0;
		(angle, f32::sin(half_angle), f32::cos(half_angle))
	}

	#[test]
	fn from_x_axis_right_angle() {
		let (angle, half_sin, half_cos) = right_angle();
		assert_vec_approx!(
			Quaternion::from_axis_angle(Vector::new([1.0, 0.0, 0.0]), angle),
			Vector::new([half_sin, 0.0, 0.0, half_cos]),
			1.0e-6,
			std::f32::consts::PI
		);
	}

	#[test]
	fn from_y_axis_right_angle() {
		let (angle, half_sin, half_cos) = right_angle();
		assert_vec_approx!(
			Quaternion::from_axis_angle(Vector::new([0.0, 1.0, 0.0]), angle),
			Vector::new([0.0, half_sin, 0.0, half_cos]),
			1.0e-6,
			std::f32::consts::PI
		);
	}

	#[test]
	fn from_z_axis_right_angle() {
		let (angle, half_sin, half_cos) = right_angle();
		assert_vec_approx!(
			Quaternion::from_axis_angle(Vector::new([0.0, 0.0, 1.0]), angle),
			Vector::new([0.0, 0.0, half_sin, half_cos]),
			1.0e-6,
			std::f32::consts::PI
		);
	}

	#[test]
	fn to_euler_x_axis() {
		let (angle, half_sin, half_cos) = right_angle();
		assert_vec_approx!(
			Quaternion::new([half_sin, 0.0, 0.0, half_cos]).to_euler(),
			Vector::new([angle, 0.0, 0.0]),
			1.0e-6,
			std::f32::consts::PI
		);
	}

	#[test]
	fn to_euler_y_axis() {
		let (angle, half_sin, half_cos) = right_angle();
		assert_vec_approx!(
			Quaternion::new([0.0, half_sin, 0.0, half_cos]).to_euler(),
			Vector::new([0.0, angle, 0.0]),
			1.0e-6,
			std::f32::consts::PI
		);
	}

	#[test]
	fn to_euler_z_axis() {
		let (angle, half_sin, half_cos) = right_angle();
		assert_vec_approx!(
			Quaternion::new([0.0, 0.0, half_sin, half_cos]).to_euler(),
			Vector::new([0.0, 0.0, angle]),
			1.0e-3,
			std::f32::consts::PI
		);
	}

	#[test]
	fn conjugate() {
		assert_eq!(
			Quaternion::new([1.0, 2.0, 3.0, 1.5]).conjugate(),
			Quaternion::new([-1.0, -2.0, -3.0, 1.5])
		);
		assert_eq!(
			Quaternion::new([-3.0, -2.0, -3.0, 1.0]).conjugate(),
			Quaternion::new([3.0, 2.0, 3.0, 1.0])
		);
	}

	#[test]
	fn inverse() {
		let quat = [-2.0, -3.0, -4.0, 1.0];
		let quat_dot: f32 = 30.0;
		let quat_mag = quat_dot.sqrt();
		let result = [
			2.0 / quat_mag,
			3.0 / quat_mag,
			4.0 / quat_mag,
			1.0 / quat_mag,
		];
		assert_vec_approx!(Quaternion::new(quat).inverse(), Quaternion::new(result));
	}

	#[test]
	fn concat_x_with_y() {
		let (angle, _, _) = right_angle();
		let quat_x = Quaternion::from_axis_angle(Vector::new([1.0, 0.0, 0.0]), angle);
		let quat_y = Quaternion::from_axis_angle(Vector::new([0.0, 1.0, 0.0]), angle);
		assert_vec_approx!(
			Quaternion::concat(&quat_x, &quat_y),
			Quaternion::new([0.5; 4])
		);
	}

	#[test]
	fn concat_x_with_z() {
		let (angle, _, _) = right_angle();
		let quat_x = Quaternion::from_axis_angle(Vector::new([1.0, 0.0, 0.0]), angle);
		let quat_z = Quaternion::from_axis_angle(Vector::new([0.0, 0.0, 1.0]), angle);
		assert_vec_approx!(
			Quaternion::concat(&quat_x, &quat_z),
			Quaternion::new([0.5, -0.5, 0.5, 0.5])
		);
	}

	#[test]
	fn rotate_x_to_z() {
		let (angle, _, _) = right_angle();
		let rot_90_y = Quaternion::from_axis_angle(Vector::new([0.0, 1.0, 0.0]), angle);
		assert_vec_approx!(
			rot_90_y.rotate(&Vector::new([1.0, 0.0, 0.0])),
			Vector::new([0.0, 0.0, -1.0])
		);
	}

	#[test]
	fn rotate_y_to_z() {
		let (angle, _, _) = right_angle();
		let rot_90_x = Quaternion::from_axis_angle(Vector::new([1.0, 0.0, 0.0]), angle);
		assert_vec_approx!(
			rot_90_x.rotate(&Vector::new([0.0, 1.0, 0.0])),
			Vector::new([0.0, 0.0, 1.0])
		);
	}

	#[test]
	fn rotate_z_to_y() {
		let (angle, _, _) = right_angle();
		let rot_90_x = Quaternion::from_axis_angle(Vector::new([1.0, 0.0, 0.0]), angle);
		assert_vec_approx!(
			rot_90_x.rotate(&Vector::new([0.0, 0.0, 1.0])),
			Vector::new([0.0, -1.0, 0.0])
		);
	}
}
