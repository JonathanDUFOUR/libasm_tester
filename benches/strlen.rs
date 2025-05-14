use {
	criterion::{criterion_group, criterion_main, Criterion},
	std::ffi::c_char,
};

unsafe extern "C" {
	unsafe fn strlen(s: *const c_char) -> usize;
}

#[link(name = "asm")]
unsafe extern "C" {
	unsafe fn ft_strlen(s: *const c_char) -> usize;
}

fn criterion_benchmark(c: &mut Criterion) {
	use criterion::{measurement::WallTime, BenchmarkGroup};

	const MAX_INPUT_SIZE: usize = 10_000;
	assert_ne!(MAX_INPUT_SIZE, 0, "MAX_INPUT_SIZE must be greater than 0");

	const OFFSET: usize = 4_077;
	assert!(OFFSET < ALIGN, "OFFSET must be less than ALIGN");

	const ALIGN: usize = std::mem::align_of::<AlignedCChars>();
	const BUFFER_SIZE: usize = MAX_INPUT_SIZE + OFFSET + 1;

	#[repr(align(4_096))]
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

		const FUNCTIONS: &[(Function, &str)] = &[
			(strlen, "0_std"),
			(ft_strlen, "1_ft"),
		];

		for (function, name) in FUNCTIONS {
			use {
				criterion::BenchmarkId,
				rand::{rng, rngs::ThreadRng, Rng},
			};

			let mut rng: ThreadRng = rng();
			let mut s: AlignedCChars = AlignedCChars::new();
			let s: &mut [c_char] = &mut s.0[OFFSET..];

			for i in 0..input_size {
				s[i] = rng.random_range(0x_01..=0x_FF) as c_char;
			}
			group.bench_with_input(BenchmarkId::new(*name, input_size), &(), |b, _| {
				b.iter(|| unsafe { function(s.as_ptr()) })
			});
		}
	}
	group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
