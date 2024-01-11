use std::ffi::{c_char, c_int, CString};

extern "C" {
	fn ft_atoi_base(s: *const c_char, base: *const c_char) -> c_int;
}

#[inline(always)]
fn unit_test_helper(s: &[u8], base: &[u8], expected: i32) {
	let s: CString = CString::new(s).unwrap();
	let base: CString = CString::new(base).unwrap();

	assert_eq!(
		unsafe { ft_atoi_base(s.as_ptr() as *const c_char, base.as_ptr() as *const c_char) },
		expected
	);
}

// region: ft_atoi_base_00
#[test]
fn ft_atoi_base_00() {
	// invalid base: empty string
	unit_test_helper(&[0xa6, 0xec, 0x9d, 0x4c, 0xa2, 0xc4, 0xd0], &[], 0);
}
// endregion

// region: ft_atoi_base_01
#[test]
fn ft_atoi_base_01() {
	// invalid base: length < 2
	unit_test_helper(&[0x20, 0x01, 0x7d, 0xa2, 0xb3], &[0x01], 0);
}
// endregion

// region: ft_atoi_base_02
#[test]
fn ft_atoi_base_02() {
	// invalid base: contains duplicates
	unit_test_helper(
		&[0x56, 0x7e, 0xb1, 0x9a, 0xd0, 0x80, 0x90, 0x6a],
		&[0x95, 0x4b, 0x56, 0x95],
		0,
	);
}
// endregion

// region: ft_atoi_base_03
#[test]
fn ft_atoi_base_03() {
	// invalid base: contains 0x2b ('+')
	unit_test_helper(&[0x2b, 0x3b, 0xc6, 0xdd], &[0xc6, 0x3b, 0x45, 0x2b, 0x9b], 0);
}
// endregion

// region: ft_atoi_base_04
#[test]
fn ft_atoi_base_04() {
	// invalid base: contains 0x2d ('-')
	unit_test_helper(&[0x0c, 0x0c, 0x2b, 0xa3, 0x20], &[0xb6, 0x2d, 0xb7, 0xa8, 0xa3], 0);
}
// endregion

// region: ft_atoi_base_05
#[test]
fn ft_atoi_base_05() {
	// invalid base: contains 0x09 ('\t')
	unit_test_helper(&[0x09, 0x0c, 0xa1], &[0x09, 0xa1, 0xd0, 0xfc, 0xda, 0xe6, 0x1f, 0xab], 0);
}
// endregion

// region: ft_atoi_base_06
#[test]
fn ft_atoi_base_06() {
	// invalid base: contains 0x0a ('\n')
	unit_test_helper(
		&[0x20, 0x0a, 0x2d, 0x1e, 0xe1],
		&[
			0x38, 0x7b, 0xcf, 0x1e, 0xe1, 0x5b, 0x3c, 0x6b, 0x9e, 0xb4, 0x0a,
		],
		0,
	);
}
// endregion

// region: ft_atoi_base_07
#[test]
fn ft_atoi_base_07() {
	// invalid base: contains 0x0b ('\v')
	unit_test_helper(
		&[0x2b, 0x82, 0xc3, 0x0b, 0x21, 0xfa],
		&[0x82, 0x45, 0xc3, 0xfd, 0x0b, 0xb8, 0x96, 0xa5, 0x49, 0x21],
		0,
	);
}
// endregion

// region: ft_atoi_base_08
#[test]
fn ft_atoi_base_08() {
	// invalid base: contains 0x0c ('\f')
	unit_test_helper(&[0x0a, 0x84, 0xd2, 0xdd], &[0xdd, 0x84, 0xbc, 0xd4, 0x0c, 0xa5], 0);
}
// endregion

// region: ft_atoi_base_09
#[test]
fn ft_atoi_base_09() {
	// invalid base: contains 0x0d ('\r')
	unit_test_helper(
		&[0xce, 0xaa, 0x0d, 0x2d, 0x7d],
		&[
			0xe0, 0x02, 0x61, 0x78, 0x7d, 0x8b, 0xde, 0x74, 0xe1, 0x24, 0xa5, 0x0d, 0x86, 0xaa,
			0x9c, 0xce,
		],
		0,
	);
}
// endregion

// region: ft_atoi_base_10
#[test]
fn ft_atoi_base_10() {
	// invalid base: contains 0x20 (' ')
	unit_test_helper(&[0x20, 0x20, 0xea, 0x20, 0x4a], &[0x20, 0x4d, 0x32, 0xea, 0x4a], 0);
}
// endregion

// region: ft_atoi_base_11
#[test]
fn ft_atoi_base_11() {
	// invalid base: contains 0x2a ('*')
	unit_test_helper(&[0x43, 0x2b, 0xbd], &[0xbd, 0x98, 0xd4, 0x2a, 0x16, 0x97, 0x43], 0);
}
// endregion

// region: ft_atoi_base_12
#[test]
fn ft_atoi_base_12() {
	// invalid base: contains 0x2b ('+')
	unit_test_helper(&[0x2d, 0x2b, 0xce], &[0xce, 0x2b], 0);
}
// endregion

// region: ft_atoi_base_13
#[test]
fn ft_atoi_base_13() {
	// invalid base: contains 0x2d ('-')
	unit_test_helper(&[0x20, 0x2b, 0x2c, 0x2d], &[0x2c, 0x2e, 0x2d, 0x65, 0x30, 0xcf, 0xbc], 0);
}
// endregion

// region: ft_atoi_base_14
#[test]
fn ft_atoi_base_14() {
	// invalid base: contains 0x2f ('/')
	unit_test_helper(&[0x0b, 0x2b, 0x89, 0x2f], &[0xcd, 0x89, 0x2f], 0);
}
// endregion

// region: ft_atoi_base_15
#[test]
fn ft_atoi_base_15() {
	// s: empty string
	unit_test_helper(&[], &[0xb6, 0x3d, 0x21], 0);
}
// endregion

// region: ft_atoi_base_16
#[test]
fn ft_atoi_base_16() {
	// s: only whitespaces
	unit_test_helper(
		&[0x20, 0x0c, 0x09, 0x20, 0x0a, 0x0a, 0x0b, 0x0d],
		&[0xf1, 0x3c, 0x96, 0xc9, 0xa8],
		0,
	);
}
// endregion

// region: ft_atoi_base_17
#[test]
fn ft_atoi_base_17() {
	// s: only 0x2b ('+')
	unit_test_helper(&[0x2b], &[0xb2, 0x99, 0x07], 0);
}
// endregion

// region: ft_atoi_base_18
#[test]
fn ft_atoi_base_18() {
	// s: only 0x2d ('-')
	unit_test_helper(&[0x2d], &[0x46, 0x4f, 0x30], 0);
}
// endregion

// region: ft_atoi_base_19
#[test]
fn ft_atoi_base_19() {
	// s: contains more than 1 0x2b ('+') before the number
	unit_test_helper(&[0x2b, 0x2b, 0x5b, 0x5d], &[0x5b, 0x5d], 0);
}
// endregion

// region: ft_atoi_base_20
#[test]
fn ft_atoi_base_20() {
	// s: contains more than 1 0x2d ('-') before the number
	unit_test_helper(&[0x2d, 0x2d, 0x92], &[0x76, 0x77, 0x92], 0);
}
// endregion

// region: ft_atoi_base_21
#[test]
fn ft_atoi_base_21() {
	// s: contains both 0x2b ('+') and 0x2d ('-') before the number
	unit_test_helper(&[0x2b, 0x2d, 0x41, 0x41], &[0x5a, 0x41], 0);
}
// endregion

// region: ft_atoi_base_22
#[test]
fn ft_atoi_base_22() {
	// s: contains whitespaces between the sign and the number
	unit_test_helper(&[0x2d, 0x20, 0x13, 0x91], &[0x91, 0x13], 0);
}
// endregion

// region: ft_atoi_base_23
#[test]
fn ft_atoi_base_23() {
	// s: contains only null digits
	unit_test_helper(
		&[0xbd, 0xbd, 0xbd, 0xbd, 0xbd, 0xbd, 0xbd, 0xbd],
		&[0xbd, 0x02, 0x8c, 0xac, 0x95, 0x8e, 0x2a, 0x16],
		0,
	);
}
// endregion

// region: ft_atoi_base_24
#[test]
fn ft_atoi_base_24() {
	// s: contains whitespaces between the leading null digits and the signicant digits
	unit_test_helper(
		&[0xbd, 0xbd, 0x0c, 0x20, 0x09, 0x16, 0x22, 0xbd],
		&[0xbd, 0x02, 0x8c, 0xac, 0x95, 0x8e, 0x22, 0x16],
		0,
	);
}
// endregion

// region: ft_atoi_base_25
#[test]
fn ft_atoi_base_25() {
	// s: contains 0x2b ('+') between the leading null digits and the signicant digits
	unit_test_helper(
		&[0xe3, 0xe3, 0x2b, 0x05, 0xf6, 0x16],
		&[
			0xe3, 0x16, 0xf6, 0x96, 0xa2, 0x51, 0xaf, 0x49, 0x05, 0x5e, 0x1f, 0x61,
		],
		0,
	);
}
// endregion

// region: ft_atoi_base_26
#[test]
fn ft_atoi_base_26() {
	// s: contains 0x2d ('-') between the leading null digits and the signicant digits
	unit_test_helper(
		&[0xc4, 0xc4, 0xc4, 0xc4, 0x2d, 0xc9, 0xf4, 0xa3],
		&[0xc4, 0xc3, 0xf4, 0x69, 0x18, 0x96, 0xc9, 0x45, 0xa3, 0x5d],
		0,
	);
}
// endregion

// region: ft_atoi_base_27
#[test]
fn ft_atoi_base_27() {
	// s: contains only significant digits
	unit_test_helper(
		&[0x70, 0x6f, 0x6e, 0x65, 0x79, 0x76, 0x69, 0x66],
		&[0x65, 0x66, 0x69, 0x6e, 0x6f, 0x70, 0x76, 0x79],
		11636625,
	);
}
// endregion
