#[cfg(test)]
mod tests {
	use std::{
		ffi::{c_char, c_void},
		ptr::null_mut,
	};

	extern "C" {
		fn ft_strdup(s: *const c_char) -> *mut c_char;
		fn free(ptr: *mut c_void);
	}

	#[inline(always)]
	fn unit_test_helper(src: &[c_char]) {
		let end: usize = match src.iter().position(|&c| c == 0) {
			Some(i) => i,
			None => panic!("unit_test_helper: src does not contain any null byte"),
		};
		let dst: *mut c_char = unsafe { ft_strdup(src.as_ptr()) };

		assert_ne!(dst, null_mut());
		for i in 0..end {
			assert_eq!(unsafe { *dst.add(i) }, src[i]);
		}
		assert_eq!(unsafe { *dst.add(end) }, 0);
		unsafe { free(dst as *mut c_void) };
	}

	// region: ft_strdup_00
	#[test]
	fn ft_strdup_00() {
		unit_test_helper(&[0]);
	}
	// endregion

	// region: ft_strdup_01
	#[test]
	fn ft_strdup_01() {
		unit_test_helper(&[42, 0]);
	}
	// endregion

	// region: ft_strdup_02
	#[test]
	fn ft_strdup_02() {
		unit_test_helper(&[-1, 33, 69, -93, 57, 0]);
	}
	// endregion

	// region: ft_strdup_03
	#[test]
	fn ft_strdup_03() {
		unit_test_helper(&[-80, -3, -35, 100, 88, -103, 19, -90, 0, 73, -70, 45, 0]);
	}
	// endregion
}
