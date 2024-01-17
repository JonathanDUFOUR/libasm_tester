#[cfg(test)]
mod tests {
	use std::ffi::{c_char, c_int, CStr};

	#[link(name = "asm")]
	extern "C" {
		fn ft_strcmp(s0: *const c_char, s1: *const c_char) -> c_int;
	}

	#[inline(always)]
	fn unit_test_helper(s0: &str, s1: &str, predicate: fn(c_int) -> bool) {
		let s0: &CStr = CStr::from_bytes_with_nul(s0.as_bytes()).unwrap();
		let s1: &CStr = CStr::from_bytes_with_nul(s1.as_bytes()).unwrap();

		assert!(predicate(unsafe { ft_strcmp(s0.as_ptr(), s1.as_ptr()) }));
	}

	// region: ft_strcmp_00
	#[test]
	fn ft_strcmp_00() {
		unit_test_helper("\0", "\0", |n: c_int| n == 0);
	}
	// endregion

	// region: ft_strcmp_01
	#[test]
	fn ft_strcmp_01() {
		unit_test_helper("\0", "abcdefg\0", |n: c_int| n < 0);
	}
	// endregion

	// region: ft_strcmp_02
	#[test]
	fn ft_strcmp_02() {
		unit_test_helper("hijklmn\0", "\0", |n: c_int| n > 0);
	}
	// endregion

	// region: ft_strcmp_03
	#[test]
	fn ft_strcmp_03() {
		unit_test_helper("d\0", "c\0", |n: c_int| n > 0);
	}
	// endregion

	// region: ft_strcmp_04
	#[test]
	fn ft_strcmp_04() {
		unit_test_helper("42a\0", "42ai\0", |n: c_int| n < 0);
	}
	// endregion

	// region: ft_strcmp_05
	#[test]
	fn ft_strcmp_05() {
		unit_test_helper("19/3=6.\0", "19/3=6\0", |n: c_int| n > 0);
	}
	// endregion

	// region: ft_strcmp_06
	#[test]
	fn ft_strcmp_06() {
		unit_test_helper("Code_Lyoko\0", "Code-Lyoko!\0", |n: c_int| n > 0);
	}
	// endregion

	// region: ft_strcmp_07
	#[test]
	fn ft_strcmp_07() {
		unit_test_helper("Fish&Chip$\0", "Fish&ChipS\0", |n: c_int| n < 0);
	}
	// endregion

	// region: ft_strcmp_08
	#[test]
	fn ft_strcmp_08() {
		unit_test_helper("It's a match!\0", "It's a match!\0", |n: c_int| n == 0);
	}
	// endregion
}
