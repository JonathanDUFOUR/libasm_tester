use {
	criterion::{criterion_group, criterion_main, Criterion},
	std::ffi::{c_char, c_void},
};

unsafe extern "C" {
	unsafe fn strdup(s: *const c_char) -> *mut c_char;
	unsafe fn free(ptr: *mut c_void);
}

#[link(name = "asm")]
unsafe extern "C" {
	unsafe fn ft_strdup(s: *const c_char) -> *mut c_char;
}

fn criterion_benchmark(c: &mut Criterion) {
	use criterion::{measurement::WallTime, BenchmarkGroup};

	const MAX_INPUT_SIZE: usize = 10_000;
	assert_ne!(MAX_INPUT_SIZE, 0, "MAX_INPUT_SIZE must be greater than 0");

	const OFFSET: usize = 4_077;
	assert!(OFFSET < ALIGN, "OFFSET must be less than ALIGN");

	const ALIGN: usize = std::mem::align_of::<AlignedCChars>();
	const BUFFER_SIZE: usize = MAX_INPUT_SIZE + OFFSET + 1;

	#[repr(align(4096))]
	struct AlignedCChars([c_char; BUFFER_SIZE]);

	impl AlignedCChars {
		fn new() -> Self {
			Self([0; BUFFER_SIZE])
		}
	}

	let mut group: BenchmarkGroup<WallTime> = c.benchmark_group("strdup");

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
		type Function = unsafe extern "C" fn(*const c_char) -> *mut c_char;

		const FUNCTIONS: [(Function, &str); 2] = [
			(strdup, "std"),
			(ft_strdup, "ft"),
		];

		for (function, function_name) in FUNCTIONS {
			use {
				criterion::BenchmarkId,
				rand::{rngs::ThreadRng, thread_rng, Rng},
			};

			let mut rng: ThreadRng = thread_rng();
			let mut s: AlignedCChars = AlignedCChars::new();
			let s: &mut [c_char] = &mut s.0[OFFSET..];

			for i in 0..input_size {
				s[i] = rng.gen_range(0x01..=0xFF) as c_char;
			}
			group.bench_with_input(BenchmarkId::new(function_name, input_size), &(), |b, _| {
				b.iter(|| unsafe { free(function(s.as_ptr()) as *mut c_void) })
			});
		}
	}
	group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
