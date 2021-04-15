use crate::flags::ShaderStageKind;

pub struct Info {
	pub kind: ShaderStageKind,
	pub entry_point: String,
	pub bytes: Vec<u8>,
}
