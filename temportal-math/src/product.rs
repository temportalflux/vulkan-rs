pub trait DotProduct<Rhs> {
	type Output;
	fn dot(&self, right: &Rhs) -> Self::Output;
}

pub trait CrossProduct {
	fn cross(left: &Self, rgs: &Self) -> Self;
}
