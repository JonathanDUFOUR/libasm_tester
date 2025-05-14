use std::ffi::{c_char, c_int};

#[link(name = "asm_bonus")]
unsafe extern "C" {
	unsafe fn ft_atoi_base(s: *const c_char, base: *const c_char) -> c_int;
}

const BUFFER_SIZE: usize = 279;

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

		bytes.iter().enumerate().for_each(|(i, byte)| aligned_bytes.0[i] = *byte);

		aligned_bytes
	}
}

pub fn helper(s: &[u8], base: &[u8], expected: c_int) {
	type Function = unsafe extern "C" fn(*const c_char, *const c_char) -> c_int;

	const FUNCTIONS: &[Function] = &[ft_atoi_base];
	const ALIGN: usize = std::mem::align_of::<AlignedBytes>();

	assert!(BUFFER_SIZE >= ALIGN, "BUFFER_SIZE must be greater than or equal to ALIGN");
	assert!(
		s.len() <= BUFFER_SIZE - ALIGN,
		"s.len() must be lower than or equal to (BUFFER_SIZE - ALIGN)"
	);
	assert!(
		base.len() <= BUFFER_SIZE - ALIGN,
		"base.len() must be lower than or equal to (BUFFER_SIZE - ALIGN)"
	);

	let s_len: usize = s.len() + 1;
	let base_len: usize = base.len() + 1;

	for function in FUNCTIONS {
		use rand::{rng, rngs::ThreadRng, Rng};

		#[inline(always)]
		fn test_once(
			// region: parameters
			function: &Function,
			s: &[u8],
			base: &[u8],
			s_len: usize,
			base_len: usize,
			expected: c_int,
			// endregion
		) {
			// region: body
			let s_ptr: *const c_char = s.as_ptr() as *const c_char;
			let base_ptr: *const c_char = base.as_ptr() as *const c_char;
			let returned: c_int = unsafe { function(s_ptr, base_ptr) };

			if returned != expected {
				const MAX_ELEM_BY_LINE: usize = 16;

				println!("   s_ptr: {:#X}", s_ptr as usize);
				println!("base_ptr: {:#X}", base_ptr as usize);
				println!();
				println!("expected: {}", expected);
				println!("returned: {}", returned);
				println!();
				println!("s:");
				for i in (0..s_len).step_by(MAX_ELEM_BY_LINE) {
					println!("{:02X?}", &s[i..i + MAX_ELEM_BY_LINE.min(s_len - i)]);
				}
				println!();
				println!("base:");
				for i in (0..base_len).step_by(MAX_ELEM_BY_LINE) {
					println!("{:02X?}", &base[i..i + MAX_ELEM_BY_LINE.min(base_len - i)]);
				}
				panic!();
			}
			// endregion
		}

		let mut rng: ThreadRng = rng();
		let mut s: AlignedBytes = s.into();
		let mut base: AlignedBytes = base.into();
		let s: &mut [u8] = &mut s.0;
		let base: &mut [u8] = &mut base.0;

		rng.fill(s[s_len..].as_mut());
		rng.fill(base[base_len..].as_mut());
		test_once(function, s, base, s_len, base_len, expected);
		for s_offset in 1..ALIGN {
			s[s_offset - 1..].copy_within(..s_len, 1);

			let s: &[u8] = &s[s_offset..];

			test_once(function, s, base, s_len, base_len, expected);
		}
		for base_offset in 1..ALIGN {
			s.copy_within(ALIGN - 1..ALIGN - 1 + s_len, 0);
			base[base_offset - 1..].copy_within(..base_len, 1);

			let base: &[u8] = &base[base_offset..];

			test_once(function, s, base, s_len, base_len, expected);
			for s_offset in 1..ALIGN {
				s[s_offset - 1..].copy_within(..s_len, 1);

				let s: &[u8] = &s[s_offset..];

				test_once(function, s, base, s_len, base_len, expected);
			}
		}
	}
}
