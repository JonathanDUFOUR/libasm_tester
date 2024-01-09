use std::ffi::{c_char, c_int, CString};

extern "C" {
	fn ft_strcmp(s0: *const c_char, s1: *const c_char) -> c_int;
}

#[inline(always)]
fn unit_test_helper(s0: &str, s1: &str, predicate: fn(c_int) -> bool) {
	let s0: CString = CString::new(s0).unwrap();
	let s1: CString = CString::new(s1).unwrap();

	assert!(predicate(unsafe {
		ft_strcmp(s0.as_ptr() as *const c_char, s1.as_ptr() as *const c_char)
	}));
}

// region: ft_strcmp_00
#[test]
fn ft_strcmp_00() {
	unit_test_helper("", "", |n: c_int| n == 0);
}
// endregion

// region: ft_strcmp_01
#[test]
fn ft_strcmp_01() {
	unit_test_helper("", "abcdefg", |n: c_int| n < 0);
}
// endregion

// region: ft_strcmp_02
#[test]
fn ft_strcmp_02() {
	unit_test_helper("hijklmn", "", |n: c_int| n > 0);
}
// endregion

// region: ft_strcmp_03
#[test]
fn ft_strcmp_03() {
	unit_test_helper("d", "c", |n: c_int| n > 0);
}
// endregion

// region: ft_strcmp_04
#[test]
fn ft_strcmp_04() {
	unit_test_helper("42a", "42ai", |n: c_int| n < 0);
}
// endregion

// region: ft_strcmp_05
#[test]
fn ft_strcmp_05() {
	unit_test_helper("19/3=6.", "19/3=6", |n: c_int| n > 0);
}
// endregion

// region: ft_strcmp_06
#[test]
fn ft_strcmp_06() {
	unit_test_helper("Code_Lyoko", "Code-Lyoko!", |n: c_int| n > 0);
}
// endregion

// region: ft_strcmp_07
#[test]
fn ft_strcmp_07() {
	unit_test_helper("Fish&Chip$", "Fish&ChipS", |n: c_int| n < 0);
}
// endregion

// region: ft_strcmp_08
#[test]
fn ft_strcmp_08() {
	unit_test_helper("It's a match!", "It's a match!", |n: c_int| n == 0);
}
// endregion
