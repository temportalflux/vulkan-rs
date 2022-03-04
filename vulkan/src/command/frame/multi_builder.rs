use crate::{
	command::frame::Buffer, device::logical, image_view::View, renderpass, structs::Extent2D,
	utility::NameableBuilder,
};
use std::sync::Arc;

/// Information used to construct a vec/ring of [frame buffers](Buffer).
#[derive(Default)]
pub struct MultiBuilder {
	name: Option<String>,
	extent: Extent2D,
	attachments_per_frame: Vec<Vec<Arc<View>>>,
}

impl NameableBuilder for MultiBuilder {
	fn set_optname(&mut self, name: Option<String>) {
		self.name = name;
	}

	fn name(&self) -> &Option<String> {
		&self.name
	}
}

impl MultiBuilder {
	pub fn with_extent(mut self, extent: Extent2D) -> Self {
		self.set_extent(extent);
		self
	}

	pub fn set_extent(&mut self, extent: Extent2D) {
		self.extent = extent;
	}

	pub fn with_frame_count(mut self, count: usize) -> Self {
		assert!(self.attachments_per_frame.is_empty());
		self.attachments_per_frame = vec![vec![]; count];
		self
	}

	pub fn attach(self, view: Arc<View>) -> Self {
		let views = vec![view; self.attachments_per_frame.len()];
		self.attach_by_frame(views)
	}

	pub fn attach_by_frame(mut self, views: Vec<Arc<View>>) -> Self {
		assert!(!self.attachments_per_frame.is_empty());
		assert_eq!(views.len(), self.attachments_per_frame.len());
		let iter = self.attachments_per_frame.iter_mut().zip(views.into_iter());
		for (attachments, view) in iter {
			attachments.push(view);
		}
		self
	}

	pub fn build(
		self,
		device: &Arc<logical::Device>,
		render_pass: &renderpass::Pass,
	) -> anyhow::Result<Vec<Arc<Buffer>>> {
		let name = self.name().clone();
		let mut buffers = Vec::with_capacity(self.attachments_per_frame.len());
		for (i, attachments) in self.attachments_per_frame.into_iter().enumerate() {
			let deref_attachments = attachments.iter().map(|arc| &**arc).collect::<Vec<_>>();
			buffers.push(Arc::new(
				Buffer::builder()
					.with_optname(name.as_ref().map(|name| format!("{}.{}", name, i)))
					.set_extent(self.extent)
					.build(deref_attachments, &render_pass, &device)?,
			));
		}
		Ok(buffers)
	}
}
