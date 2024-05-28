#[cfg(test)]
mod tests {
	use std::ffi::c_char;

	use rand::{thread_rng, Rng};

	#[link(name = "asm")]
	extern "C" {
		pub fn ft_strcpy(dst: *mut c_char, src: *const c_char) -> *mut c_char;
	}

	#[inline(always)]
	fn helper(dst_size: usize, src: &str) {
		use {rand::distributions::Standard, std::ffi::CString};
		assert!(dst_size > src.len(), "`src` is too large for `dst`");

		let src_size: usize = src.len() + 1;
		let mut dst: Vec<u8> = thread_rng().sample_iter(Standard).take(dst_size).collect();
		let initial_last_bytes: Vec<u8> = dst[src_size..].to_vec();
		let src: Vec<u8> = CString::new(src).unwrap().into_bytes_with_nul();

		assert_eq!(dst.as_ptr() as *const c_char, unsafe {
			ft_strcpy(dst.as_mut_ptr() as *mut c_char, src.as_ptr() as *const c_char)
		});
		assert_eq!(dst[..src_size], src);
		assert_eq!(dst[src_size..], initial_last_bytes);
	}

	// region: ft_strcpy_00
	#[test]
	fn ft_strcpy_00() {
		helper(1, "");
	}
	// endregion

	// region: ft_strcpy_01
	#[test]
	fn ft_strcpy_01() {
		helper(2, "")
	}
	// endregion

	// region: ft_strcpy_02
	#[test]
	fn ft_strcpy_02() {
		helper(2, "A");
	}
	// endregion

	// region: ft_strcpy_03
	#[test]
	fn ft_strcpy_03() {
		helper(3, "");
	}
	// endregion

	// region: ft_strcpy_04
	#[test]
	fn ft_strcpy_04() {
		helper(3, "C");
	}
	// endregion

	// region: ft_strcpy_05
	#[test]
	fn ft_strcpy_05() {
		helper(3, "DC");
	}
	// endregion

	// region: ft_strcpy_06
	#[test]
	fn ft_strcpy_06() {
		helper(42, "The");
	}
	// endregion

	// region: ft_strcpy_07
	#[test]
	fn ft_strcpy_07() {
		helper(42, "true");
	}
	// endregion

	// region: ft_strcpy_08
	#[test]
	fn ft_strcpy_08() {
		helper(42, "power");
	}
	// endregion

	// region: ft_strcpy_09
	#[test]
	fn ft_strcpy_09() {
		helper(42, "of the");
	}
	// endregion

	// region: ft_strcpy_10
	#[test]
	fn ft_strcpy_10() {
		helper(42, "arcanes");
	}
	// endregion

	// region: ft_strcpy_11
	#[test]
	fn ft_strcpy_11() {
		helper(42, "can't be");
	}
	// endregion

	// region: ft_strcpy_12
	#[test]
	fn ft_strcpy_12() {
		helper(42, "controled");
	}
	// endregion

	// region: ft_strcpy_13
	#[test]
	fn ft_strcpy_13() {
		helper(42, "by a human");
	}
	// endregion

	// region: ft_strcpy_14
	#[test]
	fn ft_strcpy_14() {
		helper(42, "However, if");
	}
	// endregion

	// region: ft_strcpy_15
	#[test]
	fn ft_strcpy_15() {
		helper(42, "you find the");
	}
	// endregion

	// region: ft_strcpy_16
	#[test]
	fn ft_strcpy_16() {
		helper(42, "elder scrolls");
	}
	// endregion

	// region: ft_strcpy_17
	#[test]
	fn ft_strcpy_17() {
		helper(42, "and if you ask");
	}
	// endregion

	// region: ft_strcpy_18
	#[test]
	fn ft_strcpy_18() {
		helper(42, "the dragons for");
	}
	// endregion

	// region: ft_strcpy_19
	#[test]
	fn ft_strcpy_19() {
		helper(42, "help, maybe they");
	}
	// endregion

	// region: ft_strcpy_20
	#[test]
	fn ft_strcpy_20() {
		helper(42, "will accept to do");
	}
	// endregion

	// region: ft_strcpy_21
	#[test]
	fn ft_strcpy_21() {
		helper(42, "the translation of");
	}
	// endregion

	// region: ft_strcpy_22
	#[test]
	fn ft_strcpy_22() {
		helper(42, "the writings on the");
	}
	// endregion

	// region: ft_strcpy_23
	#[test]
	fn ft_strcpy_23() {
		helper(42, "scrolls and then you");
	}
	// endregion

	// region: ft_strcpy_24
	#[test]
	fn ft_strcpy_24() {
		helper(42, "will partially summon");
	}
	// endregion

	// region: ft_strcpy_25
	#[test]
	fn ft_strcpy_25() {
		helper(42, "arcanes, for a limited");
	}
	// endregion

	// region: ft_strcpy_26
	#[test]
	fn ft_strcpy_26() {
		helper(42, "time. Also, you may see");
	}
	// endregion

	// region: ft_strcpy_27
	#[test]
	fn ft_strcpy_27() {
		helper(42, "some strange apparitions");
	}
	// endregion
}
