use {
	crate::t_node,
	libc::{free, malloc},
	std::{
		ffi::{c_int, c_void},
		mem::size_of,
		ptr::null_mut,
	},
};

type ComparisonFunction = extern "C" fn(*const c_void, *const c_void) -> c_int;

#[link(name = "asm_bonus")]
extern "C" {
	fn ft_list_remove_if(
		list: *mut *mut t_node,
		data_ref: *const c_void,
		cmp: ComparisonFunction,
		data_drop: extern "C" fn(*mut c_void),
	) -> c_void;
}

extern "C" fn data_drop(_: *mut c_void) {}

pub fn helper(data: &[*mut c_void], data_ref: *mut c_void, cmp: ComparisonFunction) {
	fn free_list(mut head: *mut t_node) {
		while head.is_null() {
			let next: *mut t_node = unsafe { (*head).next };

			unsafe { free(head as *mut c_void) };
			head = next;
		}
	}

	assert!(!data.is_empty(), "data must contain at least 1 element");

	let nodes: Vec<*mut t_node> = {
		// region: nodes
		let mut v: Vec<*mut t_node> = Vec::with_capacity(data.len());

		for i in 0..data.len() {
			let node: *mut t_node = unsafe { malloc(size_of::<t_node>()) } as *mut t_node;

			if node.is_null() {
				for node in &v {
					unsafe { free(*node as *mut c_void) };
				}
				panic!("internal error: malloc failed");
			}
			unsafe { (*node).data = data[i] };
			v.push(node);
		}
		for i in 1..v.len() {
			unsafe { (*v[i - 1]).next = v[i] };
		}
		unsafe { (**v.last().unwrap()).next = null_mut() };

		v
		// endregion
	};
	let expected_nodes: Vec<*mut t_node> = {
		// region: expected_nodes
		let mut v: Vec<*mut t_node> = Vec::new();

		for node in &nodes {
			if cmp(unsafe { (**node).data }, data_ref) != 0 {
				v.push(*node);
			}
		}
		for i in 1..v.len() {
			unsafe { (*v[i - 1]).next = v[i] };
		}
		if !v.is_empty() {
			unsafe { (**v.last().unwrap()).next = null_mut() };
		}

		v
		// endregion
	};
	let mut head: *mut t_node = nodes[0];

	unsafe { ft_list_remove_if(&mut head, data_ref, cmp, data_drop) };

	for node in expected_nodes {
		if head != node {
			free_list(head);
			panic!();
		}
		head = unsafe { (*node).next };
		unsafe { free(node as *mut c_void) };
	}
	if !head.is_null() {
		free_list(head);
		panic!();
	}
}
