#[cfg(test)]
mod tests {
	use libasm_tester::t_list;
	use std::{
		cmp::Ordering,
		ffi::{c_char, c_int, c_void},
		ptr::null_mut,
	};

	#[link(name = "asm")]
	extern "C" {
		fn ft_list_sort(
			head: *const *const t_list,
			cmp: extern "C" fn(*const c_void, *const c_void) -> c_int,
		);
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
		extern "C" fn cmp(_: *const c_void, _: *const c_void) -> c_int {
			0
		}

		let mut head: *const t_list = null_mut();
		let mut count: usize = 0;

		unsafe { ft_list_sort(&head, cmp) };

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

	// region: ft_list_sort_01
	#[test]
	fn ft_list_sort_01() {
		extern "C" fn cmp(a: *const c_void, b: *const c_void) -> c_int {
			let a: &u8 = &unsafe { *(a as *const u8) };
			let b: &u8 = &unsafe { *(b as *const u8) };

			match a.cmp(b) {
				Ordering::Less => -1,
				Ordering::Equal => 0,
				Ordering::Greater => 1,
			}
		}

		unit_test_helper(&[&mut 37u8 as *mut _ as *mut c_void], cmp);
	}
	// endregion

	// region: ft_list_sort_02
	#[test]
	fn ft_list_sort_02() {
		extern "C" fn cmp(a: *const c_void, b: *const c_void) -> c_int {
			let a: &u16 = &unsafe { *(a as *const u16) };
			let b: &u16 = &unsafe { *(b as *const u16) };

			match a.cmp(b) {
				Ordering::Less => -1,
				Ordering::Equal => 0,
				Ordering::Greater => 1,
			}
		}

		unit_test_helper(
			&[
				&mut 7u16 as *mut _ as *mut c_void,
				&mut 6u16 as *mut _ as *mut c_void,
				&mut 5u16 as *mut _ as *mut c_void,
				&mut 4u16 as *mut _ as *mut c_void,
				&mut 3u16 as *mut _ as *mut c_void,
				&mut 2u16 as *mut _ as *mut c_void,
				&mut 1u16 as *mut _ as *mut c_void,
				&mut 0u16 as *mut _ as *mut c_void,
			],
			cmp,
		);
	}
	// endregion

	// region: ft_list_sort_03
	#[test]
	fn ft_list_sort_03() {
		extern "C" {
			fn strcmp(s0: *const c_char, s1: *const c_char) -> c_int;
		}
		extern "C" fn cmp(a: *const c_void, b: *const c_void) -> c_int {
			unsafe { strcmp(a as *const c_char, b as *const c_char) }
		}

		unit_test_helper(
			&[
				String::from("Hello there!").as_mut_ptr() as *mut c_void,
				String::from("General Kenobi!").as_mut_ptr() as *mut c_void,
				String::from("You are a bold one.").as_mut_ptr() as *mut c_void,
				String::from("I find your lack of faith disturbing.").as_mut_ptr() as *mut c_void,
				String::from("I am the Senate.").as_mut_ptr() as *mut c_void,
				String::from("May the Force be with you.").as_mut_ptr() as *mut c_void,
				String::from("I have the high ground.").as_mut_ptr() as *mut c_void,
				String::from("It's over Anakin!").as_mut_ptr() as *mut c_void,
				String::from("Don't try it.").as_mut_ptr() as *mut c_void,
				String::from("You underestimate my power!").as_mut_ptr() as *mut c_void,
				String::from("You were the chosen one!").as_mut_ptr() as *mut c_void,
				String::from("I hate you!").as_mut_ptr() as *mut c_void,
				String::from("You were my brother Anakin!").as_mut_ptr() as *mut c_void,
				String::from("I loved you!").as_mut_ptr() as *mut c_void,
				String::from("I HATE YOU!").as_mut_ptr() as *mut c_void,
				String::from("Hello there!").as_mut_ptr() as *mut c_void,
				String::from("Don't try it.").as_mut_ptr() as *mut c_void,
				String::from("Hello there!").as_mut_ptr() as *mut c_void,
			],
			cmp,
		);
	}
	// endregion
}
