use {
	crate::t_node,
	std::{ffi::c_void, ptr::null_mut},
};

#[link(name = "asm_bonus")]
extern "C" {
	fn ft_list_push_front(list: *mut *const t_node, data: *const c_void) -> ();
}

extern "C" {
	fn free(ptr: *mut c_void);
}

pub fn helper(data: &[*const c_void]) {
	let mut head: *const t_node = null_mut();

	unsafe { ft_list_push_front(&mut head, data[0]) };
	assert!(!head.is_null());
	assert_eq!(unsafe { (*head).data } as *const c_void, data[0]);
	for i0 in 1..data.len() {
		unsafe { ft_list_push_front(&mut head, data[i0]) };
		assert!(!head.is_null());
		assert_eq!(unsafe { (*head).data } as *const c_void, data[i0]);

		let mut current: *const t_node = unsafe { (*head).next };

		for i1 in (0..i0).rev() {
			assert!(!current.is_null());
			assert_eq!(unsafe { (*current).data } as *const c_void, data[i1]);
			current = unsafe { (*current).next };
		}
	}
	while !head.is_null() {
		let next: *const t_node = unsafe { (*head).next };

		unsafe { free(head as *mut c_void) };
		head = next;
	}
}
