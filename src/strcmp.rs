use std::ffi::{c_char, c_int};

#[link(name = "asm")]
extern "C" {
	fn ft_strcmp_s0a_s1u(s0: *const c_char, s1: *const c_char) -> c_int;
	fn ft_strcmp_s0u_s1a(s0: *const c_char, s1: *const c_char) -> c_int;
	fn ft_strcmp_s0u_s1u(s0: *const c_char, s1: *const c_char) -> c_int;
	fn ft_strcmp(s0: *const c_char, s1: *const c_char) -> c_int;
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

pub fn helper(s0: &[u8], s1: &[u8]) {
	type Function = unsafe extern "C" fn(*const c_char, *const c_char) -> c_int;

	const FUNCTIONS: &[Function] = &[
		ft_strcmp_s0a_s1u,
		ft_strcmp_s0u_s1a,
		ft_strcmp_s0u_s1u,
		ft_strcmp,
	];
	const ALIGN: usize = std::mem::align_of::<AlignedBytes>();

	assert!(BUFFER_SIZE >= ALIGN, "BUFFER_SIZE must be greater than or equal to ALIGN");
	assert!(
		s0.len() <= BUFFER_SIZE - ALIGN,
		"s0.len() must be lower than or equal to (BUFFER_SIZE - ALIGN)"
	);
	assert!(
		s1.len() <= BUFFER_SIZE - ALIGN,
		"s1.len() must be lower than or equal to (BUFFER_SIZE - ALIGN)"
	);

	let s0_len: usize = s0.len() + 1;
	let s1_len: usize = s1.len() + 1;

	for function in FUNCTIONS {
		use {
			rand::{rngs::ThreadRng, thread_rng, Rng},
			std::cmp::{
				Ordering,
				Ordering::{Equal, Greater, Less},
			},
		};

		#[inline(always)]
		fn test_with_given_offsets(
			// region: parameters
			function: &Function,
			a: &[u8],
			b: &[u8],
			a_len: usize,
			b_len: usize,
			// endregion
		) {
			// region: body
			let a_ptr: *const c_char = a.as_ptr() as *const c_char;
			let b_ptr: *const c_char = b.as_ptr() as *const c_char;
			let expected: Ordering = a[..a_len].cmp(&b[..b_len]);
			let returned: c_int = unsafe { function(a_ptr, b_ptr) };

			if match expected {
				Equal => returned != 0,
				Less => returned >= 0,
				Greater => returned <= 0,
			} {
				const MAX_ELEM_BY_LINE: usize = 16;

				println!("a_ptr: {:#X}", a_ptr as usize);
				println!("b_ptr: {:#X}", b_ptr as usize);
				println!();
				println!(
					"expected: {}",
					match expected {
						Equal => "=0",
						Less => "<0",
						Greater => ">0",
					}
				);
				println!(
					"returned: {} ({})",
					match returned.cmp(&0) {
						Equal => "=0",
						Less => "<0",
						Greater => ">0",
					},
					returned
				);
				println!();
				println!("a:");
				for i in (0..a_len).step_by(MAX_ELEM_BY_LINE) {
					println!("{:02X?}", &a[i..i + MAX_ELEM_BY_LINE.min(a_len - i)]);
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
		let mut s0: AlignedBytes = s0.into();
		let mut s1: AlignedBytes = s1.into();
		let s0: &mut [u8] = &mut s0.0;
		let s1: &mut [u8] = &mut s1.0;

		rng.fill(s0[s0_len..].as_mut());
		rng.fill(s1[s1_len..].as_mut());
		test_with_given_offsets(function, s0, s1, s0_len, s1_len);
		test_with_given_offsets(function, s1, s0, s1_len, s0_len);
		for s0_offset in 1..ALIGN {
			s0[s0_offset - 1..].copy_within(..s0_len, 1);

			let s0: &[u8] = &s0[s0_offset..];

			test_with_given_offsets(function, s0, s1, s0_len, s1_len);
			test_with_given_offsets(function, s1, s0, s1_len, s0_len);
		}
		for s1_offset in 1..ALIGN {
			s0.copy_within(ALIGN - 1..ALIGN - 1 + s0_len, 0);
			s1[s1_offset - 1..].copy_within(..s1_len, 1);

			let s1: &[u8] = &s1[s1_offset..];

			test_with_given_offsets(function, s0, s1, s0_len, s1_len);
			test_with_given_offsets(function, s1, s0, s1_len, s0_len);
			for s0_offset in 1..ALIGN {
				s0[s0_offset - 1..].copy_within(..s0_len, 1);

				let s0: &[u8] = &s0[s0_offset..];

				test_with_given_offsets(function, s0, s1, s0_len, s1_len);
				test_with_given_offsets(function, s1, s0, s1_len, s0_len);
			}
		}
	}
}
