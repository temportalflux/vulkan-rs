use super::{attachment, Dependency};
use crate::{
	backend,
	flags::{AttachmentKind, PipelineBindPoint},
};
use std::sync::Arc;

/// Represents a particular phase of rendering (equivalent to a [`subpass`](crate::renderpass::Subpass)).
pub struct Phase {
	name: String,
	bind_point: PipelineBindPoint,
	dependencies: Vec<Dependency>,
	attachments: Vec<attachment::Reference>,
}

impl Phase {
	/// Create a new phase with a particular name.
	pub fn new<T: Into<String>>(name: T) -> Self {
		Self {
			name: name.into(),
			bind_point: PipelineBindPoint::GRAPHICS,
			attachments: Vec::new(),
			dependencies: Vec::new(),
		}
	}

	pub fn name(&self) -> &String {
		&self.name
	}

	pub fn with_bind_point(mut self, point: PipelineBindPoint) -> Self {
		self.bind_point = point;
		self
	}

	pub fn with_dependency(mut self, dependency: Dependency) -> Self {
		self.add_dependency(dependency);
		self
	}

	pub fn add_dependency(&mut self, dependency: Dependency) {
		self.dependencies.push(dependency);
	}

	pub fn dependencies(&self) -> &Vec<Dependency> {
		&self.dependencies
	}

	pub fn with_attachment(mut self, attachment: attachment::Reference) -> Self {
		self.attachments.push(attachment);
		self
	}

	pub fn attachments(&self) -> &Vec<attachment::Reference> {
		&self.attachments
	}

	pub(crate) fn create_cache(
		&self,
		attachment_set: &attachment::Set,
	) -> BackendSubpassAttachments {
		BackendSubpassAttachments::new(&self.attachments, &attachment_set)
	}

	pub(crate) fn as_desc(
		&self,
		cache: &BackendSubpassAttachments,
	) -> backend::vk::SubpassDescription {
		let mut builder = backend::vk::SubpassDescription::builder()
			.pipeline_bind_point(self.bind_point)
			.input_attachments(&cache.input)
			.color_attachments(&cache.color)
			.preserve_attachments(&cache.preserve);
		if cache.resolve.len() > 0 {
			// vulkan assumes that resolve attachments match the number of color attachments
			assert_eq!(cache.resolve.len(), cache.color.len());
			builder = builder.resolve_attachments(&cache.resolve);
		}
		if let Some(ds_attach) = &cache.depth_stencil {
			builder = builder.depth_stencil_attachment(ds_attach);
		}
		builder.build()
	}
}

#[derive(Default)]
pub(crate) struct BackendSubpassAttachments {
	input: Vec<backend::vk::AttachmentReference>,
	color: Vec<backend::vk::AttachmentReference>,
	resolve: Vec<backend::vk::AttachmentReference>,
	preserve: Vec<u32>,
	depth_stencil: Option<backend::vk::AttachmentReference>,
}

impl BackendSubpassAttachments {
	fn new(attachments: &Vec<attachment::Reference>, attachment_set: &attachment::Set) -> Self {
		let mut cache = Self::default();
		for attachment_ref in attachments.iter() {
			let vk = backend::vk::AttachmentReference::builder()
				.attachment({
					let weak_attachment = Arc::downgrade(attachment_ref.attachment());
					let attachment_index = attachment_set.position(&weak_attachment).unwrap();
					attachment_index as u32
				})
				.layout(attachment_ref.layout().into())
				.build();
			match attachment_ref.kind() {
				AttachmentKind::Input => {
					cache.input.push(vk);
				}
				AttachmentKind::Color => {
					cache.color.push(vk);
				}
				AttachmentKind::Resolve => {
					cache.resolve.push(vk);
				}
				AttachmentKind::Preserve => {
					cache.preserve.push(vk.attachment);
				}
				AttachmentKind::DepthStencil => {
					cache.depth_stencil = Some(vk);
				}
			}
		}
		cache
	}
}
