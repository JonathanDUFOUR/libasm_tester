#[cfg(test)]
mod list_remove_if {
	use {
		libasm_tester::Node,
		libc::{free, malloc},
		std::{
			ffi::{c_int, c_long, c_short, c_uint, c_void, CString},
			mem::size_of,
			ptr::null_mut,
		},
	};

	type ComparisonFunction = unsafe extern "C" fn(*const c_void, *const c_void) -> c_int;

	#[link(name = "asm_bonus")]
	unsafe extern "C" {
		unsafe fn ft_list_remove_if(
			list: *mut *mut Node,
			data_ref: *const c_void,
			cmp: ComparisonFunction,
			data_drop: unsafe extern "C" fn(*mut c_void),
		) -> c_void;
	}

	unsafe extern "C" fn data_drop(_: *mut c_void) {}

	fn helper(data: &[*mut c_void], data_ref: *mut c_void, cmp: ComparisonFunction) {
		fn free_list(mut head: *mut Node) {
			while head.is_null() {
				let next: *mut Node = unsafe { (*head).next };

				unsafe { free(head as *mut c_void) };
				head = next;
			}
		}

		assert!(!data.is_empty(), "data must contain at least 1 element");

		let nodes: Vec<*mut Node> = {
			// region: nodes
			let mut v: Vec<*mut Node> = Vec::with_capacity(data.len());

			for i in 0..data.len() {
				let node: *mut Node = unsafe { malloc(size_of::<Node>()) } as *mut Node;

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
		let expected_nodes: Vec<*mut Node> = {
			// region: expected_nodes
			let mut v: Vec<*mut Node> = Vec::new();

			for node in &nodes {
				if unsafe { cmp((**node).data, data_ref) } != 0 {
					v.push(*node);
				}
			}

			v
			// endregion
		};
		let mut head: *mut Node = nodes[0];

		unsafe { ft_list_remove_if(&mut head, data_ref, cmp, data_drop) };

		for node in expected_nodes {
			if head != node {
				free_list(head);
				panic!();
			}

			let next: *mut Node = unsafe { (*head).next };

			unsafe { free(head as *mut c_void) };
			head = next;
		}
		if !head.is_null() {
			free_list(head);
			panic!();
		}
	}

	// region: comparison functions
	unsafe extern "C" fn always_match(_: *const c_void, _: *const c_void) -> c_int {
		0
	}

	unsafe extern "C" fn never_match(_: *const c_void, _: *const c_void) -> c_int {
		42
	}

	unsafe extern "C" fn match_if_pointer_is_lower(a: *const c_void, b: *const c_void) -> c_int {
		!(a < b) as c_int
	}

	unsafe extern "C" fn match_if_pointer_is_equal(a: *const c_void, b: *const c_void) -> c_int {
		!(a == b) as c_int
	}

	unsafe extern "C" fn match_if_pointer_is_greater(a: *const c_void, b: *const c_void) -> c_int {
		!(a > b) as c_int
	}

	unsafe extern "C" fn match_if_value_is_lower<T: Ord>(
		a: *const c_void,
		b: *const c_void,
	) -> c_int {
		let a: &T = unsafe { &*(a as *const T) };
		let b: &T = unsafe { &*(b as *const T) };

		!(a < b) as c_int
	}

	unsafe extern "C" fn match_if_value_is_equal<T: Eq>(
		a: *const c_void,
		b: *const c_void,
	) -> c_int {
		let a: &T = unsafe { &*(a as *const T) };
		let b: &T = unsafe { &*(b as *const T) };

		!(a == b) as c_int
	}

	unsafe extern "C" fn match_if_value_is_greater<T: Ord>(
		a: *const c_void,
		b: *const c_void,
	) -> c_int {
		let a: &T = unsafe { &*(a as *const T) };
		let b: &T = unsafe { &*(b as *const T) };

		!(a > b) as c_int
	}
	// endregion

	// region: empty_list
	#[test]
	fn empty_list() {
		unsafe extern "C" fn cmp(_: *const c_void, _: *const c_void) -> c_int {
			0
		}
		unsafe extern "C" fn data_drop(_: *mut c_void) {}

		let mut head: *mut Node = null_mut();
		let data_ref: *mut c_void = null_mut();

		unsafe { ft_list_remove_if(&mut head, data_ref, cmp, data_drop) };

		assert_eq!(head, null_mut());
	}
	// endregion
	// region: list_of_1_null_pointer_matching_0
	#[test]
	fn list_of_1_null_pointer_matching_0() {
		helper(&[null_mut()], null_mut(), never_match);
	}
	// endregion
	// region: list_of_1_null_pointer_matching_1
	#[test]
	fn list_of_1_null_pointer_matching_1() {
		helper(&[null_mut()], null_mut(), always_match);
	}
	// endregion
	// region: list_of_4_null_pointers_matching_0
	#[test]
	fn list_of_4_null_pointers_matching_0() {
		helper(&[null_mut(); 4], null_mut(), never_match);
	}
	// endregion
	// region: list_of_7_null_pointers_matching_7
	#[test]
	fn list_of_7_null_pointers_matching_7() {
		helper(&[null_mut(); 7], null_mut(), always_match);
	}
	// endregion
	// region: list_of_1_non_null_pointer_matching_0
	#[test]
	fn list_of_1_non_null_pointer_matching_0() {
		helper(
			&[&mut 73 as *mut _ as *mut c_void],
			&mut 12_345 as *mut _ as *mut c_void,
			never_match,
		);
	}
	// endregion
	// region: list_of_1_non_null_pointer_matching_1
	#[test]
	fn list_of_1_non_null_pointer_matching_1() {
		helper(
			&[&mut 3.14 as *mut _ as *mut c_void],
			&mut 0.5 as *mut _ as *mut c_void,
			always_match,
		);
	}
	// endregion
	// region: list_of_9_non_null_pointers_matching_0
	#[test]
	fn list_of_9_non_null_pointers_matching_0() {
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
	// region: list_of_6_non_null_pointers_matching_6
	#[test]
	fn list_of_6_non_null_pointers_matching_6() {
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
	// region: list_of_8_non_null_pointers_matching_3_at_the_beginning
	#[test]
	fn list_of_8_non_null_pointers_matching_3_at_the_beginning() {
		helper(
			&[
				0x01 as *mut c_void, // <<<
				0x01 as *mut c_void, // <<<
				0x02 as *mut c_void, // <<<
				0x03 as *mut c_void,
				0x05 as *mut c_void,
				0x08 as *mut c_void,
				0x0D as *mut c_void,
				0x15 as *mut c_void,
			],
			0x03 as *mut c_void,
			match_if_pointer_is_lower,
		);
	}
	// endregion
	// region: list_of_5_non_null_pointers_matching_2_in_the_middle
	#[test]
	fn list_of_5_non_null_pointers_matching_2_in_the_middle() {
		helper(
			&[
				0xA319 as *mut c_void,
				0x64F1 as *mut c_void,
				0x20D6 as *mut c_void, // <<<
				0x20D6 as *mut c_void, // <<<
				0xB296 as *mut c_void,
			],
			0x20D6 as *mut c_void,
			match_if_pointer_is_equal,
		);
	}
	// endregion
	// region: list_of_9_non_null_pointers_matching_6_at_the_end
	#[test]
	fn list_of_9_non_null_pointers_matching_6_at_the_end() {
		helper(
			&[
				0x01 as *mut c_void,
				0x01 as *mut c_void,
				0x02 as *mut c_void,
				0x03 as *mut c_void, // <<<
				0x05 as *mut c_void, // <<<
				0x08 as *mut c_void, // <<<
				0x0D as *mut c_void, // <<<
				0x15 as *mut c_void, // <<<
				0x22 as *mut c_void, // <<<
			],
			0x02 as *mut c_void,
			match_if_pointer_is_greater,
		);
	}
	// endregion
	// region: list_of_15_non_null_pointers_matching_3_at_the_beginning_and_4_in_the_middle
	#[test]
	fn list_of_15_non_null_pointers_matching_3_at_the_beginning_and_4_in_the_middle() {
		helper(
			&[
				&mut (42 as c_uint) as *mut _ as *mut c_void, // <<<
				&mut (42 as c_uint) as *mut _ as *mut c_void, // <<<
				&mut (42 as c_uint) as *mut _ as *mut c_void, // <<<
				&mut (41 as c_uint) as *mut _ as *mut c_void,
				&mut (43 as c_uint) as *mut _ as *mut c_void,
				&mut (42 as c_uint) as *mut _ as *mut c_void, // <<<
				&mut (24 as c_uint) as *mut _ as *mut c_void,
				&mut (22 as c_uint) as *mut _ as *mut c_void,
				&mut (44 as c_uint) as *mut _ as *mut c_void,
				&mut (42 as c_uint) as *mut _ as *mut c_void, // <<<
				&mut (21 as c_uint) as *mut _ as *mut c_void,
				&mut (42 as c_uint) as *mut _ as *mut c_void, // <<<
				&mut (42 as c_uint) as *mut _ as *mut c_void, // <<<
				&mut (69 as c_uint) as *mut _ as *mut c_void,
				&mut (33 as c_uint) as *mut _ as *mut c_void,
			],
			&mut (42 as c_uint) as *mut _ as *mut c_void,
			match_if_value_is_equal::<c_uint>,
		);
	}
	// endregion
	// region: list_of_18_non_null_pointers_matching_5_at_the_beginning_and_5_at_the_end
	#[test]
	fn list_of_21_non_null_pointers_matching_5_at_the_beginning_and_5_at_the_end() {
		helper(
			&[
				&mut (-11_111 as c_short) as *mut _ as *mut c_void, // <<<
				&mut (-2_222 as c_short) as *mut _ as *mut c_void,  // <<<
				&mut (-333 as c_short) as *mut _ as *mut c_void,    // <<<
				&mut (-44 as c_short) as *mut _ as *mut c_void,     // <<<
				&mut (-5 as c_short) as *mut _ as *mut c_void,      // <<<
				&mut (0 as c_short) as *mut _ as *mut c_void,
				&mut (5 as c_short) as *mut _ as *mut c_void,
				&mut (44 as c_short) as *mut _ as *mut c_void,
				&mut (333 as c_short) as *mut _ as *mut c_void,
				&mut (2_222 as c_short) as *mut _ as *mut c_void,
				&mut (11_111 as c_short) as *mut _ as *mut c_void,
				&mut (2_222 as c_short) as *mut _ as *mut c_void,
				&mut (333 as c_short) as *mut _ as *mut c_void,
				&mut (44 as c_short) as *mut _ as *mut c_void,
				&mut (5 as c_short) as *mut _ as *mut c_void,
				&mut (0 as c_short) as *mut _ as *mut c_void,
				&mut (-5 as c_short) as *mut _ as *mut c_void, // <<<
				&mut (-44 as c_short) as *mut _ as *mut c_void, // <<<
				&mut (-333 as c_short) as *mut _ as *mut c_void, // <<<
				&mut (-2_222 as c_short) as *mut _ as *mut c_void, // <<<
				&mut (-11_111 as c_short) as *mut _ as *mut c_void, // <<<
			],
			&mut (0 as c_short) as *mut _ as *mut c_void,
			match_if_value_is_lower::<c_short>,
		);
	}
	// endregion
	// region: list_of_22_non_null_pointers_matching_7_in_the_middle_and_2_at_the_end
	#[test]
	fn list_of_23_non_null_pointers_matching_7_in_the_middle_and_2_at_the_end() {
		helper(
			&[
				&mut CString::new("compliance").unwrap() as *mut _ as *mut c_void,
				&mut CString::new("preference").unwrap() as *mut _ as *mut c_void,
				&mut CString::new("freshman").unwrap() as *mut _ as *mut c_void, // <<<
				&mut CString::new("freshman").unwrap() as *mut _ as *mut c_void, // <<<
				&mut CString::new("freshman").unwrap() as *mut _ as *mut c_void, // <<<
				&mut CString::new("scramble").unwrap() as *mut _ as *mut c_void,
				&mut CString::new("steep").unwrap() as *mut _ as *mut c_void,
				&mut CString::new("freshman").unwrap() as *mut _ as *mut c_void, // <<<
				&mut CString::new("palace").unwrap() as *mut _ as *mut c_void,
				&mut CString::new("marine").unwrap() as *mut _ as *mut c_void,
				&mut CString::new("punch").unwrap() as *mut _ as *mut c_void,
				&mut CString::new("freshman").unwrap() as *mut _ as *mut c_void, // <<<
				&mut CString::new("freshman").unwrap() as *mut _ as *mut c_void, // <<<
				&mut CString::new("slice").unwrap() as *mut _ as *mut c_void,
				&mut CString::new("represent").unwrap() as *mut _ as *mut c_void,
				&mut CString::new("tournament").unwrap() as *mut _ as *mut c_void,
				&mut CString::new("freshman").unwrap() as *mut _ as *mut c_void, // <<<
				&mut CString::new("mechanism").unwrap() as *mut _ as *mut c_void,
				&mut CString::new("healthy").unwrap() as *mut _ as *mut c_void,
				&mut CString::new("cancer").unwrap() as *mut _ as *mut c_void,
				&mut CString::new("freshman").unwrap() as *mut _ as *mut c_void, // <<<
				&mut CString::new("freshman").unwrap() as *mut _ as *mut c_void, // <<<
			],
			&mut CString::new("freshman").unwrap() as *mut _ as *mut c_void,
			match_if_value_is_equal::<CString>,
		);
	}
	// endregion
	// region: list_of_42_non_null_pointers_matching_2_at_the_beginning_and_9_in_the_middle_and_4_at_the_end
	#[test]
	fn list_of_42_non_null_pointers_matching_2_at_the_beginning_and_9_in_the_middle_and_4_at_the_end(
	) {
		helper(
			&[
				&mut (921_360_282_394_233_230 as c_long) as *mut _ as *mut c_void, // <<<
				&mut (1_571_612_511_276_684_948 as c_long) as *mut _ as *mut c_void, // <<<
				&mut (-5_708_060_106_852_167_343 as c_long) as *mut _ as *mut c_void,
				&mut (-1_476_480_142_480_198_921 as c_long) as *mut _ as *mut c_void,
				&mut (-8_523_462_311_463_709_172 as c_long) as *mut _ as *mut c_void,
				&mut (-7_246_502_463_767_444_635 as c_long) as *mut _ as *mut c_void,
				&mut (-135_668_454_129_316_970 as c_long) as *mut _ as *mut c_void,
				&mut (-9_058_365_951_619_180_425 as c_long) as *mut _ as *mut c_void,
				&mut (3_562_467_615_391_456_229 as c_long) as *mut _ as *mut c_void, // <<<
				&mut (8_671_971_336_970_510_757 as c_long) as *mut _ as *mut c_void, // <<<
				&mut (-3_250_978_492_703_263_022 as c_long) as *mut _ as *mut c_void,
				&mut (2_906_590_648_220_347_463 as c_long) as *mut _ as *mut c_void, // <<<
				&mut (-2_238_007_579_530_703_383 as c_long) as *mut _ as *mut c_void,
				&mut (-457_748_444_871_943_744 as c_long) as *mut _ as *mut c_void,
				&mut (-2_070_947_719_325_526_716 as c_long) as *mut _ as *mut c_void,
				&mut (-8_508_097_120_030_787_534 as c_long) as *mut _ as *mut c_void,
				&mut (6_279_172_540_919_228_346 as c_long) as *mut _ as *mut c_void, // <<<
				&mut (2_250_136_196_831_276_275 as c_long) as *mut _ as *mut c_void, // <<<
				&mut (-7_642_723_434_123_957_135 as c_long) as *mut _ as *mut c_void,
				&mut (-3_218_426_017_053_640_173 as c_long) as *mut _ as *mut c_void,
				&mut (-5_373_652_984_392_797_276 as c_long) as *mut _ as *mut c_void,
				&mut (5_354_580_256_487_663_234 as c_long) as *mut _ as *mut c_void, // <<<
				&mut (-135_605_237_232_227_971 as c_long) as *mut _ as *mut c_void,
				&mut (-5_712_077_679_499_071_445 as c_long) as *mut _ as *mut c_void,
				&mut (-932_697_172_971_571_919 as c_long) as *mut _ as *mut c_void,
				&mut (-7_682_929_619_038_225_236 as c_long) as *mut _ as *mut c_void,
				&mut (-5_529_991_364_512_982_111 as c_long) as *mut _ as *mut c_void,
				&mut (-6_242_319_863_250_334_451 as c_long) as *mut _ as *mut c_void,
				&mut (-6_273_676_485_751_348_340 as c_long) as *mut _ as *mut c_void,
				&mut (-3_019_121_609_358_016_616 as c_long) as *mut _ as *mut c_void,
				&mut (-3_136_389_838_210_946_005 as c_long) as *mut _ as *mut c_void,
				&mut (-9_008_188_037_258_281_796 as c_long) as *mut _ as *mut c_void,
				&mut (6_828_792_323_936_056_707 as c_long) as *mut _ as *mut c_void, // <<<
				&mut (3_518_870_533_414_385_520 as c_long) as *mut _ as *mut c_void, // <<<
				&mut (3_920_778_999_384_813_829 as c_long) as *mut _ as *mut c_void, // <<<
				&mut (-7_434_320_870_015_286_470 as c_long) as *mut _ as *mut c_void,
				&mut (-7_580_130_461_646_989_212 as c_long) as *mut _ as *mut c_void,
				&mut (-8_787_050_484_318_530_035 as c_long) as *mut _ as *mut c_void,
				&mut (8_992_865_503_538_629_231 as c_long) as *mut _ as *mut c_void, // <<<
				&mut (7_864_456_436_672_958_725 as c_long) as *mut _ as *mut c_void, // <<<
				&mut (3_933_785_492_086_178_377 as c_long) as *mut _ as *mut c_void, // <<<
				&mut (3_047_676_419_120_875_738 as c_long) as *mut _ as *mut c_void, // <<<
			],
			&mut (9_876_543_210 as c_long) as *mut _ as *mut c_void,
			match_if_value_is_greater::<c_long>,
		);
	}
	// endregion
}
