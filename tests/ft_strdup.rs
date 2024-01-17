#[cfg(test)]
mod tests {
	use std::{
		ffi::{c_char, c_void},
		ptr::null_mut,
	};

	#[link(name = "asm")]
	extern "C" {
		fn ft_strdup(s: *const c_char) -> *mut c_char;
		fn free(ptr: *mut c_void);
	}

	#[inline(always)]
	fn unit_test_helper(src: &[u8]) {
		let nul: usize = match src.iter().position(|&c| c == 0x00) {
			Some(i) => i,
			None => panic!("missing nul terminator in src"),
		};
		let dst: *mut c_char = unsafe { ft_strdup(src.as_ptr() as *const c_char) };

		assert_ne!(dst, null_mut());
		assert_eq!(unsafe { std::slice::from_raw_parts(dst as *const u8, nul + 1) }, &src[..=nul]);
		unsafe { free(dst as *mut c_void) };
	}

	// region: ft_strdup_00
	#[test]
	fn ft_strdup_00() {
		unit_test_helper(&[0x00]);
	}
	// endregion

	// region: ft_strdup_01
	#[test]
	fn ft_strdup_01() {
		unit_test_helper(&[0x2a, 0x00]);
	}
	// endregion

	// region: ft_strdup_02
	#[test]
	fn ft_strdup_02() {
		unit_test_helper(&[0xff, 0x21, 0x45, 0xa3, 0x39, 0x00]);
	}
	// endregion

	// region: ft_strdup_03
	#[test]
	fn ft_strdup_03() {
		unit_test_helper(&[
			0xb0, 0xfd, 0xdd, 0x64, 0x58, 0x99, 0x13, 0xa6, 0x00, 0x49, 0xba, 0x2d, 0x00,
		]);
	}
	// endregion
}
