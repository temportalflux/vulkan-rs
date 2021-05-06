use crate::{Quaternion, Vector};
use std::iter::Sum;
use std::ops::*;

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub struct Matrix<T, const WIDTH: usize, const HEIGHT: usize>
where
	T: Sized,
{
	columns: [[T; HEIGHT]; WIDTH],
}

impl<T, const WIDTH: usize, const HEIGHT: usize> Default for Matrix<T, WIDTH, HEIGHT>
where
	T: Sized + Default + Copy,
{
	fn default() -> Matrix<T, WIDTH, HEIGHT> {
		Matrix {
			columns: [[T::default(); HEIGHT]; WIDTH],
		}
	}
}

impl<T, const WIDTH: usize, const HEIGHT: usize> Matrix<T, WIDTH, HEIGHT>
where
	T: Sized + Default + Copy,
{
	pub fn new(m: [[T; WIDTH]; HEIGHT]) -> Matrix<T, WIDTH, HEIGHT> {
		let mut matrix = Self::default();
		for row in 0..HEIGHT {
			for col in 0..WIDTH {
				matrix[col][row] = m[row][col];
			}
		}
		matrix
	}

	fn identity_internal(identity: T) -> Self {
		let mut matrix = Matrix::default();
		for i in 0..WIDTH {
			if i < HEIGHT {
				matrix[i][i] = identity;
			}
		}
		matrix
	}
}

impl<T, const WIDTH: usize, const HEIGHT: usize> Index<usize> for Matrix<T, WIDTH, HEIGHT> {
	type Output = [T; HEIGHT];
	fn index(&self, col_index: usize) -> &Self::Output {
		&self.columns[col_index]
	}
}

impl<T, const WIDTH: usize, const HEIGHT: usize> IndexMut<usize> for Matrix<T, WIDTH, HEIGHT> {
	fn index_mut(&mut self, col_index: usize) -> &mut Self::Output {
		&mut self.columns[col_index]
	}
}

impl<T, const WIDTH: usize, const HEIGHT: usize> Matrix<T, WIDTH, HEIGHT>
where
	T: Sized + Default + Copy,
{
	pub fn with_row(mut self, index: usize, row: &[T; WIDTH]) -> Self {
		for i in 0..WIDTH {
			self.columns[i][index] = row[i];
		}
		self
	}

	pub fn set_row(&mut self, index: usize, row: &[T; WIDTH]) {
		for i in 0..WIDTH {
			self.columns[i][index] = row[i];
		}
	}

	pub fn column(&self, index: usize) -> &[T; HEIGHT] {
		&self.columns[index]
	}

	pub fn column_vec(&self, index: usize) -> Vector<T, HEIGHT> {
		Vector::new(*self.column(index))
	}

	pub fn set_column(&mut self, index: usize, vec: &[T; HEIGHT]) {
		for i in 0..HEIGHT {
			self.columns[index][i] = vec[i];
		}
	}

	pub fn set_column_vec(&mut self, index: usize, vec: Vector<T, HEIGHT>) {
		self.set_column(index, vec.data());
	}

	pub fn set_row_vec(&mut self, index: usize, vec: Vector<T, WIDTH>) {
		self.set_row(index, vec.data());
	}
}

impl<T, U, const M: usize, const N: usize, const P: usize> Mul<Matrix<T, N, P>> for Matrix<T, M, N>
where
	T: Sized + Default + Copy + Mul<Output = U>,
	U: Sized + Default + Copy + Sum,
{
	type Output = Matrix<U, M, P>;
	fn mul(self, rhs: Matrix<T, N, P>) -> Self::Output {
		let mut matrix = Self::Output::default();
		for m in 0..M {
			for p in 0..P {
				matrix.columns[m][p] = (0..N).map(|n| self.columns[m][n] * rhs.columns[n][p]).sum();
			}
		}
		matrix
	}
}

impl<T, const S: usize> MulAssign<Matrix<T, S, S>> for Matrix<T, S, S>
where
	T: Sized + Default + Copy + Mul<Output = T> + Sum,
{
	fn mul_assign(&mut self, rhs: Matrix<T, S, S>) {
		*self = Self::mul(*self, rhs);
	}
}

#[cfg(test)]
mod multiplication {
	use super::*;

	#[test]
	fn mul_4x2_2x4_3x4() {
		let m1: Matrix<u32, 3, 2> = Matrix::new([[8, 0, 3], [5, 4, 2]]);
		let m2: Matrix<u32, 2, 4> = Matrix::new([[1, 2], [3, 5], [4, 1], [5, 0]]);
		let mr: Matrix<u32, 3, 4> =
			Matrix::new([[18, 8, 7], [49, 20, 19], [37, 4, 14], [40, 0, 15]]);
		assert_eq!(m1 * m2, mr);
	}

	#[test]
	fn mulassign_4x4() {
		let m1: Matrix<u32, 4, 4> =
			Matrix::new([[1, 0, 1, 2], [3, 1, 4, 5], [0, 1, 1, 2], [3, 4, 5, 1]]);
		let mut m2: Matrix<u32, 4, 4> =
			Matrix::new([[0, 1, 2, 3], [4, 5, 6, 7], [8, 7, 6, 5], [4, 3, 2, 1]]);
		let mr: Matrix<u32, 4, 4> = Matrix::new([
			[16, 14, 12, 10],
			[56, 51, 46, 41],
			[20, 18, 16, 14],
			[60, 61, 62, 63],
		]);
		m2 *= m1;
		assert_eq!(m2, mr);
	}
}

impl<T> From<Vector<T, 3>> for Matrix<T, 1, 4>
where
	T: Sized + Default + Copy + From<i32>,
{
	fn from(vec: Vector<T, 3>) -> Self {
		Matrix::new([[vec.x()], [vec.y()], [vec.z()], [1_i32.into()]])
	}
}

impl<T> Matrix<T, 4, 4>
where
	T: Sized + Default + Copy + MulAssign + AddAssign,
	Self: Identity,
{
	pub fn translate(v: Vector<T, 3>) -> Self {
		let identity = Self::identity();
		let mut matrix = identity.clone();
		let column = (identity.column_vec(0) * v[0])
			+ (identity.column_vec(1) * v[1])
			+ (identity.column_vec(2) * v[2])
			+ identity.column_vec(3);
		matrix.set_column_vec(3, column);
		matrix
	}

	pub fn scale(v: Vector<T, 3>) -> Self {
		let identity = Self::identity();
		let mut matrix = identity.clone();
		matrix.set_column_vec(0, identity.column_vec(0) * v[0]);
		matrix.set_column_vec(1, identity.column_vec(1) * v[1]);
		matrix.set_column_vec(2, identity.column_vec(2) * v[2]);
		matrix.set_column_vec(3, identity.column_vec(3));
		matrix
	}
}

impl From<Quaternion> for Matrix<f32, 4, 4> {
	fn from(quat: Quaternion) -> Self {
		let mut matrix = Self::identity();

		let qxx = quat.x() * quat.x();
		let qyy = quat.y() * quat.y();
		let qzz = quat.z() * quat.z();
		let qxz = quat.x() * quat.z();
		let qxy = quat.x() * quat.y();
		let qyz = quat.y() * quat.z();
		let qwx = quat.w() * quat.x();
		let qwy = quat.w() * quat.y();
		let qwz = quat.w() * quat.z();

		matrix[0][0] = 1.0 - 2.0 * (qyy + qzz);
		matrix[0][1] = 2.0 * (qxy + qwz);
		matrix[0][2] = 2.0 * (qxz - qwy);

		matrix[1][0] = 2.0 * (qxy - qwz);
		matrix[1][1] = 1.0 - 2.0 * (qxx + qzz);
		matrix[1][2] = 2.0 * (qyz + qwx);

		matrix[2][0] = 2.0 * (qxz + qwy);
		matrix[2][1] = 2.0 * (qyz - qwx);
		matrix[2][2] = 1.0 - 2.0 * (qxx + qyy);

		matrix
	}
}

impl Matrix<f32, 4, 4> {
	pub fn model_matrix(
		translation: Vector<f32, 3>,
		rotation: Quaternion,
		scale: Vector<f32, 3>,
	) -> Self {
		let mut matrix = Self::translate(translation);
		matrix *= rotation.into();
		matrix *= Self::scale(scale);
		matrix
	}

	pub fn look_at(eye_pos: Vector<f32, 3>, center: Vector<f32, 3>, up: Vector<f32, 3>) -> Self {
		let f = (center - eye_pos).normal();
		let s = Vector::cross(&f, &up).normal();
		let u = Vector::cross(&s, &f);

		let mut matrix = Self::identity();
		matrix.set_row_vec(0, s.subvec::<4>(None));
		matrix.set_row_vec(1, u.subvec::<4>(None));
		matrix.set_row_vec(2, -f.subvec::<4>(None));
		matrix[3][0] = -s.dot(&eye_pos);
		matrix[3][1] = -u.dot(&eye_pos);
		matrix[3][2] = f.dot(&eye_pos);
		matrix
	}

	pub fn perspective_right_hand_depth_zero_to_one(
		y_fov: f32,
		aspect_ratio: f32,
		near_plane: f32,
		far_plane: f32,
	) -> Self {
		/* Based on GLM
		template<typename T>
		GLM_FUNC_QUALIFIER mat<4, 4, T, defaultp> perspectiveRH_ZO(T fovy, T aspect, T zNear, T zFar)
		{
			assert(abs(aspect - std::numeric_limits<T>::epsilon()) > static_cast<T>(0));
			T const tanHalfFovy = tan(fovy / static_cast<T>(2));
			mat<4, 4, T, defaultp> Result(static_cast<T>(0));
			Result[0][0] = static_cast<T>(1) / (aspect * tanHalfFovy);
			Result[1][1] = static_cast<T>(1) / (tanHalfFovy);
			Result[2][2] = zFar / (zNear - zFar);
			Result[2][3] = - static_cast<T>(1);
			Result[3][2] = -(zFar * zNear) / (zFar - zNear);
			return Result;
		}
		*/

		// A tweet about handedness in different engines: https://twitter.com/FreyaHolmer/status/644881436982575104

		assert!(f32::abs(aspect_ratio - f32::EPSILON) > 0.0);
		let tan_half_fov_y = f32::tan(y_fov / 2.0);
		let mut perspective = Self::default();
		perspective[0][0] = 1.0 / (aspect_ratio * tan_half_fov_y);
		perspective[1][1] = -1.0 / (tan_half_fov_y);
		perspective[2][2] = far_plane / (near_plane - far_plane);
		perspective[2][3] = -1.0;
		perspective[3][2] = -(far_plane * near_plane) / (far_plane - near_plane);
		return perspective;
	}

	pub fn orthographic(
		left: f32,
		right: f32,
		bottom: f32,
		top: f32,
		z_near: f32,
		z_far: f32,
	) -> Self {
		let mut matrix = Self::identity();
		matrix[0][0] = 2.0 / (right - left);
		matrix[1][1] = 2.0 / (top - bottom);
		matrix[2][2] = 1.0 / (z_far - z_near);
		matrix[3][0] = -(right + left) / (right - left);
		matrix[3][1] = -(top + bottom) / (top - bottom);
		matrix[3][2] = -z_near / (z_far - z_near);
		matrix
	}
}

#[cfg(test)]
mod tridimensional_ops {
	use super::*;

	#[test]
	fn translate() {
		assert_eq!(
			Matrix::translate(crate::vector![1.0, 2.0, 3.0]),
			Matrix::new([
				[1.0, 0.0, 0.0, 1.0],
				[0.0, 1.0, 0.0, 2.0],
				[0.0, 0.0, 1.0, 3.0],
				[0.0, 0.0, 0.0, 1.0]
			])
		);
	}

	#[test]
	fn scale() {
		assert_eq!(
			Matrix::scale(crate::vector![4.0, 2.0, 3.0]),
			Matrix::new([
				[4.0, 0.0, 0.0, 0.0],
				[0.0, 2.0, 0.0, 0.0],
				[0.0, 0.0, 3.0, 0.0],
				[0.0, 0.0, 0.0, 1.0]
			])
		);
	}
}

pub trait Identity {
	fn identity() -> Self;
}

impl<const WIDTH: usize, const HEIGHT: usize> Identity for Matrix<f32, WIDTH, HEIGHT> {
	fn identity() -> Self {
		Self::identity_internal(1.0)
	}
}

impl<const WIDTH: usize, const HEIGHT: usize> Identity for Matrix<f64, WIDTH, HEIGHT> {
	fn identity() -> Self {
		Self::identity_internal(1.0)
	}
}

impl<const WIDTH: usize, const HEIGHT: usize> Identity for Matrix<i32, WIDTH, HEIGHT> {
	fn identity() -> Self {
		Self::identity_internal(1)
	}
}

impl<const WIDTH: usize, const HEIGHT: usize> Identity for Matrix<u32, WIDTH, HEIGHT> {
	fn identity() -> Self {
		Self::identity_internal(1)
	}
}