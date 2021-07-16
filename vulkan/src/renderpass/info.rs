use crate::{backend, device::logical, flags, renderpass, utility};
use enumset::EnumSet;
use std::sync;

/// Information used to create a [`Render Pass`](crate::renderpass::Pass).
#[derive(Debug, Clone)]
pub struct Info {
	attachments: Vec<renderpass::Attachment>,
	subpass_order: Vec<renderpass::Subpass>,
	dependencies: Vec<(Dependency, Dependency)>,
	name: Option<String>,
}

impl Default for Info {
	fn default() -> Info {
		let mut rp_info = renderpass::Info::empty();

		rp_info.attach(
			renderpass::Attachment::new("frame".to_owned())
				.with_format(flags::format::Format::B8G8R8A8_SRGB)
				.with_sample_count(flags::SampleCount::default())
				.with_general_ops(renderpass::AttachmentOps {
					load: flags::LoadOp::Clear,
					store: flags::StoreOp::Store,
				})
				.with_final_layout(flags::ImageLayout::PresentSrc),
		);

		rp_info.add_subpass(
			renderpass::Subpass::new("default".to_owned()).add_color_attachment(
				"frame".to_owned(),
				flags::ImageLayout::ColorAttachmentOptimal,
			),
		);

		rp_info.add_dependency(
			renderpass::Dependency::new(None)
				.with_stage(flags::PipelineStage::ColorAttachmentOutput),
			renderpass::Dependency::new(Some("default".to_owned()))
				.with_stage(flags::PipelineStage::ColorAttachmentOutput)
				.with_access(flags::Access::ColorAttachmentWrite),
		);

		rp_info
	}
}

impl Info {
	pub fn empty() -> Info {
		Info {
			attachments: Vec::new(),
			subpass_order: Vec::new(),
			dependencies: Vec::new(),
			name: None,
		}
	}

	/// Attaches an attachment to the renderpass info,
	/// returning its numerical id in the [`Render Pass`](crate::renderpass::Pass).
	pub fn attach(&mut self, attachment: renderpass::Attachment) -> usize {
		let index = self.attachments.len();
		self.attachments.push(attachment);
		index
	}

	/// Adds a subpass (and all its attachment references),
	/// returning its numerical id in the [`Render Pass`](crate::renderpass::Pass).
	pub fn add_subpass(&mut self, subpass: renderpass::Subpass) {
		self.subpass_order.push(subpass);
	}

	pub fn subpass_order(&self) -> Vec<String> {
		self.subpass_order
			.iter()
			.map(|subpass| subpass.id().clone())
			.collect()
	}
}

/// Information about a [`Subpass`](renderpass::Subpass) that is dependent on
/// or is a dependecy of another [`Subpass`](renderpass::Subpass).
#[derive(Clone, Debug)]
pub struct Dependency {
	subpass_id: Option<String>,
	stage_mask: EnumSet<flags::PipelineStage>,
	access_mask: EnumSet<flags::Access>,
}

impl Dependency {
	pub fn new(subpass_id: Option<String>) -> Dependency {
		Dependency {
			subpass_id,
			stage_mask: EnumSet::empty(),
			access_mask: EnumSet::empty(),
		}
	}

	pub fn with_stage_set(mut self, stages: EnumSet<flags::PipelineStage>) -> Self {
		self.stage_mask = stages;
		self
	}

	pub fn with_stage(mut self, stage: flags::PipelineStage) -> Self {
		self.stage_mask |= stage;
		self
	}

	pub fn with_access_set(mut self, access: EnumSet<flags::Access>) -> Self {
		self.access_mask = access;
		self
	}

	pub fn with_access(mut self, access: flags::Access) -> Self {
		self.access_mask |= access;
		self
	}
}

impl Info {
	/// Denotes that a given subpass is dependent on some other subpass,
	/// with the relevant stage and access flags.
	pub fn add_dependency(&mut self, requirement: Dependency, required_by: Dependency) {
		self.dependencies.push((requirement, required_by));
	}
}

struct BackendSubpassAttachments {
	input: Vec<backend::vk::AttachmentReference>,
	color: Vec<backend::vk::AttachmentReference>,
	depth_stencil: Option<backend::vk::AttachmentReference>,
}

impl BackendSubpassAttachments {
	fn new<F>(attachments: renderpass::SubpassAttachments, index_of: F) -> Self
	where
		F: Fn(&String) -> u32,
	{
		let map_attachment = |(attachment_id, layout): (String, flags::ImageLayout)| {
			backend::vk::AttachmentReference::builder()
				.attachment(index_of(&attachment_id))
				.layout(layout.into())
				.build()
		};
		Self {
			input: attachments.input.into_iter().map(map_attachment).collect(),
			color: attachments.color.into_iter().map(map_attachment).collect(),
			depth_stencil: attachments.depth_stencil.map(map_attachment),
		}
	}
}

impl Info {
	fn find_subpass_index(&self, id: &Option<String>) -> u32 {
		if let Some(id) = id {
			if let Some(idx) = self
				.subpass_order
				.iter()
				.position(|subpass| *subpass.id() == *id)
			{
				return idx as u32;
			}
		}
		backend::vk::SUBPASS_EXTERNAL
	}

	fn find_attachment_index(&self, id: &String) -> Option<u32> {
		self.attachments
			.iter()
			.position(|attachment| *attachment.id() == *id)
			.map(|i| i as u32)
	}
}

impl utility::NameableBuilder for Info {
	fn with_optname(mut self, name: Option<String>) -> Self {
		self.name = name;
		self
	}

	fn name(&self) -> &Option<String> {
		&self.name
	}
}

impl utility::BuildFromDevice for Info {
	type Output = renderpass::Pass;
	fn build(self, device: &sync::Arc<logical::Device>) -> utility::Result<Self::Output> {
		use backend::version::DeviceV1_0;
		let attachments = self
			.attachments
			.iter()
			.map(|v| v.clone().into())
			.collect::<Vec<_>>();
		let mut subpasses = Vec::with_capacity(self.subpass_order.len());
		let mut subpass_attachments = Vec::with_capacity(self.subpass_order.len());
		for subpass in self.subpass_order.iter() {
			let attachments_idx = subpass_attachments.len();
			subpass_attachments.push(BackendSubpassAttachments::new(
				subpass.attachments().clone(),
				|attachment_id| self.find_attachment_index(attachment_id).unwrap(),
			));
			let attachments = &subpass_attachments[attachments_idx];
			subpasses.push({
				let mut builder = backend::vk::SubpassDescription::builder()
					.pipeline_bind_point(subpass.bind_point())
					.input_attachments(&attachments.input)
					.color_attachments(&attachments.color);
				if let Some(ds_attach) = &attachments.depth_stencil {
					builder = builder.depth_stencil_attachment(ds_attach);
				}
				builder.build()
			});
		}
		let dependencies = self
			.dependencies
			.iter()
			.map(|(src, dst)| {
				backend::vk::SubpassDependency::builder()
					.src_subpass(self.find_subpass_index(&src.subpass_id))
					.src_stage_mask(flags::PipelineStage::fold(&src.stage_mask))
					.src_access_mask(flags::Access::fold(&src.access_mask))
					.dst_subpass(self.find_subpass_index(&dst.subpass_id))
					.dst_stage_mask(flags::PipelineStage::fold(&dst.stage_mask))
					.dst_access_mask(flags::Access::fold(&dst.access_mask))
					.build()
			})
			.collect::<Vec<_>>();
		let vk_info = backend::vk::RenderPassCreateInfo::builder()
			.attachments(&attachments)
			.subpasses(&subpasses)
			.dependencies(&dependencies)
			.build();
		let vk = unsafe { device.create_render_pass(&vk_info, None) }?;
		let pass = renderpass::Pass::from(
			device.clone(),
			vk,
			self.subpass_order(),
		);
		self.set_object_name(device, &pass);
		Ok(pass)
	}
}
