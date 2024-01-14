// use crate::t_list;
use std::{
	ffi::{c_void, CString},
	ptr::{null, null_mut},
};

#[repr(C)]
pub struct t_list {
	pub data: *mut c_void,
	pub next: *mut t_list,
}

extern "C" {
	fn ft_list_push_front(list: *mut *const t_list, data: *const c_void) -> ();
	fn free(ptr: *mut c_void);
}

#[inline(always)]
fn clear_list(mut head: *const t_list) {
	while !head.is_null() {
		let next: *const t_list = unsafe { (*head).next };

		unsafe { free(head as *mut c_void) };
		head = next;
	}
}

#[test]
fn ft_list_push_front_00() {
	let mut head: *const t_list = null_mut();
	let data: *const c_void = null();

	unsafe { ft_list_push_front(&mut head, data) };
	assert!(!head.is_null());
	assert_eq!(unsafe { (*head).data } as *const c_void, data);
	assert!(unsafe { (*head).next }.is_null());

	clear_list(head);
}

#[test]
fn ft_list_push_front_01() {
	let mut head: *const t_list = null_mut();
	let data: *const c_void = CString::new("Hello Rust!").unwrap().into_raw() as *const c_void;

	unsafe { ft_list_push_front(&mut head, data) };
	assert!(!head.is_null());
	assert_eq!(unsafe { (*head).data } as *const c_void, data);
	assert!(unsafe { (*head).next }.is_null());

	clear_list(head);
}

#[test]
fn ft_list_push_front_02() {
	// TODO
}
