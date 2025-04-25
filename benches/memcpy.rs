use {
	criterion::{criterion_group, criterion_main, Criterion},
	std::ffi::c_void,
};

unsafe extern "C" {
	unsafe fn memcpy(dst: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
}

#[link(name = "asm")]
unsafe extern "C" {
	unsafe fn ft_memcpy_dsta_srcu(dst: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
	unsafe fn ft_memcpy_dstu_srca(dst: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
	unsafe fn ft_memcpy_dstu_srcu(dst: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
}

fn criterion_benchmark(c: &mut Criterion) {
	use criterion::{measurement::WallTime, BenchmarkGroup};

	const MAX_INPUT_SIZE: usize = 10_000;
	assert_ne!(MAX_INPUT_SIZE, 0, "MAX_INPUT_SIZE must be greater than 0");

	const DST_OFFSET: usize = 4_077;
	assert!(DST_OFFSET < DST_ALIGN, "DST_OFFSET must be less than DST_ALIGN");

	const SRC_OFFSET: usize = 4_077;
	assert!(SRC_OFFSET < SRC_ALIGN, "SRC_OFFSET must be less than SRC_ALIGN");

	const DST_ALIGN: usize = std::mem::align_of::<AlignedDst>();
	const SRC_ALIGN: usize = std::mem::align_of::<AlignedSrc>();
	const DST_BUFFER_SIZE: usize = MAX_INPUT_SIZE + DST_OFFSET;
	const SRC_BUFFER_SIZE: usize = MAX_INPUT_SIZE + SRC_OFFSET;

	#[repr(align(4096))]
	struct AlignedDst([u8; DST_BUFFER_SIZE]);

	impl AlignedDst {
		fn new() -> Self {
			Self([0; DST_BUFFER_SIZE])
		}
	}

	#[repr(align(4096))]
	struct AlignedSrc([u8; SRC_BUFFER_SIZE]);

	impl AlignedSrc {
		fn new() -> Self {
			Self([0; SRC_BUFFER_SIZE])
		}
	}

	let mut group: BenchmarkGroup<WallTime> = c.benchmark_group("memcpy");

	for input_size in {
		// region: input_sizes
		let mut input_sizes: Vec<usize> = Vec::new();
		let mut input_size: usize = 1;

		while input_size <= MAX_INPUT_SIZE {
			input_sizes.push(input_size);
			input_size *= 2;
		}
		for input_size in (0..MAX_INPUT_SIZE).step_by(101) {
			input_sizes.push(input_size)
		}
		input_sizes.sort();

		input_sizes
		// endregion
	} {
		type Function = unsafe extern "C" fn(*mut c_void, *const c_void, usize) -> *mut c_void;

		const FUNCTIONS: [(Function, &str); 4] = [
			(memcpy, "std"),
			(ft_memcpy_dsta_srcu, "dsta_srcu"),
			(ft_memcpy_dstu_srca, "dstu_srca"),
			(ft_memcpy_dstu_srcu, "dstu_srcu"),
		];

		for (function, function_name) in FUNCTIONS {
			use {
				criterion::BenchmarkId,
				rand::{rngs::ThreadRng, thread_rng, Rng},
			};

			let mut rng: ThreadRng = thread_rng();
			let mut dst: AlignedDst = AlignedDst::new();
			let mut src: AlignedSrc = AlignedSrc::new();
			let dst: &mut [u8] = &mut dst.0[DST_OFFSET..];
			let src: &mut [u8] = &mut src.0[SRC_OFFSET..];

			for i in 0..input_size {
				src[i] = rng.gen_range(0x01..=0xFF);
			}
			group.bench_with_input(BenchmarkId::new(function_name, input_size), &(), |b, _| {
				b.iter(|| unsafe {
					function(
						dst.as_mut_ptr() as *mut c_void,
						src.as_ptr() as *const c_void,
						input_size,
					);
				});
			});
		}
	}
	group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
