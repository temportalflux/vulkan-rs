use crate::Vector;
use serde::{
	de::{self, SeqAccess, Visitor},
	ser::SerializeTuple,
	Deserialize, Deserializer, Serialize, Serializer,
};

impl<T, const N: usize> Serialize for Vector<T, N>
where
	T: Serialize,
{
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: Serializer,
	{
		let mut serialized = serializer.serialize_tuple(N)?;
		for i in 0..N {
			serialized.serialize_element(&self.data[i])?;
		}
		serialized.end()
	}
}

impl<'de, T, const N: usize> Deserialize<'de> for Vector<T, N>
where
	T: Deserialize<'de> + Default + Copy,
{
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
	where
		D: Deserializer<'de>,
	{
		deserializer.deserialize_seq(VectorVisitor::new())
	}
}

struct VectorVisitor<T, const N: usize> {
	marker: std::marker::PhantomData<fn() -> Vector<T, N>>,
}

impl<T, const N: usize> VectorVisitor<T, N> {
	fn new() -> Self {
		VectorVisitor {
			marker: std::marker::PhantomData,
		}
	}
}

impl<'de, T, const N: usize> Visitor<'de> for VectorVisitor<T, N>
where
	T: Deserialize<'de> + Default + Copy,
{
	type Value = Vector<T, N>;

	fn expecting(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		write!(f, "{} values of {}", N, std::any::type_name::<T>())
	}

	fn visit_seq<S>(self, mut seq: S) -> Result<Self::Value, S::Error>
	where
		S: SeqAccess<'de>,
	{
		let mut vret = Vector::filled(T::default());
		for i in 0..N {
			vret.data[i] = seq
				.next_element::<T>()?
				.ok_or_else(|| de::Error::custom(format!("Missing dimension {} in sequence", N)))?;
		}
		Ok(vret)
	}
}
