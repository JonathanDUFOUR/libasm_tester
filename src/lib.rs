use std::ffi::c_void;

pub mod atoi_base;
pub mod list_push_front;
pub mod list_remove_if;
pub mod list_size;
pub mod list_sort;
pub mod memcpy;
pub mod read;
pub mod strcmp;
pub mod strcpy;
pub mod strdup;
pub mod strlen;
pub mod write;

#[repr(C)]
pub struct t_node {
	pub data: *mut c_void,
	pub next: *mut t_node,
}
