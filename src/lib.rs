use std::ffi::c_void;

#[derive(Clone, Copy)]
#[repr(C)]
pub struct Node {
	pub data: *mut c_void,
	pub next: *mut Node,
}
