use super::product::*;
use std::iter::Sum;
use std::ops::*;

#[derive(Debug, Copy, Clone)]
pub struct Vector<T, const N: usize> {
	data: [T; N],
}

// ~~~~~ START: Initialization ~~~~~

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

impl<T, const N: usize> Vector<T, N>
where
	T: Default + Copy,
{
	pub fn from_slice(slice: &[T]) -> Vector<T, N> {
		let mut v = Vector::filled(T::default());
		for i in 0..std::cmp::min(slice.len(), N) {
			v.data[i] = slice[i];
		}
		v
	}

	pub fn subvec<const N2: usize>(&self, offset: Option<usize>) -> Vector<T, N2> {
		let first_index = match offset {
			Some(i) => i,
			None => 0,
		};
		Vector::from_slice(&self.data[first_index..N])
	}
}

#[cfg(test)]
mod init_tests {
	use super::*;

	#[test]
	fn vector_new() {
		assert_eq!(Vector::new([1.0; 5]).data, [1.0; 5]);
	}

	#[test]
	fn vector_filled() {
		let v1: Vector<f64, 3> = Vector::filled(1.0);
		assert_eq!(v1.data, [1.0; 3]);
	}
}

// ~~~~~~~ END: Initialization ~~~~~

// ~~~~~ START: Equality ~~~~~

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
mod vector_equality {
	use super::*;

	#[test]
	fn origin_equal() {
		let origin: Vector<f64, 3> = Vector::filled(0.0);
		assert!(origin == origin);
	}
}

// ~~~~~~~ END: Equality ~~~~~

// ~~~~~ START: Indexing ~~~~~

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

// ~~~~~~~ END: Indexing ~~~~~

// ~~~~~ START: Conversions ~~~~~

impl<const N: usize> Vector<f64, N> {
	pub fn from<U>(other: Vector<U, N>) -> Self
	where
		U: Into<f64> + Copy,
	{
		let mut vret: Vector<f64, N> = Vector::filled(0.0);
		for i in 0..3 {
			vret.data[i] = other.data[i].into()
		}
		vret
	}
}

#[cfg(test)]
mod vector_conversions {
	use super::*;

	#[test]
	fn i32_to_f64() {
		let vi: Vector<i32, 3> = Vector::new([1, 2, 3]);
		let vif: Vector<f64, 3> = Vector::from(vi);
		let vf: Vector<f64, 3> = Vector::new([1.0, 2.0, 3.0]);
		assert_eq!(vif, vf);
	}
}

// ~~~~~~~ END: Conversions ~~~~~

// ~~~~~ START: Ops ~~~~~

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
		let mut vret: Self::Output = Vector::filled(T::default());
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
		let mut vret: Self::Output = Vector::filled(T::default());
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
		let mut vret: Self::Output = Vector::filled(T::default());
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
		let mut vret: Self::Output = Vector::filled(T::default());
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
		let mut vret: Self::Output = Vector::filled(T::default());
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
		let mut vret: Self::Output = Vector::filled(T::default());
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
		let mut vret: Self::Output = Vector::filled(T::default());
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
		let mut vret: Self::Output = Vector::filled(T::default());
		for i in 0..N {
			vret.data[i] = -self.data[i];
		}
		vret
	}
}

// ~~~~~~~ END: Ops ~~~~~

// ~~~~~ START: Dot Product ~~~~~

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
mod vector_dot_tests {
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

// ~~~~~~~ END: Dot Product ~~~~~

// ~~~~~ START: Cross Product ~~~~~

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

// ~~~~~~~ END: Cross Product ~~~~~

// ~~~~~ START: Properties ~~~~~

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
		let magnitude = self.magnitude();
		for i in 0..N {
			self.data[i] = self.data[i] / magnitude;
		}
	}

	pub fn normalized(&self) -> Self {
		let mut vret = self.clone();
		vret.normalize();
		vret
	}
}

// ~~~~~~~ END: Properties ~~~~~
