use std::ffi::{c_char, c_void};

#[link(name = "asm")]
extern "C" {
	fn ft_strdup(s: *const c_char) -> *mut c_char;
}

extern "C" {
	fn free(ptr: *mut c_void);
}

const BUFFER_SIZE: usize = 1_056;

#[repr(align(32))]
struct AlignedBytes([u8; BUFFER_SIZE]);

impl AlignedBytes {
	fn new() -> Self {
		Self([0; BUFFER_SIZE])
	}
}

impl std::convert::From<&[u8]> for AlignedBytes {
	fn from(bytes: &[u8]) -> Self {
		let mut aligned_bytes: Self = Self::new();

		for (i, byte) in bytes.iter().enumerate() {
			aligned_bytes.0[i] = *byte;
		}

		aligned_bytes
	}
}

#[inline(always)]
pub fn helper(s: &[u8]) {
	type Function = unsafe extern "C" fn(*const c_char) -> *mut c_char;

	const FUNCTIONS: &[Function] = &[ft_strdup];
	const ALIGN: usize = std::mem::align_of::<AlignedBytes>();

	assert!(BUFFER_SIZE >= ALIGN, "BUFFER_SIZE must be greater than or equal to ALIGN");
	assert!(
		s.len() <= BUFFER_SIZE - ALIGN,
		"s.len() must be less than or equal to (BUFFER_SIZE - ALIGN)"
	);

	let len: usize = s.len();

	for function in FUNCTIONS {
		fn test_with_given_offset(
			// region: parameters
			function: &Function,
			s: &[u8],
			// endregion
		) {
			// region: body
			let p: *const c_char = s.as_ptr() as *const c_char;
			let returned: *mut u8 = unsafe { function(p) } as *mut u8;
			let returned_wrong_val: bool = returned.is_null();

			if returned_wrong_val {
				const MAX_ELEM_BY_LINE: usize = 16;

				println!("       p: {:#X}", p as usize);
				println!();
				println!("returned: {:#X}", returned as usize);
				println!();
				println!("s:");

				let len: usize = s.len();

				for i in (0..len).step_by(MAX_ELEM_BY_LINE) {
					println!("{:02X?}", &s[i..i + MAX_ELEM_BY_LINE.min(len - i)]);
				}

				panic!();
			}

			let new_s: &[u8] = unsafe { std::slice::from_raw_parts(returned, s.len()) };

			if new_s != s {
				const MAX_ELEM_BY_LINE: usize = 16;

				println!("       p: {:#X}", p as usize);
				println!();
				println!("s:");

				let len: usize = s.len();

				for i in (0..len).step_by(MAX_ELEM_BY_LINE) {
					println!("{:02X?}", &s[i..i + MAX_ELEM_BY_LINE.min(len - i)]);
				}
				println!();
				println!("new_s:");
				for i in (0..len).step_by(MAX_ELEM_BY_LINE) {
					println!("{:02X?}", &new_s[i..i + MAX_ELEM_BY_LINE.min(len - i)]);
				}

				unsafe { free(returned as *mut c_void) };
				panic!();
			}

			unsafe { free(returned as *mut c_void) };
			// endregion
		}

		let mut s: AlignedBytes = s.into();
		let s: &mut [u8] = &mut s.0;

		test_with_given_offset(function, s[..=len].as_ref());
		for offset in 1..ALIGN {
			s[offset - 1..].copy_within(..len, 1);
			test_with_given_offset(function, s[offset..=offset + len].as_ref());
		}
	}
}
