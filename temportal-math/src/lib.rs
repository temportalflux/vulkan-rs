mod product;
mod quaternion;
mod vector;

pub use product::*;
pub use quaternion::*;
pub use vector::*;

#[macro_export]
macro_rules! assert_vec_approx {
	($a:expr, $b:expr) => {
		assert_eq!($a.capacity(), $b.capacity());
		for i in 0..$a.capacity() {
			let eps = 1.0e-6;
			let (a, b) = (&$a[i], &$b[i]);
			assert!(
					(*a - *b).abs() < eps,
					"(left !== right)':\n\
						index={}\n\
						left=`{:?}`\n\
						right=`{:?}`\n\
						expect diff=`{:?}`\n\
						real diff=`{:?}`\n",
					i, $a, $b,
					eps, (*a - *b).abs()
			);
		}
	};
	($a:expr, $b:expr, $eps:expr) => {
		assert_eq!($a.capacity(), $b.capacity());
		for i in 0..$a.capacity() {
			let (a, b) = (&$a[i], &$b[i]);
			assert!(
					(*a - *b).abs() < $eps,
					"(left !== right)':\n\
						index={}\n\
						left=`{:?}`\n\
						right=`{:?}`\n\
						expect diff=`{:?}`\n\
						real diff=`{:?}`\n",
					i, $a, $b,
					$eps, (*a - *b).abs()
			);
		}
	};
	($a:expr, $b:expr, $eps:expr, $modulus:expr) => {
		assert_eq!($a.capacity(), $b.capacity());
		for i in 0..$a.capacity() {
			let (a, b) = (&$a[i].rem_euclid($modulus), &$b[i]);
			assert!(
					(*a - *b).abs() < $eps,
					"(left !== right)':\n\
						index={}\n\
						left=`{:?}`\n\
						right=`{:?}`\n\
						expect diff=`{:?}`\n\
						real diff=`{:?}`\n",
					i, $a, $b,
					$eps, (*a - *b).abs()
			);
		}
	};
}
