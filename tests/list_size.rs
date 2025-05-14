#[cfg(test)]
mod list_size {
	use {
		libasm_tester::Node,
		std::{
			ffi::{c_int, c_void},
			ptr::null_mut,
		},
	};

	#[link(name = "asm_bonus")]
	unsafe extern "C" {
		unsafe fn ft_list_size(list: *const Node) -> c_int;
	}

	fn helper(data: &[*mut c_void]) {
		assert!(!data.is_empty());

		let mut nodes: Vec<Node> =
			data.iter().map(|data| Node { data: *data, next: null_mut() }).collect();

		for i in 0..nodes.len() - 1 {
			nodes[i].next = &mut nodes[i + 1];
		}
		assert_eq!(unsafe { ft_list_size(nodes.as_ptr()) }, nodes.len() as c_int);
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
			&mut 0x0BD8u32 as *mut _ as *mut c_void,
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
			&mut 17_299u16 as *mut _ as *mut c_void,
			&mut 2_371_597_596u32 as *mut _ as *mut c_void,
			&mut 7_492_647_612_764_487_385u64 as *mut _ as *mut c_void,
			&mut 84_379_664_544_895_219_780_130_836_660_816_119_438u128 as *mut _ as *mut c_void,
			&mut -26i8 as *mut _ as *mut c_void,
			&mut -30_084i16 as *mut _ as *mut c_void,
			&mut -686_473_075i32 as *mut _ as *mut c_void,
			&mut -4_281_136_400_791_984_104i64 as *mut _ as *mut c_void,
			&mut -24_202_283_039_553_026_342_690_646_794_959_372_026i128 as *mut _ as *mut c_void,
			&mut 55_938_292_887.209_505_257_986_582_117_620f32 as *mut _ as *mut c_void,
			&mut 19_194_834_770_157_109_268_519.099_941_875_444_714_699_041f64 as *mut _
				as *mut c_void,
		]);
	}
	// endregion
}
