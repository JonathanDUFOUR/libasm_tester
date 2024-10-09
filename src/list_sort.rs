use {
	crate::t_node,
	std::{
		ffi::{c_int, c_void},
		ptr::null_mut,
	},
};

type ComparisonFunction = extern "C" fn(*const c_void, *const c_void) -> c_int;

#[link(name = "asm_bonus")]
extern "C" {
	fn ft_list_sort(list: *mut *mut t_node, cmp: ComparisonFunction);
}

pub fn helper(data: &[*mut c_void], cmp: ComparisonFunction) {
	assert!(!data.is_empty(), "data must contain at least 1 element");

	let mut nodes: Vec<t_node> =
		data.iter().map(|data| t_node { data: *data, next: null_mut() }).collect();

	for i in 0..nodes.len() - 1 {
		nodes[i].next = &mut nodes[i + 1];
	}

	let mut head: *mut t_node = nodes.as_mut_ptr();

	unsafe { ft_list_sort(&mut head, cmp) };

	let mut count: usize = 1;
	let mut curr: *const t_node = head;
	let mut next: *const t_node = unsafe { (*curr).next };

	while !next.is_null() {
		assert!(cmp(unsafe { (*curr).data }, unsafe { (*next).data }) <= 0);
		curr = next;
		next = unsafe { (*curr).next };
		count += 1;
	}
	assert_eq!(count, nodes.len());
}
