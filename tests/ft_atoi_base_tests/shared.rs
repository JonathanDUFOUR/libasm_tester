use std::ffi::{c_char, c_int};

#[link(name = "asm_bonus")]
extern "C" {
	fn ft_atoi_base(s: *const c_char, b: *const c_char) -> c_int;
}

const BUFFER_SIZE: usize = 1_057;

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

pub fn helper(s: &[u8], b: &[u8], expected: c_int) {
	type Function = unsafe extern "C" fn(*const c_char, *const c_char) -> c_int;

	const FUNCTIONS: [Function; 1] = [ft_atoi_base];
	const ALIGN: usize = std::mem::align_of::<AlignedBytes>();

	assert!(BUFFER_SIZE >= ALIGN, "BUFFER_SIZE must be greater than or equal to ALIGN");
	assert!(
		s.len() <= BUFFER_SIZE - ALIGN,
		"s.len() must be lower than or equal to (BUFFER_SIZE - ALIGN)"
	);
	assert!(
		b.len() <= BUFFER_SIZE - ALIGN,
		"b.len() must be lower than or equal to (BUFFER_SIZE - ALIGN)"
	);

	let s_len: usize = s.len() + 1;
	let b_len: usize = b.len() + 1;

	for function in FUNCTIONS {
		use rand::{rngs::ThreadRng, thread_rng, Rng};

		#[inline(always)]
		fn test_with_given_offsets(
			// region: parameters
			function: Function,
			s: &[u8],
			b: &[u8],
			s_len: usize,
			b_len: usize,
			expected: c_int,
			// endregion
		) {
			// region: body
			let s_ptr: *const c_char = s.as_ptr() as *const c_char;
			let b_ptr: *const c_char = b.as_ptr() as *const c_char;
			let returned: c_int = unsafe { function(s_ptr, b_ptr) };

			if returned != expected {
				const MAX_ELEM_BY_LINE: usize = 16;

				println!("s_ptr: {:#X}", s_ptr as usize);
				println!("b_ptr: {:#X}", b_ptr as usize);
				println!();
				println!("expected: {}", expected);
				println!("returned: {}", returned);
				println!();
				println!("s:");
				for i in (0..s_len).step_by(MAX_ELEM_BY_LINE) {
					println!("{:02X?}", &s[i..i + MAX_ELEM_BY_LINE.min(s_len - i)]);
				}
				println!();
				println!("b:");
				for i in (0..b_len).step_by(MAX_ELEM_BY_LINE) {
					println!("{:02X?}", &b[i..i + MAX_ELEM_BY_LINE.min(b_len - i)]);
				}
				panic!();
			}
			// endregion
		}

		let mut rng: ThreadRng = thread_rng();
		let mut s: AlignedBytes = s.into();
		let mut b: AlignedBytes = b.into();
		let s: &mut [u8] = &mut s.0;
		let b: &mut [u8] = &mut b.0;

		rng.fill(s[s_len..].as_mut());
		rng.fill(b[b_len..].as_mut());
		test_with_given_offsets(function, s, b, s_len, b_len, expected);
		for s_offset in 1..ALIGN {
			s[s_offset - 1..].copy_within(..s_len, 1);

			let s: &[u8] = &s[s_offset..];

			test_with_given_offsets(function, s, b, s_len, b_len, expected);
		}
		for b_offset in 1..ALIGN {
			s.copy_within(ALIGN - 1..ALIGN - 1 + s_len, 0);
			b[b_offset - 1..].copy_within(..b_len, 1);

			let b: &[u8] = &b[b_offset..];

			test_with_given_offsets(function, s, b, s_len, b_len, expected);
			for s_offset in 1..ALIGN {
				s[s_offset - 1..].copy_within(..s_len, 1);

				let s: &[u8] = &s[s_offset..];

				test_with_given_offsets(function, s, b, s_len, b_len, expected);
			}
		}
	}
}
