use crate::{
	backend,
	device::logical,
	flags, renderpass,
	utility::{self, VulkanInfo, VulkanObject},
};
use std::sync;

/// Information used to create a [`Render Pass`](crate::renderpass::Pass).
pub struct Info {
	attachments: Vec<renderpass::Attachment>,
	subpasses: Vec<renderpass::Subpass>,
	dependencies: Vec<(Dependency, Dependency)>,
}

impl Default for Info {
	fn default() -> Info {
		let mut rp_info = renderpass::Info::empty();

		let frame_attachment_index = rp_info.attach(
			renderpass::Attachment::default()
				.set_format(flags::Format::B8G8R8A8_SRGB)
				.set_sample_count(flags::SampleCount::TYPE_1)
				.set_general_ops(renderpass::AttachmentOps {
					load: flags::AttachmentLoadOp::CLEAR,
					store: flags::AttachmentStoreOp::STORE,
				})
				.set_final_layout(flags::ImageLayout::PRESENT_SRC_KHR),
		);

		let main_pass_index =
			rp_info.add_subpass(renderpass::Subpass::default().add_attachment_ref(
				frame_attachment_index,
				flags::ImageLayout::COLOR_ATTACHMENT_OPTIMAL,
			));

		rp_info.add_dependency(
			renderpass::Dependency::new(None)
				.set_stage(flags::PipelineStage::COLOR_ATTACHMENT_OUTPUT),
			renderpass::Dependency::new(Some(main_pass_index))
				.set_stage(flags::PipelineStage::COLOR_ATTACHMENT_OUTPUT)
				.set_access(flags::Access::COLOR_ATTACHMENT_WRITE),
		);

		rp_info
	}
}

impl Info {
	pub fn empty() -> Info {
		Info {
			attachments: Vec::new(),
			subpasses: Vec::new(),
			dependencies: Vec::new(),
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
	pub fn add_subpass(&mut self, subpass: renderpass::Subpass) -> usize {
		let index = self.subpasses.len();
		self.subpasses.push(subpass);
		index
	}
}

/// Information about a [`Subpass`](renderpass::Subpass) that is dependent on
/// or is a dependecy of another [`Subpass`](renderpass::Subpass).
#[derive(Copy, Clone, Debug)]
pub struct Dependency {
	subpass_index: Option<usize>,
	stage_mask: flags::PipelineStage,
	access_mask: flags::Access,
}

impl Dependency {
	pub fn new(subpass_index: Option<usize>) -> Dependency {
		Dependency {
			subpass_index,
			stage_mask: flags::PipelineStage::empty(),
			access_mask: flags::Access::empty(),
		}
	}

	pub fn set_stage(mut self, stage: flags::PipelineStage) -> Self {
		self.stage_mask = stage;
		self
	}

	pub fn set_access(mut self, access: flags::Access) -> Self {
		self.access_mask = access;
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

impl Info {
	pub fn create_object(
		&self,
		device: &sync::Arc<logical::Device>,
	) -> utility::Result<renderpass::Pass> {
		use backend::version::DeviceV1_0;
		let attachments = self
			.attachments
			.iter()
			.map(|v| v.to_vk())
			.collect::<Vec<_>>();
		let subpasses = self.subpasses.iter().map(|v| v.to_vk()).collect::<Vec<_>>();
		let dependencies = self
			.dependencies
			.iter()
			.map(|(src, dst)| {
				backend::vk::SubpassDependency::builder()
					.src_subpass(
						src.subpass_index
							.unwrap_or(backend::vk::SUBPASS_EXTERNAL as usize) as u32,
					)
					.src_stage_mask(src.stage_mask)
					.src_access_mask(src.access_mask)
					.dst_subpass(
						dst.subpass_index
							.unwrap_or(backend::vk::SUBPASS_EXTERNAL as usize) as u32,
					)
					.dst_stage_mask(dst.stage_mask)
					.dst_access_mask(dst.access_mask)
					.build()
			})
			.collect::<Vec<_>>();
		let vk_info = backend::vk::RenderPassCreateInfo::builder()
			.attachments(&attachments)
			.subpasses(&subpasses)
			.dependencies(&dependencies)
			.build();
		let vk = utility::as_vulkan_error(unsafe {
			device.unwrap().create_render_pass(&vk_info, None)
		})?;
		Ok(renderpass::Pass::from(device.clone(), vk))
	}
}
