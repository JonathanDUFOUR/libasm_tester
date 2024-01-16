#[cfg(test)]
mod tests {
	use libasm_tester::t_list;
	use std::{
		ffi::{c_int, c_void},
		ptr::null_mut,
	};

	extern "C" {
		fn ft_list_sort(
			head: *const *const t_list,
			cmp: extern "C" fn(*const c_void, *const c_void) -> c_int,
		);
	}
	extern "C" fn cmp_nothing(_: *const c_void, _: *const c_void) -> c_int {
		0
	}

	#[inline(always)]
	fn unit_test_helper(
		data: &[*mut c_void],
		cmp: extern "C" fn(*const c_void, *const c_void) -> c_int,
	) {
		assert!(!data.is_empty());

		let mut nodes: Vec<t_list> =
			data.iter().map(|data| t_list { data: *data, next: null_mut() }).collect();
		let mut head: *const t_list = nodes.as_ptr();

		for i in 0..nodes.len() - 1 {
			nodes[i].next = &mut nodes[i + 1];
		}
		unsafe { ft_list_sort(&head, cmp) };

		let mut count: usize = 0;

		if !head.is_null() {
			let mut next: *const t_list = unsafe { (*head).next };

			count += 1;
			while !next.is_null() {
				assert!(cmp(unsafe { (*head).data }, unsafe { (*next).data }) <= 0);
				head = next;
				next = unsafe { (*head).next };
				count += 1;
			}
		}
		assert_eq!(count, nodes.len());
	}

	// region: ft_list_sort_00
	#[test]
	fn ft_list_sort_00() {
		let mut head: *const t_list = null_mut();
		let mut count: usize = 0;

		unsafe { ft_list_sort(&head, cmp_nothing) };

		if !head.is_null() {
			let mut next: *const t_list = unsafe { (*head).next };

			count += 1;
			while !next.is_null() {
				head = next;
				next = unsafe { (*head).next };
				count += 1;
			}
		}
		assert_eq!(count, 0);
	}
	// endregion
}
