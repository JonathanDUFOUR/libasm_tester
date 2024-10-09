#[cfg(test)]
mod list_remove_if {
	use {
		libasm_tester::{list_remove_if::helper, t_node},
		std::{
			ffi::{c_int, c_void},
			ptr::null_mut,
		},
	};

	#[link(name = "asm_bonus")]
	extern "C" {
		fn ft_list_remove_if(
			list: *mut *mut t_node,
			data_ref: *const c_void,
			cmp: extern "C" fn(*const c_void, *const c_void) -> c_int,
			data_drop: extern "C" fn(*mut c_void),
		) -> c_void;
	}

	extern "C" fn always_match(_: *const c_void, _: *const c_void) -> c_int {
		0
	}

	extern "C" fn never_match(_: *const c_void, _: *const c_void) -> c_int {
		42
	}

	extern "C" fn is_lower(a: *const c_void, b: *const c_void) -> c_int {
		(a < b) as c_int
	}

	// region: empty_list
	#[test]
	fn empty_list() {
		extern "C" fn cmp(_: *const c_void, _: *const c_void) -> c_int {
			0
		}
		extern "C" fn data_drop(_: *mut c_void) {}

		let mut head: *mut t_node = null_mut();
		let data_ref: *mut c_void = null_mut();

		unsafe { ft_list_remove_if(&mut head, data_ref, cmp, data_drop) };

		assert_eq!(head, null_mut());
	}
	// endregion
	// region: list_of_1_null_pointer_with_0_matching
	#[test]
	fn list_of_1_null_pointer_with_0_matching() {
		helper(&[null_mut()], null_mut(), never_match);
	}
	// endregion
	// region: list_of_1_null_pointer_with_1_matching
	#[test]
	fn list_of_1_null_pointer_with_1_matching() {
		helper(&[null_mut()], null_mut(), always_match);
	}
	// endregion
	// region: list_of_4_null_pointers_with_0_matching
	#[test]
	fn list_of_4_null_pointers_with_0_matching() {
		helper(&[null_mut(); 4], null_mut(), never_match);
	}
	// endregion
	// region: list_of_7_null_pointers_with_7_matching
	#[test]
	fn list_of_7_null_pointers_with_7_matching() {
		helper(&[null_mut(); 7], null_mut(), always_match);
	}
	// endregion
	// region: list_of_1_non_null_pointer_with_0_matching
	#[test]
	fn list_of_1_non_null_pointer_with_0_matching() {
		helper(
			&[&mut 73 as *mut _ as *mut c_void],
			&mut 12345 as *mut _ as *mut c_void,
			never_match,
		);
	}
	// endregion
	// region: list_of_1_non_null_pointer_with_1_matching
	#[test]
	fn list_of_1_non_null_pointer_with_1_matching() {
		helper(
			&[&mut 3.14 as *mut _ as *mut c_void],
			&mut 0.5 as *mut _ as *mut c_void,
			always_match,
		);
	}
	// endregion
	// region: list_of_9_non_null_pointers_with_0_matching
	#[test]
	fn list_of_9_non_null_pointers_with_0_matching() {
		helper(
			&[
				&mut 0u8 as *mut _ as *mut c_void,
				&mut 1u16 as *mut _ as *mut c_void,
				&mut 2u32 as *mut _ as *mut c_void,
				&mut 3u64 as *mut _ as *mut c_void,
				&mut 4u128 as *mut _ as *mut c_void,
				&mut 5.0f32 as *mut _ as *mut c_void,
				&mut 6.0f64 as *mut _ as *mut c_void,
				&mut '7' as *mut _ as *mut c_void,
				&mut "8" as *mut _ as *mut c_void,
			],
			&mut 0u8 as *mut _ as *mut c_void,
			never_match,
		);
	}
	// endregion
	// region: list_of_6_non_null_pointers_with_6_matching
	#[test]
	fn list_of_6_non_null_pointers_with_6_matching() {
		helper(
			&[
				0x01 as *mut c_void,
				0x02 as *mut c_void,
				0x04 as *mut c_void,
				0x08 as *mut c_void,
				0x10 as *mut c_void,
				0x20 as *mut c_void,
			],
			0x40 as *mut c_void,
			always_match,
		);
	}
	// endregion
	// region: list_of_8_non_null_pointers_with_3_matching
	#[test]
	fn list_of_8_non_null_pointers_with_2_matching() {
		helper(
			&[
				0x01 as *mut c_void,
				0x01 as *mut c_void,
				0x02 as *mut c_void,
				0x03 as *mut c_void,
				0x05 as *mut c_void,
				0x08 as *mut c_void,
				0x0D as *mut c_void,
				0x15 as *mut c_void,
			],
			0x07 as *mut c_void,
			is_lower,
		);
	}
	// endregion
}
