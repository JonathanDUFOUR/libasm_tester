use {
	criterion::{criterion_group, criterion_main, Criterion},
	std::ffi::c_void,
};

unsafe extern "C" {
	unsafe fn memcpy(dst: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
}

#[link(name = "asm")]
unsafe extern "C" {
	unsafe fn ft_memcpy(dst: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
}

fn criterion_benchmark(c: &mut Criterion) {
	use criterion::{measurement::WallTime, BenchmarkGroup};

	const MAX_INPUT_SIZE: usize = 10_000;
	assert_ne!(MAX_INPUT_SIZE, 0, "MAX_INPUT_SIZE must be greater than 0");

	const DST_OFFSET: usize = 4_077;
	assert!(DST_OFFSET < ALIGNMENT, "DST_OFFSET must be less than ALIGNMENT");

	const SRC_OFFSET: usize = 4_077;
	assert!(SRC_OFFSET < ALIGNMENT, "SRC_OFFSET must be less than ALIGNMENT");

	const ALIGNMENT: usize = std::mem::align_of::<AlignedBytes>();
	const BUFFER_SIZE: usize = MAX_INPUT_SIZE + libasm_tester::max(DST_OFFSET, SRC_OFFSET);

	#[repr(align(4096))]
	struct AlignedBytes([u8; BUFFER_SIZE]);

	impl AlignedBytes {
		fn new() -> Self {
			Self([0; BUFFER_SIZE])
		}
	}

	let group_name: String = "memcpy".to_owned()
		+ "_" + &ALIGNMENT.to_string()
		+ "_" + &DST_OFFSET.to_string()
		+ "_" + &SRC_OFFSET.to_string();
	let mut group: BenchmarkGroup<WallTime> = c.benchmark_group(group_name);

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

		const FUNCTIONS: &[(Function, &str)] = &[
			(ft_memcpy, "ft"),
			(memcpy, "std"),
		];

		for (function, name) in FUNCTIONS {
			use {
				criterion::BenchmarkId,
				rand::{rng, rngs::ThreadRng, Rng},
			};

			let mut rng: ThreadRng = rng();
			let dst: &mut [u8] = &mut AlignedBytes::new().0[DST_OFFSET..];
			let src: &mut [u8] = &mut AlignedBytes::new().0[SRC_OFFSET..];

			for i in 0..input_size {
				src[i] = rng.random();
			}
			group.bench_with_input(BenchmarkId::new(*name, input_size), &(), |b, _| {
				b.iter(|| unsafe {
					function(dst.as_mut_ptr().cast(), src.as_ptr().cast(), input_size);
				});
			});
		}
	}
	group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
