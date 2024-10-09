use std::ffi::c_char;

#[link(name = "asm")]
extern "C" {
	fn ft_strlen_sa(s: *const c_char) -> usize;
	fn ft_strlen_su(s: *const c_char) -> usize;
	fn ft_strlen(s: *const c_char) -> usize;
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

pub fn helper(s: &[u8]) {
	type Function = unsafe extern "C" fn(*const c_char) -> usize;

	const FUNCTIONS: &[Function] = &[
		ft_strlen_sa,
		ft_strlen_su,
		ft_strlen,
	];
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
			len: usize,
			// endregion
		) {
			// region: body
			let p: *const c_char = s.as_ptr() as *const c_char;
			let returned: usize = unsafe { function(p) };
			let returned_wrong_val: bool = returned != len;

			if returned_wrong_val {
				const MAX_ELEM_BY_LINE: usize = 16;

				println!("       p: {:#X}", p as usize);
				println!();
				println!("expected: {}", len);
				println!("returned: {}", returned);
				println!();
				println!("s:");

				let len: usize = len + 1;

				for i in (0..len).step_by(MAX_ELEM_BY_LINE) {
					println!("{:02X?}", &s[i..i + MAX_ELEM_BY_LINE.min(len - i)]);
				}
				panic!();
			}
			// endregion
		}

		let mut s: AlignedBytes = s.into();
		let s: &mut [u8] = &mut s.0;

		test_with_given_offset(function, s, len);
		for offset in 1..ALIGN {
			s[offset - 1..].copy_within(..len, 1);
			test_with_given_offset(function, s[offset..].as_ref(), len);
		}
	}
}
