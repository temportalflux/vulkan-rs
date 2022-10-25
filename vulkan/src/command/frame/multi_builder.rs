use crate::{
	command::frame::Buffer, device::logical, image_view::View, renderpass, structs::Extent2D,
	utility::NameableBuilder,
};
use std::sync::{Arc, Weak};

/// Represents an [`image view`](View) which is attached to a [`framebuffer`](Buffer) at a particular slot.
///
/// This structure holds a strong reference to the view attachments.
pub enum AttachedView {
	/// This slot has a different view for each frame.
	PerFrame(Vec<Arc<View>>),
	/// This slot has a single view which is shared by each frame.
	Shared(Arc<View>),
}

impl AttachedView {
	/// Returns true if the attachment supports the provided number of frames.
	fn has_frames(&self, count: usize) -> bool {
		match &self {
			Self::PerFrame(frames) => frames.len() == count,
			Self::Shared(_) => true,
		}
	}

	/// Returns a weak pointer to the view to attach, when provided a specific frame.
	fn get_frame(&self, index: usize) -> Weak<View> {
		Arc::downgrade(match &self {
			Self::PerFrame(frames) => &frames[index],
			Self::Shared(view) => view,
		})
	}
}

/// Information used to construct a vec/ring of [frame buffers](Buffer).
#[derive(Default)]
pub struct MultiBuilder {
	name: String,
	extent: Extent2D,
	attachments_per_frame: Vec<Vec<Weak<View>>>,
}

impl NameableBuilder for MultiBuilder {
	fn set_name(&mut self, name: impl Into<String>) {
		self.name = name.into();
	}

	fn name(&self) -> &String {
		&self.name
	}
}

impl MultiBuilder {
	/// Apply the resolution of the framebuffers.
	pub fn with_extent(mut self, extent: Extent2D) -> Self {
		self.set_extent(extent);
		self
	}

	/// Set the resolution of the framebuffers.
	pub fn set_extent(&mut self, extent: Extent2D) -> &mut Self {
		self.extent = extent;
		self
	}

	pub fn with_sizes(mut self, frame_count: usize, attachment_count: usize) -> Self {
		assert!(self.attachments_per_frame.is_empty());
		self.attachments_per_frame = vec![vec![Weak::new(); attachment_count]; frame_count];
		self
	}

	pub fn attach(&mut self, slot: usize, view: AttachedView) -> &mut Self {
		assert!(slot < self.attachments_per_frame[0].len());
		assert!(view.has_frames(self.attachments_per_frame.len()));
		for (index, frame) in self.attachments_per_frame.iter_mut().enumerate() {
			frame[slot] = view.get_frame(index);
		}
		self
	}

	pub fn build(
		self,
		device: &Arc<logical::Device>,
		render_pass: &renderpass::Pass,
	) -> anyhow::Result<Vec<Arc<Buffer>>> {
		let mut buffers = Vec::with_capacity(self.attachments_per_frame.len());
		for (i, attachments) in self.attachments_per_frame.into_iter().enumerate() {
			let attachments = attachments
				.into_iter()
				.map(|weak| weak.upgrade().unwrap())
				.collect::<Vec<_>>();
			let deref_attachments = attachments.iter().map(|arc| &**arc).collect::<Vec<_>>();
			buffers.push(Arc::new(
				Buffer::builder()
					.with_name(format!("{}.{}", self.name, i))
					.set_extent(self.extent)
					.build(deref_attachments, &render_pass, &device)?,
			));
		}
		Ok(buffers)
	}
}
