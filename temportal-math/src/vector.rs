use super::product::*;
use std::iter::Sum;
use std::ops::*;

#[derive(Debug, Copy, Clone)]
pub struct Vector<T, const N: usize> {
	data: [T; N],
}

// #region Initialization

impl<T, const N: usize> Vector<T, N> {
	pub fn new(data: [T; N]) -> Vector<T, N> {
		Vector { data }
	}
}

impl<T, const N: usize> Vector<T, N>
where
	T: Sized + Copy,
{
	pub fn filled(fill: T) -> Vector<T, N> {
		Vector::new([fill; N])
	}
}

#[cfg(test)]
mod init_tests {
	use super::*;

	#[test]
	fn new() {
		assert_eq!(Vector::new([1.0; 5]).data, [1.0; 5]);
	}

	#[test]
	fn filled() {
		let v1: Vector<f64, 3> = Vector::filled(1.0);
		assert_eq!(v1.data, [1.0; 3]);
	}
}

// #endregion

// #region Partial Vector

impl<T, const N: usize> Vector<T, N>
where
	T: Default + Copy,
{
	pub fn partial(iter: &mut impl Iterator<Item = T>, offset: Option<usize>) -> Vector<T, N> {
		let mut v = Vector::filled(T::default());
		let mut value_iter = iter.skip(match offset {
			Some(i) => i,
			None => 0,
		});
		for i in 0..N {
			match value_iter.next() {
				Some(value) => v.data[i] = value,
				None => break,
			}
		}
		v
	}

	pub fn subvec<const N2: usize>(&self, offset: Option<usize>) -> Vector<T, N2> {
		Vector::partial(&mut self.data.iter().cloned(), offset)
	}
}

#[cfg(test)]
mod partial {
	use super::*;

	#[test]
	fn one_to_two() {
		let one = Vector::new([5]);
		let two: Vector<i32, 2> = one.subvec(None);
		assert_eq!(two.data, [5, 0]);
	}

	#[test]
	fn two_to_three() {
		let two = Vector::new([2, 2]);
		let three: Vector<i32, 3> = two.subvec(None);
		assert_eq!(three.data, [2, 2, 0]);
	}

	#[test]
	fn four_to_three_no_offset() {
		let four = Vector::new([1, 2, 3, 4]);
		let three: Vector<i32, 3> = four.subvec(None);
		assert_eq!(three.data, [1, 2, 3]);
	}

	#[test]
	fn five_to_two_offset_2() {
		let five = Vector::new([1, 2, 3, 4, 5]);
		let two: Vector<i32, 2> = five.subvec(Some(2));
		assert_eq!(two.data, [3, 4]);
	}

	#[test]
	fn partial_array_to_vec_no_offset() {
		let arr = [6, 5, 4, 3, 2];
		let v: Vector<i32, 5> = Vector::partial(&mut arr.iter().cloned(), None);
		assert_eq!(v.data, arr);
	}

	#[test]
	fn partial_array_to_vec_offset_3() {
		let arr = [60, 50, 40, 30, 20, 10];
		let v: Vector<i32, 4> = Vector::partial(&mut arr.iter().cloned(), Some(3));
		assert_eq!(v.data, [30, 20, 10, 0]);
	}
}

// #endregion

// #region Equality

impl<T, const N: usize> PartialEq for Vector<T, N>
where
	T: PartialEq,
{
	fn eq(&self, other: &Self) -> bool {
		for i in 0..N {
			if self.data[i] != other.data[i] {
				return false;
			}
		}
		true
	}
}
impl<T, const N: usize> Eq for Vector<T, N> where T: Eq {}

#[cfg(test)]
mod equality {
	use super::*;

	#[test]
	fn origin_equal() {
		let origin: Vector<f64, 3> = Vector::filled(0.0);
		assert!(origin == origin);
	}

	#[test]
	fn rand_equal() {
		let data = [53, 73, 12, 98, 27, 18, 26];
		let origin = Vector::new(data);
		assert_eq!(data, origin.data);
	}
}

// #endregion

// #region Indexing

impl<T, const N: usize> Index<usize> for Vector<T, N> {
	type Output = T;
	fn index(&self, idx: usize) -> &Self::Output {
		&self.data[idx]
	}
}

impl<T, const N: usize> IndexMut<usize> for Vector<T, N> {
	fn index_mut(&mut self, idx: usize) -> &mut Self::Output {
		&mut self.data[idx]
	}
}

impl<T, const N: usize> Vector<T, N>
where
	T: Copy,
{
	pub fn x(&self) -> &T {
		&self.data[0]
	}
	pub fn x_mut(&mut self, val: T) {
		self.data[0] = val;
	}

	pub fn y(&self) -> &T {
		&self.data[1]
	}
	pub fn y_mut(&mut self, val: T) {
		self.data[1] = val;
	}

	pub fn z(&self) -> &T {
		&self.data[2]
	}
	pub fn z_mut(&mut self, val: T) {
		self.data[2] = val;
	}

	pub fn w(&self) -> &T {
		&self.data[3]
	}
	pub fn w_mut(&mut self, val: T) {
		self.data[3] = val;
	}
}

#[cfg(test)]
mod indexing {
	use super::*;

	#[test]
	fn index_op_works() {
		let vec = Vector::new([0, 1, 2, 3, 4]);
		assert_eq!(vec[0], 0);
		assert_eq!(vec[1], 1);
		assert_eq!(vec[2], 2);
		assert_eq!(vec[3], 3);
		assert_eq!(vec[4], 4);
	}

	#[test]
	fn index_mut_op_works() {
		let mut vec = Vector::new([0; 5]);
		vec[0] = 0;
		vec[1] = 1;
		vec[2] = 2;
		vec[3] = 3;
		vec[4] = 4;
		assert_eq!(vec[0], 0);
		assert_eq!(vec[1], 1);
		assert_eq!(vec[2], 2);
		assert_eq!(vec[3], 3);
		assert_eq!(vec[4], 4);
	}

	#[test]
	fn accessors_work() {
		let vec = Vector::new([0, 1, 2, 3]);
		assert_eq!(vec.x(), &0);
		assert_eq!(vec.y(), &1);
		assert_eq!(vec.z(), &2);
		assert_eq!(vec.w(), &3);
	}

	#[test]
	fn accessor_mut_works() {
		let mut vec = Vector::new([5; 4]);
		vec.x_mut(0);
		vec.y_mut(1);
		vec.z_mut(2);
		vec.w_mut(3);
		assert_eq!(vec.x(), &0);
		assert_eq!(vec.y(), &1);
		assert_eq!(vec.z(), &2);
		assert_eq!(vec.w(), &3);
	}
}

// #endregion

// #region Conversions

impl<T, const N: usize> Vector<T, N> {
	pub fn from<U>(other: Vector<U, N>) -> Self
	where
		T: Default + Copy,
		U: Into<T> + Copy,
	{
		let mut vret: Vector<T, N> = Vector::filled(T::default());
		for i in 0..3 {
			vret.data[i] = other.data[i].into()
		}
		vret
	}
}

#[cfg(test)]
mod conversions {
	use super::*;

	#[test]
	fn i32_to_f64() {
		let input: Vector<i32, 3> = Vector::new([1, 2, 3]);
		let calculated: Vector<f64, 3> = Vector::from(input);
		let expected: Vector<f64, 3> = Vector::new([1.0, 2.0, 3.0]);
		assert_eq!(calculated, expected);
	}
}

// #endregion

// #region Operations

impl<T, const N: usize> AddAssign<Vector<T, N>> for Vector<T, N>
where
	T: Default + Copy + AddAssign,
{
	fn add_assign(&mut self, other: Self) {
		for i in 0..N {
			self.data[i] += other.data[i];
		}
	}
}

impl<T, const N: usize> Add<Vector<T, N>> for Vector<T, N>
where
	T: Default + Copy + AddAssign,
{
	type Output = Vector<T, N>;
	fn add(self, other: Self) -> Self::Output {
		let mut vret = self.clone();
		vret.add_assign(other);
		vret
	}
}

impl<T, const N: usize> SubAssign<Vector<T, N>> for Vector<T, N>
where
	T: Default + Copy + SubAssign,
{
	fn sub_assign(&mut self, other: Self) {
		for i in 0..N {
			self.data[i] -= other.data[i];
		}
	}
}

impl<T, const N: usize> Sub<Vector<T, N>> for Vector<T, N>
where
	T: Default + Copy + SubAssign,
{
	type Output = Vector<T, N>;
	fn sub(self, other: Self) -> Self::Output {
		let mut vret = self.clone();
		vret.sub_assign(other);
		vret
	}
}

impl<T, const N: usize> MulAssign<Vector<T, N>> for Vector<T, N>
where
	T: Default + Copy + MulAssign,
{
	fn mul_assign(&mut self, other: Self) {
		for i in 0..N {
			self.data[i] *= other.data[i];
		}
	}
}

impl<T, const N: usize> Mul<Vector<T, N>> for Vector<T, N>
where
	T: Default + Copy + MulAssign,
{
	type Output = Vector<T, N>;
	fn mul(self, other: Self) -> Self::Output {
		let mut vret = self.clone();
		vret.mul_assign(other);
		vret
	}
}

impl<T, const N: usize> DivAssign<Vector<T, N>> for Vector<T, N>
where
	T: Default + Copy + DivAssign,
{
	fn div_assign(&mut self, other: Self) {
		for i in 0..N {
			self.data[i] /= other.data[i];
		}
	}
}

impl<T, const N: usize> Div<Vector<T, N>> for Vector<T, N>
where
	T: Default + Copy + DivAssign,
{
	type Output = Vector<T, N>;
	fn div(self, other: Self) -> Self::Output {
		let mut vret = self.clone();
		vret.div_assign(other);
		vret
	}
}

impl<T, const N: usize> MulAssign<T> for Vector<T, N>
where
	T: Default + Copy + MulAssign,
{
	fn mul_assign(&mut self, other: T) {
		for i in 0..N {
			self.data[i] *= other;
		}
	}
}

impl<T, const N: usize> Mul<T> for Vector<T, N>
where
	T: Default + Copy + MulAssign,
{
	type Output = Vector<T, N>;
	fn mul(self, other: T) -> Self::Output {
		let mut vret = self.clone();
		vret.mul_assign(other);
		vret
	}
}

impl<T, const N: usize> DivAssign<T> for Vector<T, N>
where
	T: Default + Copy + DivAssign,
{
	fn div_assign(&mut self, other: T) {
		for i in 0..N {
			self.data[i] /= other;
		}
	}
}

impl<T, const N: usize> Div<T> for Vector<T, N>
where
	T: Default + Copy + DivAssign,
{
	type Output = Vector<T, N>;
	fn div(self, other: T) -> Self::Output {
		let mut vret = self.clone();
		vret.div_assign(other);
		vret
	}
}

impl<T, const N: usize> RemAssign<T> for Vector<T, N>
where
	T: RemAssign + Copy,
{
	fn rem_assign(&mut self, modulus: T) {
		for i in 0..N {
			self.data[i] %= modulus;
		}
	}
}

impl<T, const N: usize> Rem<T> for Vector<T, N>
where
	T: RemAssign + Default + Copy,
{
	type Output = Self;

	fn rem(self, modulus: T) -> Self::Output {
		let mut vret = self.clone();
		vret.rem_assign(modulus);
		vret
	}
}

impl<T, const N: usize> Vector<T, N>
where
	T: Default + Copy + AddAssign + SubAssign + PartialOrd,
{
	pub fn mod_reduce(&mut self, step: T) -> Self {
		let mut removed: Self = Vector::filled(T::default());
		for i in 0..N {
			while self.data[i] >= step {
				removed.data[i] += step;
				self.data[i] -= step;
			}
			while self.data[i] < T::default() {
				removed.data[i] -= step;
				self.data[i] += step;
			}
		}
		removed
	}
}

impl<T, const N: usize> Neg for Vector<T, N>
where
	T: Default + Copy + Neg<Output = T>,
{
	type Output = Self;
	fn neg(self) -> Self::Output {
		let mut vret = self.clone();
		for i in 0..N {
			vret.data[i] = -self.data[i];
		}
		vret
	}
}

#[cfg(test)]
mod operations {
	use super::*;

	#[test]
	fn add_assign_vec() {
		let mut v1 = Vector::new([1, 2, 3]);
		let v2 = Vector::new([3, 2, 1]);
		v1 += v2;
		assert_eq!(v1.data, [4, 4, 4]);
	}

	#[test]
	fn add_vec() {
		let v1 = Vector::new([1, 2, 3]);
		let v2 = Vector::new([3, 2, 1]);
		let v3 = v1 + v2;
		assert_eq!(v3.data, [4, 4, 4]);
	}

	#[test]
	fn sub_assign_vec() {
		let mut v1 = Vector::new([1, 2, 3]);
		let v2 = Vector::new([3, 2, 1]);
		v1 -= v2;
		assert_eq!(v1.data, [-2, 0, 2]);
	}

	#[test]
	fn sub_vec() {
		let v1 = Vector::new([1, 2, 3]);
		let v2 = Vector::new([3, 2, 1]);
		let v3 = v1 - v2;
		assert_eq!(v3.data, [-2, 0, 2]);
	}

	#[test]
	fn mul_assign_vec() {
		let mut v1 = Vector::new([1, 2, 3]);
		let v2 = Vector::new([3, 2, 1]);
		v1 *= v2;
		assert_eq!(v1.data, [3, 4, 3]);
	}

	#[test]
	fn mul_vec() {
		let v1 = Vector::new([1, 2, 3]);
		let v2 = Vector::new([3, 2, 1]);
		let v3 = v1 * v2;
		assert_eq!(v3.data, [3, 4, 3]);
	}

	#[test]
	fn div_assign_vec() {
		let mut v1 = Vector::new([4, 9, 16]);
		let v2 = Vector::new([2, 3, 4]);
		v1 /= v2;
		assert_eq!(v1.data, [2, 3, 4]);
	}

	#[test]
	fn div_vec() {
		let v1 = Vector::new([6, 12, 14]);
		let v2 = Vector::new([3, 4, 7]);
		let v3 = v1 / v2;
		assert_eq!(v3.data, [2, 3, 2]);
	}

	#[test]
	fn mul_assign_scalar() {
		let mut v1 = Vector::new([1, 2, 3]);
		v1 *= 2;
		assert_eq!(v1.data, [2, 4, 6]);
	}

	#[test]
	fn mul_sclar() {
		let v1 = Vector::new([1, 2, 3]);
		let v3 = v1 * 3;
		assert_eq!(v3.data, [3, 6, 9]);
	}

	#[test]
	fn div_assign_scalar() {
		let mut v1 = Vector::new([4, 8, 24]);
		v1 /= 2;
		assert_eq!(v1.data, [2, 4, 12]);
	}

	#[test]
	fn div_scalar() {
		let v1 = Vector::new([6, 12, 15]);
		let v3 = v1 / 3;
		assert_eq!(v3.data, [2, 4, 5]);
	}

	#[test]
	fn remainder_assign_scalar() {
		let mut v1 = Vector::new([4, 5, 10]);
		v1 %= 4;
		assert_eq!(v1.data, [0, 1, 2]);
	}

	#[test]
	fn remainder_scalar() {
		let v1 = Vector::new([4, 13, 50]);
		let v3 = v1 % 5;
		assert_eq!(v3.data, [4, 3, 0]);
	}

	#[test]
	fn mod_reduce() {
		let mut v1 = Vector::new([4, 18, -7, 5]);
		let v2 = v1.mod_reduce(5);
		assert_eq!(v1.data, [4, 3, 3, 0]);
		assert_eq!(v2.data, [0, 15, -10, 5]);
	}
	
	#[test]
	fn negate() {
		let v1 = Vector::new([1, 7, 3]);
		let v3 = -v1;
		assert_eq!(v3.data, [-1, -7, -3]);
	}
}

// #endregion

// #region Dot Product

impl<T, const N: usize> DotProduct<Vector<T, N>> for Vector<T, N>
where
	T: Sized + Copy + Mul<Output = T> + Add<Output = T> + Sum,
{
	type Output = <<T as Mul>::Output as Add>::Output;
	fn dot(&self, right: &Vector<T, N>) -> Self::Output {
		self.data
			.iter()
			.zip(right.data.iter())
			.map(|lr| (*lr.0) * (*lr.1))
			.sum()
	}
}

#[cfg(test)]
mod dot_tests {
	use super::*;

	#[test]
	fn origin_dot_origin_is_zero() {
		let origin: Vector<f64, 3> = Vector::filled(0.0);
		assert_eq!(origin.dot(&origin), 0.0);
	}

	#[test]
	fn identity_dot_origin_is_zero() {
		let origin: Vector<f64, 3> = Vector::filled(0.0);
		let identity: Vector<f64, 3> = Vector::filled(1.0);
		assert_eq!(identity.dot(&origin), 0.0);
	}

	#[test]
	fn one_dot_one() {
		let identity: Vector<f64, 3> = Vector::filled(1.0);
		assert_eq!(identity.dot(&identity), 3.0);
	}
}

// #endregion

// #region Cross Product

impl<T> CrossProduct for Vector<T, 3>
where
	T: Default + Sized + Copy + Mul<Output = T> + Add<Output = T> + Sub<Output = T>,
{
	fn cross(left: &Self, right: &Self) -> Self {
		let mut vret: Self = Vector::filled(T::default());
		vret[0] = left[1] * right[2] - left[2] * right[1];
		vret[1] = left[2] * right[0] - left[0] * right[2];
		vret[2] = left[0] * right[1] - left[1] * right[0];
		vret
	}
}

#[cfg(test)]
mod cross_tests {
	use super::*;

	#[test]
	fn x_axis_with_y_axis() {
		let z_axis = Vector::cross(
			&Vector::new([1, 0, 0]),
			&Vector::new([0, 1, 0])
		);
		assert_eq!(z_axis.data, [0, 0, 1]);
	}

}

// #endregion

// #region Properties

impl<T, const N: usize> Vector<T, N>
where
	T: Sized + Copy + Mul<Output = T> + Add<Output = T> + Sum + Into<f64>,
{
	pub fn magnitude_sq(&self) -> f64 {
		self.dot(&self).into()
	}

	pub fn magnitude(&self) -> f64 {
		self.magnitude_sq().sqrt()
	}
}

impl<const N: usize> Vector<f64, N> {
	pub fn normalize(&mut self) {
		*self /= self.magnitude()
	}

	pub fn normal(&self) -> Self {
		let mut vret = self.clone();
		vret.normalize();
		vret
	}
}

#[cfg(test)]
mod property_tests {
	use super::*;

	#[test]
	fn magnitude_sq() {
		assert_eq!(Vector::new([0, 0, 0]).magnitude_sq(), 0.0);
		assert_eq!(Vector::new([0, 0, 1]).magnitude_sq(), 1.0);
		assert_eq!(Vector::new([1, 2, 0]).magnitude_sq(), 5.0);
		assert_eq!(Vector::new([1, 1, 1]).magnitude_sq(), 3.0);
	}

	#[test]
	fn magnitude() {
		assert_eq!(Vector::new([0, 0, 0]).magnitude(), 0.0);
		assert_eq!(Vector::new([0, 0, 1]).magnitude(), 1.0);
		assert_eq!(Vector::new([0, 2, 0]).magnitude(), 2.0);
		assert_eq!(Vector::new([2, 1, 2]).magnitude(), 3.0);
	}

	#[test]
	fn normal() {
		assert_eq!(Vector::new([5.0, 0.0, 0.0]).normal().data, [1.0, 0.0, 0.0]);
		assert_eq!(Vector::new([0.0, 2.0, 0.0]).normal().data, [0.0, 1.0, 0.0]);
		assert_eq!(Vector::new([0.0, 0.0, 8.0]).normal().data, [0.0, 0.0, 1.0]);
		assert_eq!(Vector::new([2.0, 2.0, 1.0]).normal().data, [2.0 / 3.0, 2.0 / 3.0, 1.0 / 3.0]);
	}

	#[test]
	fn normalize() {
		let mut vec = Vector::new([5.0, 0.0, 0.0]);
		vec.normalize();
		assert_eq!(vec.data, [1.0, 0.0, 0.0]);

		let mut vec = Vector::new([0.0, 3.0, 0.0]);
		vec.normalize();
		assert_eq!(vec.data, [0.0, 1.0, 0.0]);

		let mut vec = Vector::new([0.0, 0.0, 8.0]);
		vec.normalize();
		assert_eq!(vec.data, [0.0, 0.0, 1.0]);

		let mut vec = Vector::new([2.0, 2.0, 1.0]);
		vec.normalize();
		assert_eq!(vec.data, [2.0 / 3.0, 2.0 / 3.0, 1.0 / 3.0]);
	}

}

// #endregion
