use crate::t_node;
use std::{
	ffi::{c_int, c_void},
	ptr::null_mut,
};

#[link(name = "asm_bonus")]
extern "C" {
	fn ft_list_size(list: *const t_node) -> c_int;
}

#[inline(always)]
pub fn helper(data: &[*mut c_void]) {
	assert!(!data.is_empty());

	let mut nodes: Vec<t_node> =
		data.iter().map(|data| t_node { data: *data, next: null_mut() }).collect();

	for i in 0..nodes.len() - 1 {
		nodes[i].next = &mut nodes[i + 1];
	}
	assert_eq!(unsafe { ft_list_size(nodes.as_ptr()) }, nodes.len() as c_int);
}
