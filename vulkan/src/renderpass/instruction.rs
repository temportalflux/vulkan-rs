use crate::{renderpass, structs};

#[derive(Clone, Debug)]
pub struct RecordInstruction {
	pub render_area: structs::Rect2D,
	pub clear_values: Vec<renderpass::ClearValue>,
}

impl Default for RecordInstruction {
	fn default() -> RecordInstruction {
		RecordInstruction {
			render_area: structs::Rect2D {
				offset: structs::Offset2D { x: 0, y: 0 },
				extent: structs::Extent2D {
					width: 0,
					height: 0,
				},
			},
			clear_values: Vec::new(),
		}
	}
}

impl RecordInstruction {
	pub fn with_extent(mut self, area: structs::Extent2D) -> Self {
		self.set_extent(area);
		self
	}

	pub fn set_extent(&mut self, area: structs::Extent2D) {
		self.render_area.extent = area;
	}

	pub fn clear_with(mut self, clear_value: renderpass::ClearValue) -> Self {
		self.add_clear_value(clear_value);
		self
	}

	pub fn add_clear_value(&mut self, clear_value: renderpass::ClearValue) {
		self.clear_values.push(clear_value);
	}
}
