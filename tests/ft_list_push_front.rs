#[cfg(test)]
mod tests {
	use libasm_tester::t_list;
	use std::{
		ffi::c_void,
		ptr::{null, null_mut},
	};

	#[link(name = "asm_bonus")]
	extern "C" {
		fn ft_list_push_front(list: *mut *const t_list, data: *const c_void) -> ();
		fn free(ptr: *mut c_void);
	}

	#[inline(always)]
	fn unit_test_helper(data: &[*const c_void]) {
		let mut head: *const t_list = null_mut();

		for i0 in 0..data.len() {
			unsafe { ft_list_push_front(&mut head, data[i0]) };
			assert!(!head.is_null());
			assert_eq!(unsafe { (*head).data } as *const c_void, data[i0]);

			let mut current: *const t_list = unsafe { (*head).next };

			for i1 in (0..i0).rev() {
				assert!(!current.is_null());
				assert_eq!(unsafe { (*current).data } as *const c_void, data[i1]);
				current = unsafe { (*current).next };
			}
		}
		while !head.is_null() {
			let next: *const t_list = unsafe { (*head).next };

			unsafe { free(head as *mut c_void) };
			head = next;
		}
	}

	// region: ft_list_push_front_00
	#[test]
	fn ft_list_push_front_00() {
		unit_test_helper(&[null()]);
	}
	// endregion

	// region: ft_list_push_front_01
	#[test]
	fn ft_list_push_front_01() {
		unit_test_helper(&[42 as *const c_void]);
	}
	// endregion

	// region: ft_list_push_front_02
	#[test]
	fn ft_list_push_front_02() {
		unit_test_helper(&[
			&0x26c7ba39u32 as *const _ as *const c_void,
			&0xcb1c4b02u32 as *const _ as *const c_void,
			&0x5f45f4cbu32 as *const _ as *const c_void,
		]);
	}
	// endregion

	// region: ft_list_push_front_03
	#[test]
	fn ft_list_push_front_03() {
		unit_test_helper(&[
			"foo" as *const _ as *const c_void,
			"bar" as *const _ as *const c_void,
			"muf" as *const _ as *const c_void,
			"liz" as *const _ as *const c_void,
			"kro" as *const _ as *const c_void,
			"awe" as *const _ as *const c_void,
			"miu" as *const _ as *const c_void,
			"odj" as *const _ as *const c_void,
			"err" as *const _ as *const c_void,
		]);
	}
	// endregion

	// region: ft_list_push_front_04
	#[test]
	fn ft_list_push_front_04() {
		unit_test_helper(&[
			&0x00i8 as *const _ as *const c_void,
			&0x1111i16 as *const _ as *const c_void,
			&0x22222222i32 as *const _ as *const c_void,
			&0x3333333333333333i64 as *const _ as *const c_void,
			&0x44444444444444444444444444444444i128 as *const _ as *const c_void,
			&0x55u8 as *const _ as *const c_void,
			&0x6666u16 as *const _ as *const c_void,
			&0x77777777u32 as *const _ as *const c_void,
			&0x8888888888888888u64 as *const _ as *const c_void,
			&0x99999999999999999999999999999999u128 as *const _ as *const c_void,
			&f32::INFINITY as *const _ as *const c_void,
			&f64::NEG_INFINITY as *const _ as *const c_void,
			&String::from("I'll be back!") as *const _ as *const c_void,
		]);
	}
	// endregion
}
