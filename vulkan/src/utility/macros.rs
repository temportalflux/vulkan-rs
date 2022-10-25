use crate::backend;

pub type CStrPtr = *const ::std::os::raw::c_char;

pub fn to_cstr_ptr(name: &String) -> CStrPtr {
	to_cstr(name.as_str())
}

pub fn to_cstr(name: &str) -> CStrPtr {
	name as *const str as CStrPtr
}

// TODO: support the semver crate
pub fn make_version(major: u64, minor: u64, patch: u64) -> u32 {
	backend::vk::make_api_version(0u32, major as u32, minor as u32, patch as u32)
}

pub fn as_version_string(version: &u32) -> String {
	format!(
		"{}.{}.{}",
		backend::vk::api_version_major(*version),
		backend::vk::api_version_minor(*version),
		backend::vk::api_version_patch(*version)
	)
}
