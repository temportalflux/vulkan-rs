extern crate sdl2;
extern crate vk_mem;

/// Various forwarded/exposed enumerations from Vulkan/Erupt
pub mod flags {
	pub use erupt::vk::AccessFlags as Access;
	pub use erupt::vk::AttachmentLoadOp;
	pub use erupt::vk::AttachmentStoreOp;
	pub use erupt::vk::ColorComponentFlags as ColorComponent;
	pub use erupt::vk::ColorSpaceKHR as ColorSpace;
	pub use erupt::vk::ComponentSwizzle;
	pub use erupt::vk::CompositeAlphaFlagBitsKHR as CompositeAlpha;
	pub use erupt::vk::CullModeFlags as CullMode;
	pub use erupt::vk::FenceCreateFlags as FenceState;
	pub use erupt::vk::Format;
	pub use erupt::vk::FrontFace;
	pub use erupt::vk::ImageAspectFlags as ImageAspect;
	pub use erupt::vk::ImageLayout;
	pub use erupt::vk::ImageUsageFlags;
	pub use erupt::vk::ImageViewType;
	pub use erupt::vk::PipelineBindPoint;
	pub use erupt::vk::PipelineStageFlags as PipelineStage;
	pub use erupt::vk::PolygonMode;
	pub use erupt::vk::PresentModeKHR as PresentMode;
	pub use erupt::vk::QueueFlags;
	pub use erupt::vk::SampleCountFlagBits as SampleCount;
	pub use erupt::vk::ShaderStageFlagBits as ShaderStageKind;
	pub use erupt::vk::SharingMode;
	pub use erupt::vk::SurfaceTransformFlagBitsKHR as SurfaceTransform;
}

/// Various forwarded/exposed structures from Vulkan/Erupt
pub mod structs {
	pub use erupt::vk::ComponentMapping;
	pub use erupt::vk::Extent2D;
	pub use erupt::vk::ImageSubresourceRange;
	pub use erupt::vk::Offset2D;
	pub use erupt::vk::Rect2D;
}

#[path = "context.rs"]
mod context;
pub use context::Context;

#[path = "general/lib.rs"]
mod general;
pub use general::*;

#[path = "object/lib.rs"]
mod object;
pub use object::*;

/// Instruction related structs.
#[path = "command/lib.rs"]
pub mod command;

/// Vulkan Instance related structs.
#[path = "instance/lib.rs"]
pub mod instance;

/// Physical (GPU) and Logical device related structs.
#[path = "device/lib.rs"]
pub mod device;

/// General-use traits and macros.
#[path = "utility/lib.rs"]
pub mod utility;

/// Structs used in the creation or representation of Pipelines and Pipeline Layouts.
#[path = "pipeline/lib.rs"]
pub mod pipeline;

/// Structs used in the creation or representation of a Render Pass.
#[path = "renderpass/lib.rs"]
pub mod renderpass;

#[path = "shader/lib.rs"]
pub mod shader;
