use std::ffi::c_char;

extern "C" {
	fn ft_strcpy(dst: *mut c_char, src: *const c_char) -> *const c_char;
}

// region: ft_strcpy_00
#[test]
fn ft_strcpy_00() {
	let src: [c_char; 1] = [0];
	let mut dst: [c_char; 1] = [0];

	assert_eq!(unsafe { ft_strcpy(dst.as_mut_ptr(), src.as_ptr()) }, dst.as_ptr());
	assert_eq!(dst, src);
}
// endregion: ft_strcpy_00

// region: ft_strcpy_01
#[test]
fn ft_strcpy_01() {
	let src: [c_char; 6] = [1, 33, 69, -93, 57, 0];
	let mut dst: [c_char; 11] = [-1; 11];

	assert_eq!(unsafe { ft_strcpy(dst.as_mut_ptr(), src.as_ptr()) }, dst.as_ptr());
	assert_eq!(dst[..6], src);
	assert_eq!(dst[6..], [-1; 5]);
}
// endregion: ft_strcpy_01

// region: ft_strcpy_02
#[test]
fn ft_strcpy_02() {
	let src: [c_char; 15] = [
		-46, 64, 9, 57, 65, 29, -100, 0, 46, 63, -15, 51, 94, -128, 0,
	];
	let mut dst: [c_char; 15] = [-109; 15];

	assert_eq!(unsafe { ft_strcpy(dst.as_mut_ptr(), src.as_ptr()) }, dst.as_ptr());
	assert_eq!(dst[..8], src[..8]);
	assert_eq!(dst[8..], [-109; 7]);
}
// endregion: ft_strcpy_02
