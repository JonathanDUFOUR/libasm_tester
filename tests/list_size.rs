#[cfg(test)]
mod list_size {
	use {
		libasm_tester::{list_size::helper, t_node},
		std::{
			ffi::{c_int, c_void},
			ptr::null_mut,
		},
	};

	#[link(name = "asm_bonus")]
	extern "C" {
		fn ft_list_size(list: *const t_node) -> c_int;
	}

	// region: empty_list
	#[test]
	fn empty_list() {
		assert_eq!(unsafe { ft_list_size(null_mut()) }, 0);
	}
	// endregion
	// region: list_of_1_null_pointer
	#[test]
	fn list_of_1_null_pointer() {
		helper(&[null_mut()]);
	}
	// endregion
	// region: list_of_2_non_null_pointers
	#[test]
	fn list_of_2_non_null_pointers() {
		helper(&[
			&mut 0xCDu32 as *mut _ as *mut c_void,
			&mut 0xBD8u32 as *mut _ as *mut c_void,
		]);
	}
	// endregion
	// region: list_of_5_cstr
	#[test]
	fn list_of_5_cstr() {
		helper(&[
			&mut c"Hi" as *mut _ as *mut c_void,
			&mut c"there" as *mut _ as *mut c_void,
			&mut c"How" as *mut _ as *mut c_void,
			&mut c"are" as *mut _ as *mut c_void,
			&mut c"you" as *mut _ as *mut c_void,
		]);
	}
	// endregion
	// region: list_of_12_non_null_pointers
	#[test]
	fn list_of_12_non_null_pointers() {
		helper(&[
			&mut 242u8 as *mut _ as *mut c_void,
			&mut 17299u16 as *mut _ as *mut c_void,
			&mut 2371597596u32 as *mut _ as *mut c_void,
			&mut 7492647612764487385u64 as *mut _ as *mut c_void,
			&mut 84379664544895219780130836660816119438u128 as *mut _ as *mut c_void,
			&mut -26i8 as *mut _ as *mut c_void,
			&mut -30084i16 as *mut _ as *mut c_void,
			&mut -686473075i32 as *mut _ as *mut c_void,
			&mut -4281136400791984104i64 as *mut _ as *mut c_void,
			&mut -24202283039553026342690646794959372026i128 as *mut _ as *mut c_void,
			&mut 55938292887.209505257986582117620f32 as *mut _ as *mut c_void,
			&mut 19194834770157109268519.099941875444714699041f64 as *mut _ as *mut c_void,
		]);
	}
	// endregion
}
