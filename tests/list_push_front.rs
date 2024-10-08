#[cfg(test)]
mod list_push_front {
	use {
		libasm_tester::list_push_front::helper,
		std::{ffi::c_void, ptr::null},
	};

	// region: ft_list_push_front_00
	#[test]
	fn ft_list_push_front_00() {
		helper(&[null()]);
	}
	// endregion
	// region: ft_list_push_front_01
	#[test]
	fn ft_list_push_front_01() {
		helper(&[42 as *const c_void]);
	}
	// endregion
	// region: ft_list_push_front_02
	#[test]
	fn ft_list_push_front_02() {
		helper(&[
			&0xBA39u32 as *const _ as *const c_void,
			&0x4B02u32 as *const _ as *const c_void,
			&0x00CBu32 as *const _ as *const c_void,
		]);
	}
	// endregion
	// region: ft_list_push_front_03
	#[test]
	fn ft_list_push_front_03() {
		helper(&[
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
		helper(&[
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
