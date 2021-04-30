use crate::structs::subresource;
use temportal_math::Vector;

pub struct CopyBufferToImage {
	pub buffer_offset: usize,
	pub layers: subresource::Layers,
	pub offset: Vector<i32, 3>,
	pub size: Vector<usize, 3>,
}
