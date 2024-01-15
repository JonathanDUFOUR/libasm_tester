use std::ffi::c_void;

#[repr(C)]
pub struct t_list {
	pub data: *mut c_void,
	pub next: *mut t_list,
}
