use std::ffi::c_void;

#[link(name = "asm")]
extern "C" {
	fn ft_memcpy_dsta_srcu(dst: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
	fn ft_memcpy_dstu_srca(dst: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
	fn ft_memcpy_dstu_srcu(dst: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
	fn ft_memcpy(dst: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
}

const BUFFER_SIZE: usize = 1_055;

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

pub fn helper(src: &[u8]) {
	type Function = unsafe extern "C" fn(*mut c_void, *const c_void, usize) -> *mut c_void;

	const FUNCTIONS: &[Function] = &[
		ft_memcpy_dsta_srcu,
		ft_memcpy_dstu_srca,
		ft_memcpy_dstu_srcu,
		ft_memcpy,
	];
	const ALIGN: usize = std::mem::align_of::<AlignedBytes>();

	assert!(BUFFER_SIZE >= ALIGN, "BUFFER_SIZE must be greater than or equal to ALIGN");
	assert!(
		src.len() <= BUFFER_SIZE - ALIGN + 1,
		"src.len() must be less than or equal to (BUFFER_SIZE - ALIGN + 1)"
	);

	let n: usize = src.len();

	for function in FUNCTIONS {
		use rand::{rngs::ThreadRng, thread_rng, Rng};

		#[inline(always)]
		fn test_with_given_offset(
			// region: parameters
			function: &Function,
			dst: &mut [u8],
			src: &[u8],
			n: usize,
			// endregion
		) {
			// region: body
			let dst_p: *mut c_void = dst.as_mut_ptr() as *mut c_void;
			let src_p: *const c_void = src.as_ptr() as *const c_void;
			let dst_overflow: &[u8] = &dst[n..];
			let src_overflow: &[u8] = &src[n..];
			let dst_overflow_old: Vec<u8> = dst_overflow.to_vec();
			let returned: *mut c_void = unsafe { function(dst_p, src_p, n) };
			let returned_wrong_val: bool = returned != dst_p;
			let dst_and_src_differ: bool = dst[..n] != src[..n];
			let wrote_out_of_range: bool = dst_overflow != dst_overflow_old;

			if returned_wrong_val || dst_and_src_differ || wrote_out_of_range {
				const MAX_ELEM_BY_LINE: usize = 16;

				println!("dst_p: {:#X}", dst_p as usize);
				println!("src_p: {:#X}", src_p as usize);
				println!("    n: {}", n);
				if returned_wrong_val {
					println!();
					println!("expected: {:#X}", dst_p as usize);
					println!("returned: {:#X}", returned as usize);
				}
				if dst_and_src_differ {
					println!();
					println!("dst:");
					for i in (0..n).step_by(MAX_ELEM_BY_LINE) {
						println!("{:02X?}", &dst[i..i + MAX_ELEM_BY_LINE.min(n - i)]);
					}
					println!();
					println!("src:");
					for i in (0..n).step_by(MAX_ELEM_BY_LINE) {
						println!("{:02X?}", &src[i..i + MAX_ELEM_BY_LINE.min(n - i)]);
					}
				}
				if wrote_out_of_range {
					let dst_overflow_old_len: usize = dst_overflow_old.len();
					let dst_overflow_len: usize = dst_overflow.len();
					let src_overflow_len: usize = src_overflow.len();

					println!();
					println!("dst_overflow_old:");
					for i in (0..dst_overflow_old_len).step_by(MAX_ELEM_BY_LINE) {
						println!(
							"{:02X?}",
							&dst_overflow_old
								[i..i + MAX_ELEM_BY_LINE.min(dst_overflow_old_len - i)]
						);
					}
					println!();
					println!("dst_overflow:");
					for i in (0..dst_overflow_len).step_by(MAX_ELEM_BY_LINE) {
						println!(
							"{:02X?}",
							&dst_overflow[i..i + MAX_ELEM_BY_LINE.min(dst_overflow_len - i)]
						);
					}
					println!();
					println!("src_overflow:");
					for i in (0..src_overflow_len).step_by(MAX_ELEM_BY_LINE) {
						println!(
							"{:02X?}",
							&src_overflow[i..i + MAX_ELEM_BY_LINE.min(src_overflow_len - i)]
						);
					}
				}
				panic!();
			}
			// endregion
		}

		let mut rng: ThreadRng = thread_rng();
		let mut dst: AlignedBytes = AlignedBytes::new();
		let mut src: AlignedBytes = src.into();
		let dst: &mut [u8] = &mut dst.0;
		let src: &mut [u8] = &mut src.0;

		rng.fill(src[n..].as_mut());
		rng.fill(dst[n..].as_mut());
		for dst_offset in 0..ALIGN {
			let dst: &mut [u8] = &mut dst[dst_offset..];

			test_with_given_offset(function, dst, src, n);
		}
		for src_offset in 1..ALIGN {
			src[src_offset - 1..].copy_within(..n, 1);
			rng.fill(dst[n..].as_mut());

			let src: &[u8] = &src[src_offset..];

			for dst_offset in 0..ALIGN {
				let dst: &mut [u8] = &mut dst[dst_offset..];
				test_with_given_offset(function, dst, src, n);
			}
		}
	}
}
