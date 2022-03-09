use super::Phase;
use crate::flags::{Access, PipelineStage};
use enumset::EnumSet;
use std::sync::{Arc, Weak};

#[derive(Default)]
pub struct PhaseAccess {
	stage: EnumSet<PipelineStage>,
	access: EnumSet<Access>,
}
impl PhaseAccess {
	pub fn with_stage(mut self, stage: PipelineStage) -> Self {
		self.stage.insert(stage);
		self
	}

	pub fn stage(&self) -> &EnumSet<PipelineStage> {
		&self.stage
	}

	pub fn with_access(mut self, access: Access) -> Self {
		self.access.insert(access);
		self
	}

	pub fn access(&self) -> &EnumSet<Access> {
		&self.access
	}
}

pub struct Dependency {
	phase: Option<Weak<Phase>>,
	first: PhaseAccess,
	then: PhaseAccess,
}
impl Dependency {
	pub fn new(phase: Option<&Arc<Phase>>) -> Self {
		Self {
			phase: phase.map(|arc| Arc::downgrade(&arc)),
			first: PhaseAccess::default(),
			then: PhaseAccess::default(),
		}
	}

	pub fn first(mut self, link: PhaseAccess) -> Self {
		self.first = link;
		self
	}

	pub fn then(mut self, link: PhaseAccess) -> Self {
		self.then = link;
		self
	}

	pub fn get_phase(&self) -> Option<Arc<Phase>> {
		self.phase.as_ref().map(|phase| phase.upgrade()).flatten()
	}

	pub fn get_first(&self) -> &PhaseAccess {
		&self.first
	}

	pub fn get_then(&self) -> &PhaseAccess {
		&self.then
	}
}
