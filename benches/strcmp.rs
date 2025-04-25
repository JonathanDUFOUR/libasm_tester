use {
	criterion::{criterion_group, criterion_main, Criterion},
	std::ffi::{c_char, c_int},
};

unsafe extern "C" {
	unsafe fn strcmp(s0: *const c_char, s1: *const c_char) -> c_int;
}

#[link(name = "asm")]
unsafe extern "C" {
	unsafe fn ft_strcmp_s0a_s1u(s0: *const c_char, s1: *const c_char) -> c_int;
	unsafe fn ft_strcmp_s0u_s1a(s0: *const c_char, s1: *const c_char) -> c_int;
	unsafe fn ft_strcmp_s0u_s1u(s0: *const c_char, s1: *const c_char) -> c_int;
}

fn criterion_benchmark(c: &mut Criterion) {
	use criterion::{measurement::WallTime, BenchmarkGroup};

	const MAX_INPUT_SIZE: usize = 10_000;
	assert_ne!(MAX_INPUT_SIZE, 0, "MAX_INPUT_SIZE must be greater than 0");

	const S0_OFFSET: usize = 4_077;
	assert!(S0_OFFSET < S0_ALIGN, "S0_OFFSET must be less than S0_ALIGN");

	const S1_OFFSET: usize = 4_077;
	assert!(S1_OFFSET < S1_ALIGN, "S1_OFFSET must be less than S1_ALIGN");

	const S0_ALIGN: usize = std::mem::align_of::<AlignedS0>();
	const S1_ALIGN: usize = std::mem::align_of::<AlignedS1>();
	const S0_BUFFER_SIZE: usize = MAX_INPUT_SIZE + S0_OFFSET + 1;
	const S1_BUFFER_SIZE: usize = MAX_INPUT_SIZE + S1_OFFSET + 1;

	#[repr(align(4096))]
	struct AlignedS0([c_char; S0_BUFFER_SIZE]);

	impl AlignedS0 {
		fn new() -> Self {
			Self([0; S0_BUFFER_SIZE])
		}
	}

	#[repr(align(4096))]
	struct AlignedS1([c_char; S1_BUFFER_SIZE]);

	impl AlignedS1 {
		fn new() -> Self {
			Self([0; S1_BUFFER_SIZE])
		}
	}

	let mut group: BenchmarkGroup<WallTime> = c.benchmark_group("strcmp");

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
		type Function = unsafe extern "C" fn(*const c_char, *const c_char) -> c_int;

		const FUNCTIONS: [(Function, &str); 4] = [
			(strcmp, "std"),
			(ft_strcmp_s0a_s1u, "s0a_s1u"),
			(ft_strcmp_s0u_s1a, "s0u_s1a"),
			(ft_strcmp_s0u_s1u, "s0u_s1u"),
		];

		for (function, function_name) in FUNCTIONS {
			use {
				criterion::BenchmarkId,
				rand::{rngs::ThreadRng, thread_rng, Rng},
			};

			let mut rng: ThreadRng = thread_rng();
			let mut s0: AlignedS0 = AlignedS0::new();
			let mut s1: AlignedS1 = AlignedS1::new();
			let s0: &mut [c_char] = &mut s0.0[S0_OFFSET..];
			let s1: &mut [c_char] = &mut s1.0[S1_OFFSET..];

			for i in 0..input_size {
				let c: c_char = rng.gen_range(0x01..=0xFF) as c_char;

				s0[i] = c;
				s1[i] = c;
			}
			group.bench_with_input(BenchmarkId::new(function_name, input_size), &(), |b, _| {
				b.iter(|| unsafe { function(s0.as_ptr(), s1.as_ptr()) })
			});
		}
	}
	group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
