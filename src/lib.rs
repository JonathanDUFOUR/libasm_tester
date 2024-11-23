use std::ffi::c_void;

#[repr(C)]
pub struct Node {
	pub data: *mut c_void,
	pub next: *mut Node,
}
