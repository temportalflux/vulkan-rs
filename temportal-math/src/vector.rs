use std::iter::Sum;
use std::ops::*;

/// Linear Algebraic structure for vectors in multiple dimensions.
#[derive(Copy, Clone, Debug)]
pub struct Vector<T, const N: usize> {
	pub data: [T; N],
}

// #region Initialization

impl<T, const N: usize> Default for Vector<T, N>
where
	T: Default + Copy,
{
	fn default() -> Vector<T, N> {
		Vector::filled(T::default())
	}
}

impl<T, const N: usize> Vector<T, N> {
	/// Creates a new vector with the provided components.
	///
	/// # Examples
	/// ```
	/// use temportal_math::Vector;
	/// let vec = Vector::new([1.0, 2.0, 3.0]);
	/// assert!(vec[0] == 1.0 && vec[1] == 2.0 && vec[2] == 3.0);
	/// ```
	pub fn new(data: [T; N]) -> Vector<T, N> {
		Vector { data }
	}

	/// Returns the amount of dimensions of the vector (i.e. the `N` const-generic).
	///
	/// # Examples
	/// ```
	/// use temportal_math::Vector;
	/// assert_eq!(Vector::new([0.0; 2]).capacity(), 2);
	/// assert_eq!(Vector::new([0.0; 7]).capacity(), 7);
	/// ```
	pub fn capacity(&self) -> usize {
		N
	}

	pub fn from_vec<U>(other: Vector<U, N>) -> Vector<T, N>
	where
		U: Sized + Into<T> + Copy,
		T: Default + Copy,
	{
		let mut vret = Vector::new([T::default(); N]);
		for i in 0..N {
			vret[i] = other[i].into();
		}
		vret
	}
}

impl<T, const N: usize> Vector<T, N>
where
	T: Clone,
{
	/// Converts the mathemtical Vector into a collection Vec containing the n-dimensional data.
	///
	/// # Examples
	/// ```
	/// use temportal_math::Vector;
	/// let vec = Vector::new([1.0, 2.0, 3.0]);
	/// assert_eq!(vec.to_vec(), vec![1.0, 2.0, 3.0]);
	/// ```
	pub fn to_vec(&self) -> Vec<T> {
		self.data.to_vec()
	}
}

impl<T, const N: usize> Vector<T, N>
where
	T: Sized + Copy,
{
	/// Creates a vector with `N` dimensions, where each dimensional component has the value `fill`.
	///
	/// # Examples
	/// ```
	/// use temportal_math::Vector;
	/// let vec: Vector<f64, 4> = Vector::filled(12.63);
	/// assert_eq!(vec.to_vec(), vec![12.63; 4]);
	/// ```
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
	/// Creates a vector populated with some data from `iter`.
	/// If `offset` is provided, the `iter` will `iter.skip` with the provided offset before being read from.
	/// The returned vector will contain up to `N` components populated from `iter` (after skipping).
	/// If the length of `iter` after skipping is less than `N`, the remaining components of the vector will
	/// be `T::default()`.
	///
	/// # Examples
	/// Creating a 2-dimensional integer vector from an array of 2 integers.
	/// ```
	/// use temportal_math::Vector;
	/// let vec: Vector<i32, 2> = Vector::partial(&mut [1, 2].iter().cloned(), None);
	/// assert_eq!(vec.to_vec(), vec![1, 2]);
	/// ```
	/// Creating a 3-dimensional integer vector from an array of 5 integers, starting at the third value.
	/// ```
	/// use temportal_math::Vector;
	/// let vec: Vector<i32, 3> = Vector::partial(&mut [1, 2, 3, 4, 5].iter().cloned(), Some(2));
	/// assert_eq!(vec.to_vec(), vec![3, 4, 5]);
	/// ```
	/// Creating a 4-dimensional integer vector from an array of 3 integers, starting at the second value.
	/// ```
	/// use temportal_math::Vector;
	/// let vec: Vector<i32, 4> = Vector::partial(&mut [1, 2, 3].iter().cloned(), Some(1));
	/// assert_eq!(vec.to_vec(), vec![2, 3, 0, 0]);
	/// ```
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

	/// An ease-of-access wrapper for `partial` when working directly with Vectors.
	/// Creates a new vector with a different dimensional constraint which has some or all of the same values as `self`.
	///
	/// # Examples
	/// Create a 2-dimensional vector from the second-two dimensions of a 3-dimensional vector.
	/// ```
	/// use temportal_math::Vector;
	/// let vec3 = Vector::new([1, 2, 3]);
	/// let vec2 = vec3.subvec::<2>(Some(1));
	/// assert_eq!(vec2.to_vec(), vec![2, 3]);
	/// ```
	/// Create a 4-dimensional vector from a 3-dimensional vector.
	/// ```
	/// use temportal_math::Vector;
	/// let vec3 = Vector::new([1, 2, 3]);
	/// let vec4 = vec3.subvec::<4>(None);
	/// assert_eq!(vec4.to_vec(), vec![1, 2, 3, 0]);
	/// ```
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
	pub fn data(&self) -> &[T; N] {
		&self.data
	}

	/// Returns a copy of first-dimensional `x` component.
	/// Will panic if `N < 1`.
	///
	/// # Examples
	/// ```
	/// use temportal_math::Vector;
	/// let vec = Vector::new([1, 2, 3, 4]);
	/// assert_eq!(vec.x(), 1);
	/// ```
	pub fn x(&self) -> T {
		self.data[0]
	}

	/// Returns a mutatable reference to the first-dimensional `x` component.
	/// Will panic if `N < 1`.
	///
	/// # Examples
	/// ```
	/// use temportal_math::Vector;
	/// let mut vec = Vector::new([1, 2, 3, 4]);
	/// *vec.x_mut() = 5;
	/// assert_eq!(vec.to_vec(), vec![5, 2, 3, 4]);
	/// ```
	pub fn x_mut(&mut self) -> &mut T {
		&mut self.data[0]
	}

	pub fn with_x(mut self, x: T) -> Self {
		self.data[0] = x;
		self
	}

	/// Returns a copy of second-dimensional `y` component.
	/// Will panic if `N < 2`.
	///
	/// # Examples
	/// ```
	/// use temportal_math::Vector;
	/// let vec = Vector::new([1, 2, 3, 4]);
	/// assert_eq!(vec.y(), 2);
	/// ```
	pub fn y(&self) -> T {
		self.data[1]
	}

	/// Returns a mutatable reference to the second-dimensional `y` component.
	/// Will panic if `N < 2`.
	///
	/// # Examples
	/// ```
	/// use temportal_math::Vector;
	/// let mut vec = Vector::new([1, 2, 3, 4]);
	/// *vec.y_mut() = 5;
	/// assert_eq!(vec.to_vec(), vec![1, 5, 3, 4]);
	/// ```
	pub fn y_mut(&mut self) -> &mut T {
		&mut self.data[1]
	}

	pub fn with_y(mut self, y: T) -> Self {
		self.data[1] = y;
		self
	}

	/// Returns a copy of third-dimensional `z` component.
	/// Will panic if `N < 3`.
	///
	/// # Examples
	/// ```
	/// use temportal_math::Vector;
	/// let vec = Vector::new([1, 2, 3, 4]);
	/// assert_eq!(vec.z(), 3);
	/// ```
	pub fn z(&self) -> T {
		self.data[2]
	}

	/// Returns a mutatable reference to the third-dimensional `z` component.
	/// Will panic if `N < 3`.
	///
	/// # Examples
	/// ```
	/// use temportal_math::Vector;
	/// let mut vec = Vector::new([1, 2, 3, 4]);
	/// *vec.z_mut() = 5;
	/// assert_eq!(vec.to_vec(), vec![1, 2, 5, 4]);
	/// ```
	pub fn z_mut(&mut self) -> &mut T {
		&mut self.data[2]
	}

	pub fn with_z(mut self, z: T) -> Self {
		self.data[2] = z;
		self
	}

	/// Returns a copy of fourth-dimensional `w` component.
	/// Will panic if `N < 4`.
	///
	/// # Examples
	/// ```
	/// use temportal_math::Vector;
	/// let vec = Vector::new([1, 2, 3, 4]);
	/// assert_eq!(vec.w(), 4);
	/// ```
	pub fn w(&self) -> T {
		self.data[3]
	}

	/// Returns a mutatable reference to the fourth-dimensional `w` component.
	/// Will panic if `N < 4`.
	///
	/// # Examples
	/// ```
	/// use temportal_math::Vector;
	/// let mut vec = Vector::new([1, 2, 3, 4]);
	/// *vec.w_mut() = 5;
	/// assert_eq!(vec.to_vec(), vec![1, 2, 3, 5]);
	/// ```
	pub fn w_mut(&mut self) -> &mut T {
		&mut self.data[3]
	}

	pub fn with_w(mut self, w: T) -> Self {
		self.data[3] = w;
		self
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
		assert_eq!(vec.x(), 0);
		assert_eq!(vec.y(), 1);
		assert_eq!(vec.z(), 2);
		assert_eq!(vec.w(), 3);
	}

	#[test]
	fn accessor_mut_works() {
		let mut vec = Vector::new([5; 4]);
		*vec.x_mut() = 0;
		*vec.y_mut() = 1;
		*vec.z_mut() = 2;
		*vec.w_mut() = 3;
		assert_eq!(vec.x(), 0);
		assert_eq!(vec.y(), 1);
		assert_eq!(vec.z(), 2);
		assert_eq!(vec.w(), 3);
	}
}

// #endregion

// #region Conversions

impl<T, const N: usize> Vector<T, N> {
	/// Converts between vectors of different types with the same dimensional count.
	/// This is the opposite of `partial` and `subvec`.
	/// As long as `U` implements the trait `Into<T>`,
	/// `Vector<U, _>` can be converted into `Vector<T, _>`,
	/// as long as their `Vector::capacity` is the same.
	///
	/// # Examples
	/// ```
	/// use temportal_math::Vector;
	/// let input: Vector<u8, 5> = Vector::new([0, 1, 2, 3, 4]);
	/// let calculated: Vector<f32, 5> = Vector::from(input);
	/// let expected: Vector<f32, 5> = Vector::new([0.0, 1.0, 2.0, 3.0, 4.0]);
	/// assert_eq!(calculated, expected);
	/// ```
	pub fn from<U>(other: Vector<U, N>) -> Self
	where
		T: Default + Copy,
		U: Into<T> + Copy,
	{
		let mut vret: Vector<T, N> = Vector::filled(T::default());
		for i in 0..N {
			vret.data[i] = other.data[i].into()
		}
		vret
	}
}

#[cfg(test)]
mod conversions {
	use super::*;

	#[test]
	fn i32_to_f64_3() {
		let input: Vector<i32, 3> = Vector::new([1, 2, 3]);
		let calculated: Vector<f64, 3> = Vector::from(input);
		let expected: Vector<f64, 3> = Vector::new([1.0, 2.0, 3.0]);
		assert_eq!(calculated, expected);
	}

	#[test]
	fn i8_to_i32_2() {
		let input: Vector<i8, 2> = Vector::new([1, 2]);
		let calculated: Vector<i32, 2> = Vector::from(input);
		let expected: Vector<i32, 2> = Vector::new([1, 2]);
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

impl<T, const N: usize> AddAssign<T> for Vector<T, N>
where
	T: Default + Copy + AddAssign,
{
	fn add_assign(&mut self, other: T) {
		for i in 0..N {
			self.data[i] += other;
		}
	}
}

impl<T, const N: usize> Add<T> for Vector<T, N>
where
	T: Default + Copy + AddAssign,
{
	type Output = Vector<T, N>;
	fn add(self, other: T) -> Self::Output {
		let mut vret = self.clone();
		vret.add_assign(other);
		vret
	}
}

impl<T, const N: usize> SubAssign<T> for Vector<T, N>
where
	T: Default + Copy + SubAssign,
{
	fn sub_assign(&mut self, other: T) {
		for i in 0..N {
			self.data[i] -= other;
		}
	}
}

impl<T, const N: usize> Sub<T> for Vector<T, N>
where
	T: Default + Copy + SubAssign,
{
	type Output = Vector<T, N>;
	fn sub(self, other: T) -> Self::Output {
		let mut vret = self.clone();
		vret.sub_assign(other);
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
	/// Mutates the vector such that no dimension is greater than `step` or less than `0`.
	/// The returned vector contains the multiples of `step` that were removed for each dimension.
	/// The returned vector can be added with the modified `self` to return the vector to its original state.
	///
	/// # Examples
	/// ```
	/// use temportal_math::Vector;
	/// let vec = Vector::new([ 8.0, -5.0, 3.0 ]);
	/// let mut v_reduced = vec.clone();
	/// let remainder = v_reduced.mod_reduce(3.0);
	/// assert_eq!(v_reduced.to_vec(), vec![ 2.0, 1.0, 0.0 ]);
	/// assert_eq!(remainder.to_vec(), vec![ 6.0, -6.0, 3.0 ]);
	/// assert_eq!((v_reduced + remainder).to_vec(), vec.to_vec());
	/// ```
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

impl<const N: usize> Vector<f64, N> {
	/// Returns the euclidian remainder of the vector for a modulus `rhs`.
	/// See `f64::rem_euclid` for how `rhs` is applied to each dimension.
	pub fn rem_euclid(&self, rhs: f64) -> Self {
		let mut vret = self.clone();
		for i in 0..N {
			vret.data[i] = vret.data[i].rem_euclid(rhs);
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

impl<T, const N: usize> Vector<T, N>
where
	T: Sized + Copy + Mul<Output = T> + Add<Output = T> + Sum,
{
	/// Calculates the dot product between this and another vector.
	/// This operation is commuative.
	///
	/// # Examples
	/// Calculate the dot product between the x axis and a vector at 45 degrees.
	/// ```
	/// use temportal_math::Vector;
	/// let x_axis = Vector::new([1.0, 0.0, 0.0]);
	/// let vec = Vector::new([0.5, 0.5, 0.0]);
	/// assert_eq!(x_axis.dot(&vec), 0.5);
	/// ```
	/// The dot product shows alignment/similarity between two vectors, returning `1` if the vectors are the same,
	/// ```
	/// use temportal_math::Vector;
	/// let vec = Vector::new([1.0, 0.0, 0.0]);
	/// assert_eq!(vec.dot(&vec), 1.0);
	/// ```
	/// `0` if the vectors are entirely different,
	/// ```
	/// use temportal_math::Vector;
	/// let x_axis = Vector::new([1.0, 0.0]);
	/// let y_axis = Vector::new([0.0, 1.0]);
	/// assert_eq!(x_axis.dot(&y_axis), 0.0);
	/// ```
	/// and `-1` if the vectors are opposite.
	/// ```
	/// use temportal_math::Vector;
	/// let x_axis = Vector::new([1.0, 0.0]);
	/// let neg_x_axis = Vector::new([-1.0, 0.0]);
	/// assert_eq!(x_axis.dot(&neg_x_axis), -1.0);
	/// ```
	/// This function is commutative, so the return values of `dot` are always the same for any two vectors.
	/// ```
	/// use temportal_math::Vector;
	/// let v1 = Vector::new([0.5, 1.25, 1.0]);
	/// let v2 = Vector::new([1.0, 0.25, 2.0]);
	/// assert_eq!(v1.dot(&v2), v2.dot(&v1));
	/// ```
	pub fn dot(&self, right: &Self) -> T {
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

impl<T> Vector<T, 3>
where
	T: Default + Sized + Copy + Mul<Output = T> + Add<Output = T> + Sub<Output = T>,
{
	/// Calculates the cross-product between two three-dimensional vectors,
	/// resulting in a third three-dimensional vector.
	/// Its recommend this be only used with floating-point types (`f32`, `f64`, etc).
	/// The cross-product of two vectors is NOT COMMUTATIVE, so the order of `left` and `right` matters.
	///
	/// # Examples
	/// Crossing two vectors which are perpendicular will result in a third vector which is perpendicular to them both,
	/// and orthogonal to the plane which the inputs create.
	/// ```
	/// use temportal_math::Vector;
	/// let result = Vector::cross(&Vector::new([0.0, 1.0, 0.0]), &Vector::new([0.0, 0.0, 1.0]));
	/// assert_eq!(result.to_vec(), vec![1.0, 0.0, 0.0]);
	/// ```
	/// This property is not commutative, and passing the vectors in the opposite order
	/// results in a vector in the opposite direction.
	/// ```
	/// use temportal_math::Vector;
	/// let result = Vector::cross(&Vector::new([0.0, 0.0, 1.0]), &Vector::new([0.0, 1.0, 0.0]));
	/// assert_eq!(result.to_vec(), vec![-1.0, 0.0, 0.0]);
	/// ```
	pub fn cross(left: &Self, right: &Self) -> Self {
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
		let z_axis = Vector::cross(&Vector::new([1, 0, 0]), &Vector::new([0, 1, 0]));
		assert_eq!(z_axis.data, [0, 0, 1]);
	}
}

// #endregion

// #region Properties

impl<T, const N: usize> Vector<T, N> {
	pub fn total(&self) -> T
	where
		T: std::iter::Sum + Copy,
	{
		self.data.iter().map(|dim| *dim).sum()
	}
}

impl<T, const N: usize> Vector<T, N>
where
	T: Sized + Copy + Mul<Output = T> + Add<Output = T> + Sum,
{
	/// Calculates the magnitude^2 for the vector, as a float.
	/// Equivalent to calling dot with itself.
	///
	/// # Examples
	/// ```
	/// use temportal_math::Vector;
	/// let vec = Vector::new([2.0; 3]);
	/// assert_eq!(vec.magnitude_sq(), 12.0);
	/// ```
	pub fn magnitude_sq(&self) -> T {
		self.dot(&self)
	}

	/// Calculates the length of the vector.
	///
	/// # Examples
	/// ```
	/// use temportal_math::Vector;
	/// let vec = Vector::new([0.0, 1.0, 0.0]);
	/// assert_eq!(vec.magnitude(), 1.0);
	/// ```
	/// ```
	/// use temportal_math::Vector;
	/// let vec = Vector::new([1.0, 2.0, 3.0]);
	/// assert_eq!(vec.magnitude(), 14.0_f32.sqrt());
	/// ```
	pub fn magnitude(&self) -> f32
	where
		T: Into<f32>,
	{
		self.magnitude_sq().into().sqrt()
	}
}

impl<const N: usize> Vector<f32, N> {
	/// Mutates the vector so that its length is one, but maintains its direction.
	///
	/// # Examples
	/// ```
	/// use temportal_math::Vector;
	/// let mut vec = Vector::new([1.0, 2.0, 3.0]);
	/// assert_ne!(vec.magnitude(), 1.0);
	/// vec.normalize();
	/// assert!(vec.magnitude() - 1.0 < 1.0e-6);
	/// ```
	pub fn normalize(&mut self) {
		*self /= self.magnitude()
	}

	/// Returns a vector with the same direction as `self`, but with a length of `1.0`.
	///
	/// # Examples
	/// ```
	/// use temportal_math::Vector;
	/// let vec = Vector::new([1.0, 2.0, 3.0]);
	/// assert!(vec.normal().magnitude() - 1.0 < 1.0e-6);
	/// ```
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
		assert_eq!(Vector::new([0.0, 0.0, 0.0]).magnitude_sq(), 0.0);
		assert_eq!(Vector::new([0.0, 0.0, 1.0]).magnitude_sq(), 1.0);
		assert_eq!(Vector::new([1.0, 2.0, 0.0]).magnitude_sq(), 5.0);
		assert_eq!(Vector::new([1.0, 1.0, 1.0]).magnitude_sq(), 3.0);
	}

	#[test]
	fn magnitude() {
		assert_eq!(Vector::new([0.0, 0.0, 0.0]).magnitude(), 0.0);
		assert_eq!(Vector::new([0.0, 0.0, 1.0]).magnitude(), 1.0);
		assert_eq!(Vector::new([0.0, 2.0, 0.0]).magnitude(), 2.0);
		assert_eq!(Vector::new([2.0, 1.0, 2.0]).magnitude(), 3.0);
	}

	#[test]
	fn normal() {
		assert_eq!(Vector::new([5.0, 0.0, 0.0]).normal().data, [1.0, 0.0, 0.0]);
		assert_eq!(Vector::new([0.0, 2.0, 0.0]).normal().data, [0.0, 1.0, 0.0]);
		assert_eq!(Vector::new([0.0, 0.0, 8.0]).normal().data, [0.0, 0.0, 1.0]);
		assert_eq!(
			Vector::new([2.0, 2.0, 1.0]).normal().data,
			[2.0 / 3.0, 2.0 / 3.0, 1.0 / 3.0]
		);
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

impl<T, const N: usize> std::fmt::Display for Vector<T, N>
where
	T: std::fmt::Display,
{
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		write!(f, "<")?;
		for i in 0..N {
			if i == 0 {
				write!(f, "{}", self.data[i])?;
			} else {
				write!(f, ", {}", self.data[i])?;
			}
		}
		write!(f, ">")?;
		Ok(())
	}
}

impl<T, const N: usize> Vector<T, N>
where
	T: std::fmt::Display,
{
	pub fn display(&self) -> String {
		format!("{}", *self)
	}
}

pub struct VectorIter<T, const N: usize>
where
	T: Sized,
{
	min: Vector<T, N>,
	range: Vector<T, N>,
	step: Vector<T, N>,

	has_started: bool,
	value: Vector<T, N>,
}

impl<T, const N: usize> VectorIter<T, N> {
	pub fn new(min: Vector<T, N>, range: Vector<T, N>, step: Vector<T, N>) -> VectorIter<T, N>
	where
		T: Default + Copy,
	{
		VectorIter {
			min,
			range,
			step,
			has_started: false,
			value: Vector::new([T::default(); N]),
		}
	}
}

impl<T, const N: usize> std::iter::Iterator for VectorIter<T, N>
where
	T: Default
		+ Copy
		+ Add<Output = T>
		+ AddAssign
		+ PartialEq
		+ Rem<Output = T>
		+ std::fmt::Display
		+ Div<Output = T>,
{
	type Item = Vector<T, N>;
	fn next(&mut self) -> Option<Self::Item> {
		if !self.has_started {
			self.has_started = true;
			return Some(self.min + self.value);
		}

		for dim in 0..N {
			self.value[dim] += self.step[dim];
			if self.value[dim] == self.range[dim] {
				self.value[dim] = T::default();
			} else {
				return Some(self.min + self.value);
			}
		}

		None
	}
}

impl<T, const N: usize> Vector<T, N>
where
	T: Default + Copy,
{
	pub fn iter(&self, step: T) -> VectorIter<T, N> {
		self.iter_on(Vector::filled(step))
	}

	pub fn iter_on(&self, step: Vector<T, N>) -> VectorIter<T, N> {
		VectorIter::new(Vector::filled(T::default()), *self, step)
	}

	pub fn iter_range(min: Vector<T, N>, max: Vector<T, N>, step: Vector<T, N>) -> VectorIter<T, N>
	where
		T: SubAssign,
	{
		VectorIter::new(min, max - min, step)
	}
}

#[cfg(test)]
mod vector_iter_tests {
	use super::*;

	#[test]
	fn vector_iterates_incremental() {
		let mut iter = VectorIter::new(
			Vector::new([0, 0, 0]),
			Vector::new([2, 3, 4]),
			Vector::filled(1),
		);
		assert_eq!(iter.next(), Some(Vector::new([0, 0, 0])));
		assert_eq!(iter.next(), Some(Vector::new([1, 0, 0])));
		assert_eq!(iter.next(), Some(Vector::new([0, 1, 0])));
		assert_eq!(iter.next(), Some(Vector::new([1, 1, 0])));
		assert_eq!(iter.next(), Some(Vector::new([0, 2, 0])));
		assert_eq!(iter.next(), Some(Vector::new([1, 2, 0])));

		assert_eq!(iter.next(), Some(Vector::new([0, 0, 1])));
		assert_eq!(iter.next(), Some(Vector::new([1, 0, 1])));
		assert_eq!(iter.next(), Some(Vector::new([0, 1, 1])));
		assert_eq!(iter.next(), Some(Vector::new([1, 1, 1])));
		assert_eq!(iter.next(), Some(Vector::new([0, 2, 1])));
		assert_eq!(iter.next(), Some(Vector::new([1, 2, 1])));

		assert_eq!(iter.next(), Some(Vector::new([0, 0, 2])));
		assert_eq!(iter.next(), Some(Vector::new([1, 0, 2])));
		assert_eq!(iter.next(), Some(Vector::new([0, 1, 2])));
		assert_eq!(iter.next(), Some(Vector::new([1, 1, 2])));
		assert_eq!(iter.next(), Some(Vector::new([0, 2, 2])));
		assert_eq!(iter.next(), Some(Vector::new([1, 2, 2])));

		assert_eq!(iter.next(), Some(Vector::new([0, 0, 3])));
		assert_eq!(iter.next(), Some(Vector::new([1, 0, 3])));
		assert_eq!(iter.next(), Some(Vector::new([0, 1, 3])));
		assert_eq!(iter.next(), Some(Vector::new([1, 1, 3])));
		assert_eq!(iter.next(), Some(Vector::new([0, 2, 3])));
		assert_eq!(iter.next(), Some(Vector::new([1, 2, 3])));

		assert_eq!(iter.next(), None);
	}
}
