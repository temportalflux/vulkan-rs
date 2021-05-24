use crate::structs::{subresource, Extent3D, Offset3D};

pub struct CopyBufferToImage {
	pub buffer_offset: usize,
	pub layers: subresource::Layers,
	pub offset: Offset3D,
	pub size: Extent3D,
}

pub struct CopyBufferRange {
	pub start_in_src: usize,
	pub start_in_dst: usize,
	pub size: usize,
}
