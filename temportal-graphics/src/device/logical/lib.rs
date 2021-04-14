#[path = "device.rs"]
mod device;
pub use device::Device;

#[path = "info.rs"]
mod info;
pub use info::DeviceQueue;
pub use info::Info;
