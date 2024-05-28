#[cfg(test)]
mod tests {
	use std::ffi::{c_char, c_int};

	#[link(name = "asm")]
	extern "C" {
		fn ft_strcmp(s0: *const c_char, s1: *const c_char) -> c_int;
	}

	#[inline(always)]
	fn helper(s0: &str, s1: &str) {
		use std::{
			cmp::Ordering::{Equal, Greater, Less},
			ffi::CString,
		};

		let s0: CString = CString::new(s0).unwrap();
		let s1: CString = CString::new(s1).unwrap();
		let p0: *const c_char = s0.as_ptr();
		let p1: *const c_char = s1.as_ptr();

		match s0.cmp(&s1) {
			Equal => {
				assert!(unsafe { ft_strcmp(p0, p1) } == 0);
				assert!(unsafe { ft_strcmp(p1, p0) } == 0);
			}
			Less => {
				assert!(unsafe { ft_strcmp(p0, p1) } < 0);
				assert!(unsafe { ft_strcmp(p1, p0) } > 0);
			}
			Greater => {
				assert!(unsafe { ft_strcmp(p0, p1) } > 0);
				assert!(unsafe { ft_strcmp(p1, p0) } < 0);
			}
		}
	}

	// region: ft_strcmp_00
	#[test]
	fn ft_strcmp_00() {
		helper("", "");
	}
	// endregion

	// region: ft_strcmp_01
	#[test]
	fn ft_strcmp_01() {
		helper("", "A");
	}
	// endregion

	// region: ft_strcmp_02
	#[test]
	fn ft_strcmp_02() {
		helper("A", "A");
	}
	// endregion

	// region: ft_strcmp_03
	#[test]
	fn ft_strcmp_03() {
		helper("A", "B");
	}
	// endregion

	// region: ft_strcmp_04
	#[test]
	fn ft_strcmp_04() {
		helper("A", "AA");
	}
	// endregion

	// region: ft_strcmp_05
	#[test]
	fn ft_strcmp_05() {
		helper("A", "BA");
	}
	// endregion

	// region: ft_strcmp_06
	#[test]
	fn ft_strcmp_06() {
		helper("B", "AA");
	}
	// endregion

	// region: ft_strcmp_07
	#[test]
	fn ft_strcmp_07() {
		helper("0123456789ABCDEF", "0123456789abcdef");
	}
	// endregion

	// region: ft_strcmp_08
	#[test]
	fn ft_strcmp_08() {
		helper("It's a Trap!", "It's a tRAP!");
	}
	// endregion

	// region: ft_strcmp_09
	#[test]
	fn ft_strcmp_09() {
		helper(
			"What if the strings to compare are a little longer but still match?",
			"What if the strings to compare are a little longer but still match?",
		);
	}
	// endregion

	// region: ft_strcmp_10
	#[test]
	fn ft_strcmp_10() {
		helper(
			"And if they don't, but very very lately, would it still work? Or is it about to fail?!
			Let's see!",
			"And if they don't, but very very lately, would it still work? Or is it about to fail?!
			Let's see.",
		);
	}
	// endregion

	// region: ft_strcmp_11
	#[test]
	fn ft_strcmp_11() {
		helper("At least once, we should check", "when the strings are completely different");
	}
	// endregion

	// region: ft_strcmp_12
	#[test]
	fn ft_strcmp_12() {
		helper(
			"Even if the strings are the same, but the length is different, it should still work",
			"Even if the strings are the same, but the length is different, it should still work.",
		);
	}
	// endregion

	// region: ft_strcmp_13
	#[test]
	fn ft_strcmp_13() {
		helper(
			"Finally, let's check if the function is not too slow when comparing two identical
			strings that are very very very very very very very very very very very very very very
			very very very very very very very very very very very very very very very very very
			very very very very very very very very very very very very very very very very very
			very very very very very very very very very very very very very very very very very
			very very very very very very very very very very very very very very very very very
			very very very very very very very very very very very very very very very very very
			very very very very very very very very very very very very very very very very very
			very very very very very very very very very very very very very very very very very
			very very very very very very very very very very very very very very very very very
			very very very very very very very very very very very very very very very very very
			very very very very very very very very very very very very very very very very very
			very very very very very very very very very very very very very very very very very
			very very very very very very very very very very very very very very very very very
			very very very very very very very very very very very very very very very very very
			very very very very very very very very very very very very very very very very very
			very very very very very very very very very very very very very very very very very
			very very very very very very very very very very very very very very very very very
			very very very very very very very very very very very very very very very very very
			very very very very very very very very very very very very very very very very very
			very very very very very very very very very very very very very very very very very
			very very very very very very very very very very very very very very very very very
			very very very very very very very very very very very very very very very very very
			very very very very very very very very very very very very very very very very very
			very very very very very very very very very very very very very very very very very
			very very very very very very very very very very very very very very very very very
			very very very very very very very very very very very very very very very very very
			very very very very very very very very very very very very very very very very very
			very very very very very very very very very very very very very very very very very
			very very very very very very very very very very very very very very very very very
			very very very very very very very very very very very very very very very very very
			very very very very very very very very very very very very very very very very very
			very very very very very very very very very very very very very very very very very
			very very very very very very very very very very very very very very very very very
			long",
			"Finally, let's check if the function is not too slow when comparing two identical
			strings that are very very very very very very very very very very very very very very
			very very very very very very very very very very very very very very very very very
			very very very very very very very very very very very very very very very very very
			very very very very very very very very very very very very very very very very very
			very very very very very very very very very very very very very very very very very
			very very very very very very very very very very very very very very very very very
			very very very very very very very very very very very very very very very very very
			very very very very very very very very very very very very very very very very very
			very very very very very very very very very very very very very very very very very
			very very very very very very very very very very very very very very very very very
			very very very very very very very very very very very very very very very very very
			very very very very very very very very very very very very very very very very very
			very very very very very very very very very very very very very very very very very
			very very very very very very very very very very very very very very very very very
			very very very very very very very very very very very very very very very very very
			very very very very very very very very very very very very very very very very very
			very very very very very very very very very very very very very very very very very
			very very very very very very very very very very very very very very very very very
			very very very very very very very very very very very very very very very very very
			very very very very very very very very very very very very very very very very very
			very very very very very very very very very very very very very very very very very
			very very very very very very very very very very very very very very very very very
			very very very very very very very very very very very very very very very very very
			very very very very very very very very very very very very very very very very very
			very very very very very very very very very very very very very very very very very
			very very very very very very very very very very very very very very very very very
			very very very very very very very very very very very very very very very very very
			very very very very very very very very very very very very very very very very very
			very very very very very very very very very very very very very very very very very
			very very very very very very very very very very very very very very very very very
			very very very very very very very very very very very very very very very very very
			very very very very very very very very very very very very very very very very very
			very very very very very very very very very very very very very very very very very
			long",
		);
	}
	// endregion
}
