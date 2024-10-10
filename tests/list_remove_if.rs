#[cfg(test)]
mod list_remove_if {
	use {
		libasm_tester::{list_remove_if::helper, t_node},
		std::{
			ffi::{c_int, c_long, c_short, c_uint, c_void, CString},
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

	// region: comparison functions
	extern "C" fn always_match(_: *const c_void, _: *const c_void) -> c_int {
		0
	}

	extern "C" fn never_match(_: *const c_void, _: *const c_void) -> c_int {
		42
	}

	extern "C" fn match_if_pointer_is_lower(a: *const c_void, b: *const c_void) -> c_int {
		!(a < b) as c_int
	}

	extern "C" fn match_if_pointer_is_equal(a: *const c_void, b: *const c_void) -> c_int {
		!(a == b) as c_int
	}

	extern "C" fn match_if_pointer_is_greater(a: *const c_void, b: *const c_void) -> c_int {
		!(a > b) as c_int
	}

	extern "C" fn match_if_value_is_lower<T: Ord>(a: *const c_void, b: *const c_void) -> c_int {
		let a: &T = unsafe { &*(a as *const T) };
		let b: &T = unsafe { &*(b as *const T) };

		!(a < b) as c_int
	}

	extern "C" fn match_if_value_is_equal<T: Eq>(a: *const c_void, b: *const c_void) -> c_int {
		let a: &T = unsafe { &*(a as *const T) };
		let b: &T = unsafe { &*(b as *const T) };

		!(a == b) as c_int
	}

	extern "C" fn match_if_value_is_greater<T: Ord>(a: *const c_void, b: *const c_void) -> c_int {
		let a: &T = unsafe { &*(a as *const T) };
		let b: &T = unsafe { &*(b as *const T) };

		!(a > b) as c_int
	}
	// endregion

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
			&mut 12345 as *mut _ as *mut c_void,
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
				0x01 as *mut c_void,
				0x01 as *mut c_void,
				0x02 as *mut c_void,
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
				0x20D6 as *mut c_void,
				0x20D6 as *mut c_void,
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
				0x03 as *mut c_void,
				0x05 as *mut c_void,
				0x08 as *mut c_void,
				0x0D as *mut c_void,
				0x15 as *mut c_void,
				0x22 as *mut c_void,
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
				&mut (42 as c_uint) as *mut _ as *mut c_void,
				&mut (42 as c_uint) as *mut _ as *mut c_void,
				&mut (42 as c_uint) as *mut _ as *mut c_void,
				&mut (2847438268 as c_uint) as *mut _ as *mut c_void,
				&mut (4043705122 as c_uint) as *mut _ as *mut c_void,
				&mut (42 as c_uint) as *mut _ as *mut c_void,
				&mut (660979405 as c_uint) as *mut _ as *mut c_void,
				&mut (3705672050 as c_uint) as *mut _ as *mut c_void,
				&mut (2161945519 as c_uint) as *mut _ as *mut c_void,
				&mut (42 as c_uint) as *mut _ as *mut c_void,
				&mut (1214239939 as c_uint) as *mut _ as *mut c_void,
				&mut (42 as c_uint) as *mut _ as *mut c_void,
				&mut (42 as c_uint) as *mut _ as *mut c_void,
				&mut (4179093640 as c_uint) as *mut _ as *mut c_void,
				&mut (3391391544 as c_uint) as *mut _ as *mut c_void,
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
				&mut (-11111 as c_short) as *mut _ as *mut c_void,
				&mut (-2222 as c_short) as *mut _ as *mut c_void,
				&mut (-333 as c_short) as *mut _ as *mut c_void,
				&mut (-44 as c_short) as *mut _ as *mut c_void,
				&mut (-5 as c_short) as *mut _ as *mut c_void,
				&mut (0 as c_short) as *mut _ as *mut c_void,
				&mut (5 as c_short) as *mut _ as *mut c_void,
				&mut (44 as c_short) as *mut _ as *mut c_void,
				&mut (333 as c_short) as *mut _ as *mut c_void,
				&mut (2222 as c_short) as *mut _ as *mut c_void,
				&mut (11111 as c_short) as *mut _ as *mut c_void,
				&mut (2222 as c_short) as *mut _ as *mut c_void,
				&mut (333 as c_short) as *mut _ as *mut c_void,
				&mut (44 as c_short) as *mut _ as *mut c_void,
				&mut (5 as c_short) as *mut _ as *mut c_void,
				&mut (0 as c_short) as *mut _ as *mut c_void,
				&mut (-5 as c_short) as *mut _ as *mut c_void,
				&mut (-44 as c_short) as *mut _ as *mut c_void,
				&mut (-333 as c_short) as *mut _ as *mut c_void,
				&mut (-2222 as c_short) as *mut _ as *mut c_void,
				&mut (-11111 as c_short) as *mut _ as *mut c_void,
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
				&mut CString::new("Comme toi...").unwrap() as *mut _ as *mut c_void,
				&mut CString::new("Comme toi !").unwrap() as *mut _ as *mut c_void,
				&mut CString::new("Comme toi !").unwrap() as *mut _ as *mut c_void,
				&mut CString::new("Comme toi !").unwrap() as *mut _ as *mut c_void,
				&mut CString::new("...").unwrap() as *mut _ as *mut c_void,
				&mut CString::new("Comme toi que je regarde tout bas").unwrap() as *mut _
					as *mut c_void,
				&mut CString::new("Comme toi qui dort en rêvant à quoi ?").unwrap() as *mut _
					as *mut c_void,
				&mut CString::new("Comme toi...").unwrap() as *mut _ as *mut c_void,
				&mut CString::new("Comme toi !").unwrap() as *mut _ as *mut c_void,
				&mut CString::new("Comme toi !").unwrap() as *mut _ as *mut c_void,
				&mut CString::new("Comme toi !").unwrap() as *mut _ as *mut c_void,
				&mut CString::new("...").unwrap() as *mut _ as *mut c_void,
				&mut CString::new("Elle s'appelait Sarah, elle n'avait pas huit ans").unwrap()
					as *mut _ as *mut c_void,
				&mut CString::new("Sa vie c'était douceur, rêve et nuages blancs").unwrap()
					as *mut _ as *mut c_void,
				&mut CString::new("Mais d'autres gens en avaient décidé autrement").unwrap()
					as *mut _ as *mut c_void,
				&mut CString::new("Elle avait tes yeux clairs, et elle avait ton âge").unwrap()
					as *mut _ as *mut c_void,
				&mut CString::new("C'était une petite fille sans histoire et très sage").unwrap()
					as *mut _ as *mut c_void,
				&mut CString::new("Mais elle n'est pas née comme toi, ici et maintenant").unwrap()
					as *mut _ as *mut c_void,
				&mut CString::new("Comme toiii !...").unwrap() as *mut _ as *mut c_void,
				&mut CString::new("Comme toii !").unwrap() as *mut _ as *mut c_void,
				&mut CString::new("Comme toi !").unwrap() as *mut _ as *mut c_void,
				&mut CString::new("Comme toi !").unwrap() as *mut _ as *mut c_void,
			],
			&mut CString::new("Comme toi !").unwrap() as *mut _ as *mut c_void,
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
				&mut (921360282394233230 as c_long) as *mut _ as *mut c_void,
				&mut (1571612511276684948 as c_long) as *mut _ as *mut c_void,
				&mut (-5708060106852167343 as c_long) as *mut _ as *mut c_void,
				&mut (-1476480142480198921 as c_long) as *mut _ as *mut c_void,
				&mut (-8523462311463709172 as c_long) as *mut _ as *mut c_void,
				&mut (-7246502463767444635 as c_long) as *mut _ as *mut c_void,
				&mut (-135668454129316970 as c_long) as *mut _ as *mut c_void,
				&mut (-9058365951619180425 as c_long) as *mut _ as *mut c_void,
				&mut (3562467615391456229 as c_long) as *mut _ as *mut c_void,
				&mut (8671971336970510757 as c_long) as *mut _ as *mut c_void,
				&mut (-3250978492703263022 as c_long) as *mut _ as *mut c_void,
				&mut (2906590648220347463 as c_long) as *mut _ as *mut c_void,
				&mut (-2238007579530703383 as c_long) as *mut _ as *mut c_void,
				&mut (-457748444871943744 as c_long) as *mut _ as *mut c_void,
				&mut (-2070947719325526716 as c_long) as *mut _ as *mut c_void,
				&mut (-8508097120030787534 as c_long) as *mut _ as *mut c_void,
				&mut (6279172540919228346 as c_long) as *mut _ as *mut c_void,
				&mut (2250136196831276275 as c_long) as *mut _ as *mut c_void,
				&mut (-7642723434123957135 as c_long) as *mut _ as *mut c_void,
				&mut (-3218426017053640173 as c_long) as *mut _ as *mut c_void,
				&mut (-5373652984392797276 as c_long) as *mut _ as *mut c_void,
				&mut (5354580256487663234 as c_long) as *mut _ as *mut c_void,
				&mut (-135605237232227971 as c_long) as *mut _ as *mut c_void,
				&mut (-5712077679499071445 as c_long) as *mut _ as *mut c_void,
				&mut (-932697172971571919 as c_long) as *mut _ as *mut c_void,
				&mut (-7682929619038225236 as c_long) as *mut _ as *mut c_void,
				&mut (-5529991364512982111 as c_long) as *mut _ as *mut c_void,
				&mut (-6242319863250334451 as c_long) as *mut _ as *mut c_void,
				&mut (-6273676485751348340 as c_long) as *mut _ as *mut c_void,
				&mut (-3019121609358016616 as c_long) as *mut _ as *mut c_void,
				&mut (-3136389838210946005 as c_long) as *mut _ as *mut c_void,
				&mut (-9008188037258281796 as c_long) as *mut _ as *mut c_void,
				&mut (6828792323936056707 as c_long) as *mut _ as *mut c_void,
				&mut (3518870533414385520 as c_long) as *mut _ as *mut c_void,
				&mut (3920778999384813829 as c_long) as *mut _ as *mut c_void,
				&mut (-7434320870015286470 as c_long) as *mut _ as *mut c_void,
				&mut (-7580130461646989212 as c_long) as *mut _ as *mut c_void,
				&mut (-8787050484318530035 as c_long) as *mut _ as *mut c_void,
				&mut (8992865503538629231 as c_long) as *mut _ as *mut c_void,
				&mut (7864456436672958725 as c_long) as *mut _ as *mut c_void,
				&mut (3933785492086178377 as c_long) as *mut _ as *mut c_void,
				&mut (3047676419120875738 as c_long) as *mut _ as *mut c_void,
			],
			&mut (9876543210 as c_long) as *mut _ as *mut c_void,
			match_if_value_is_greater::<c_long>,
		);
	}
	// endregion
}
