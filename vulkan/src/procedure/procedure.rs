use super::{attachment, Phase};
use crate::{
	backend,
	device::logical,
	flags::{Access, PipelineStage},
	renderpass, utility,
};
use std::sync::Arc;

/// A high-level approach to building [`render passes`](renderpass::Pass).
#[derive(Default)]
pub struct Procedure {
	name: Option<String>,
	phases: Vec<Arc<Phase>>,
	attachments: attachment::Set,
}

impl Procedure {

	/// Apply a phase to the render order of the procedure.
	pub fn with_phase(
		mut self,
		phase: Arc<Phase>,
	) -> Result<Self, PhaseAddedWithMissingDependency> {
		self.add_phase(phase)?;
		Ok(self)
	}

	/// Add a phase to the render order of the procedure.
	/// All dependencies of the provided phase must already be added.
	/// 
	/// Also adds the attachments for each phase as weak references to the set of attachments.
	pub fn add_phase(&mut self, phase: Arc<Phase>) -> Result<&mut Self, PhaseAddedWithMissingDependency> {
		for (index, dependency) in phase.dependencies().iter().enumerate() {
			if let Some(phase) = dependency.get_phase() {
				if self.position(&phase).is_none() {
					return Err(PhaseAddedWithMissingDependency(index));
				}
			}
		}
		for attachment_ref in phase.attachments().iter() {
			self.attachments
				.insert(Arc::downgrade(&attachment_ref.attachment()));
		}
		self.phases.push(phase);
		Ok(self)
	}

	/// Returns the index position of the provided phase in the list of added phases.
	pub fn position(&self, phase: &Arc<Phase>) -> Option<usize> {
		self.phases.iter().position(|arc| Arc::ptr_eq(arc, phase))
	}

	/// Returns the number of phases.
	pub fn num_phases(&self) -> usize {
		self.phases.len()
	}

	/// Returns an iterator over the phases.
	pub fn iter(&self) -> impl std::iter::Iterator<Item = &Arc<Phase>> {
		self.phases.iter()
	}

	/// Returns a reference to the set of attachments.
	pub fn attachments(&self) -> &attachment::Set {
		&self.attachments
	}
}

impl utility::NameableBuilder for Procedure {
	fn set_optname(&mut self, name: Option<String>) {
		self.name = name;
	}

	fn name(&self) -> &Option<String> {
		&self.name
	}
}

#[derive(thiserror::Error, Debug)]
pub struct PhaseAddedWithMissingDependency(usize);
impl std::fmt::Display for PhaseAddedWithMissingDependency {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		write!(f, "Attempted to add phase to procedure, but it dependency {} has not been added to the procedure (phases can only be added after their dependencies have been added).", self.0)
	}
}

impl Procedure {
	/// Builds a render pass from the procedure.
	/// This does not consume the procedure so that it can be use to create additional passes in the future,
	/// as well as referenced when creating pipelines and framebuffers.
	pub fn build(&self, device: &Arc<logical::Device>) -> anyhow::Result<renderpass::Pass> {
		use backend::version::DeviceV1_0;
		use utility::{HandledObject, NameableBuilder};

		let vk_attachments = self
			.attachments
			.iter()
			.filter_map(|weak| weak.upgrade())
			.map(|attachment| attachment.as_desc())
			.collect::<Vec<_>>();

		let mut subpasses = Vec::with_capacity(self.phases.len());
		let mut attachment_caches = Vec::with_capacity(self.phases.len());
		let mut dependencies = Vec::with_capacity(self.phases.len());
		for (index, phase) in self.phases.iter().enumerate() {
			let cache_index = attachment_caches.len();
			attachment_caches.push(phase.create_cache(&self.attachments));
			subpasses.push(phase.as_desc(&attachment_caches[cache_index]));

			for dependency in phase.dependencies().iter() {
				let src_index = dependency
					.get_phase()
					.map(|arc| self.position(&arc).unwrap() as u32)
					.unwrap_or(backend::vk::SUBPASS_EXTERNAL);
				let src_stage = PipelineStage::fold(&dependency.get_first().stage());
				let src_access = Access::fold(&dependency.get_first().access());

				let dst_index = index as u32;
				let dst_stage = PipelineStage::fold(&dependency.get_then().stage());
				let dst_access = Access::fold(&dependency.get_then().access());

				dependencies.push(
					backend::vk::SubpassDependency::builder()
						.src_subpass(src_index)
						.src_stage_mask(src_stage)
						.src_access_mask(src_access)
						.dst_subpass(dst_index)
						.dst_stage_mask(dst_stage)
						.dst_access_mask(dst_access)
						.build(),
				);
			}
		}

		let vk_info = backend::vk::RenderPassCreateInfo::builder()
			.attachments(&vk_attachments)
			.subpasses(&subpasses)
			.dependencies(&dependencies)
			.build();
		let vk = unsafe { device.create_render_pass(&vk_info, None) }?;
		let pass = renderpass::Pass::from(device.clone(), vk, vec![]);
		if let Some(name) = self.name().as_ref() {
			device.set_object_name_logged(&pass.create_name(name.as_str()));
		}
		Ok(pass)
	}
}
