use crate::backend;

pub type CStrPtr = *const ::std::os::raw::c_char;

pub fn to_cstr_ptr(name: &String) -> CStrPtr {
	to_cstr(name.as_str())
}

pub fn to_cstr(name: &str) -> CStrPtr {
	name as *const str as CStrPtr
}

pub fn make_version(major: u32, minor: u32, patch: u32) -> u32 {
	backend::vk::make_version(major, minor, patch)
}

pub fn as_version_string(version: &u32) -> String {
	format!(
		"{}.{}.{}",
		backend::vk::version_major(*version),
		backend::vk::version_minor(*version),
		backend::vk::version_patch(*version)
	)
}
