#[cfg(test)]
mod tests {
	use std::ffi::c_char;

	#[link(name = "asm")]
	extern "C" {
		fn ft_strcpy(dst: *mut c_char, src: *const c_char) -> *const c_char;
	}

	#[inline(always)]
	fn unit_test_helper(dst: &mut [u8], src: &[u8]) {
		let n: usize = match src.iter().position(|c: &u8| *c == 0x00) {
			Some(i) if i < dst.len() => i + 1,
			Some(_) => panic!("copy would overflow dst"),
			None => panic!("missing nul terminator in src"),
		};
		let initial_last_bytes_of_dst: Vec<u8> = dst[n..].to_vec();

		assert_eq!(
			unsafe { ft_strcpy(dst.as_mut_ptr() as *mut c_char, src.as_ptr() as *const c_char) },
			dst.as_ptr() as *const c_char
		);
		assert_eq!(dst[..n], src[..n]);
		assert_eq!(dst[n..], initial_last_bytes_of_dst);
	}

	// region: ft_strcpy_00
	#[test]
	fn ft_strcpy_00() {
		unit_test_helper(&mut [0x00], &[0x00]);
	}
	// endregion

	// region: ft_strcpy_01
	#[test]
	fn ft_strcpy_01() {
		unit_test_helper(&mut [0xff; 11], &[0x01, 0x21, 0x45, 0xa3, 0x39, 0x00])
	}
	// endregion

	// region: ft_strcpy_02
	#[test]
	fn ft_strcpy_02() {
		unit_test_helper(
			&mut [0x93; 15],
			&[
				0xd2, 0x40, 0x09, 0x39, 0x41, 0x1d, 0x9c, 0x00, 0x2e, 0x3f, 0xf1, 0x33, 0x5e, 0x80,
				0x00,
			],
		);
	}
	// endregion
}
