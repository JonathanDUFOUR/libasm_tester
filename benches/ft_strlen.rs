use {
	criterion::{criterion_group, criterion_main, Criterion},
	std::ffi::c_char,
};

extern "C" {
	fn strlen(s: *const c_char) -> usize;
}

#[link(name = "asm")]
extern "C" {
	fn ft_strlen_sa(s: *const c_char) -> usize;
	fn ft_strlen_su(s: *const c_char) -> usize;
}

fn criterion_benchmark(c: &mut Criterion) {
	use criterion::{measurement::WallTime, BenchmarkGroup};

	const MAX_INPUT_SIZE: usize = 10_000;
	assert_ne!(MAX_INPUT_SIZE, 0, "MAX_INPUT_SIZE must be greater than 0");

	const OFFSET: usize = 4077;
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

	let mut group: BenchmarkGroup<WallTime> = c.benchmark_group("strlen");

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
		type Function = unsafe extern "C" fn(*const c_char) -> usize;

		const FUNCTIONS: [(Function, &str); 3] = [
			(strlen, "std"),
			(ft_strlen_sa, "sa"),
			(ft_strlen_su, "su"),
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
				b.iter(|| unsafe { function(s.as_ptr()) })
			});
		}
	}
	group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
