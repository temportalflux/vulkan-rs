use crate::structs::{subresource, Extent3D, Offset3D};

/// Properties used to copy
/// a portion of a [`buffer`](crate::buffer::Buffer)
/// to an area of an [`image`](crate::image::Image).
///
/// Used in conjunction with [`copy_buffer_to_image`](crate::command::Buffer::copy_buffer_to_image).
pub struct CopyBufferToImage {
	/// The offset from the start of the buffer to read data from.
	pub buffer_offset: usize,
	/// The image's subresource layers.
	pub layers: subresource::Layers,
	/// The offset from the start of each dimension of the image to write to.
	pub offset: Offset3D,
	/// The size of the image segment to write to.
	pub size: Extent3D,
}

/// Properties used to copy
/// a portion of a [`buffer`](crate::buffer::Buffer)
/// to a portion of another [`buffer`](crate::buffer::Buffer).
///
/// Used in conjunction with [`copy_buffer_to_buffer`](crate::command::Buffer::copy_buffer_to_buffer).
pub struct CopyBufferRange {
	/// The byte offset from the start of the source buffer.
	pub start_in_src: usize,
	/// The byte offset from the start of the destination buffer.
	pub start_in_dst: usize,
	/// How many bytes to copy.
	pub size: usize,
}
