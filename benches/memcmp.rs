use {
	criterion::{criterion_group, criterion_main, Criterion},
	std::ffi::{c_int, c_void},
};

unsafe extern "C" {
	unsafe fn memcmp(s0: *const c_void, s1: *const c_void, n: usize) -> c_int;
}

#[link(name = "asm")]
unsafe extern "C" {
	unsafe fn ft_memcmp(s0: *const c_void, s1: *const c_void, n: usize) -> c_int;
}

fn criterion_benchmark(c: &mut Criterion) {
	use criterion::{measurement::WallTime, BenchmarkGroup};

	const MAX_INPUT_SIZE: usize = 10_000;
	assert_ne!(MAX_INPUT_SIZE, 0, "MAX_INPUT_SIZE must be greater than 0");

	const S0_OFFSET: usize = 4_077;
	assert!(S0_OFFSET < ALIGNMENT, "S0_OFFSET must be less than ALIGNMENT");

	const S1_OFFSET: usize = 4_077;
	assert!(S1_OFFSET < ALIGNMENT, "S1_OFFSET must be less than ALIGNMENT");

	const ALIGNMENT: usize = std::mem::align_of::<AlignedBytes>();
	const BUFFER_SIZE: usize = MAX_INPUT_SIZE + libasm_tester::max(S0_OFFSET, S1_OFFSET);

	#[repr(align(4_096))]
	struct AlignedBytes([u8; BUFFER_SIZE]);

	impl AlignedBytes {
		fn new() -> Self {
			Self([0; BUFFER_SIZE])
		}
	}

	let group_name: String = "memcmp".to_owned()
		+ "_" + &ALIGNMENT.to_string()
		+ "_" + &S0_OFFSET.to_string()
		+ "_" + &S1_OFFSET.to_string();
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
		type Function = unsafe extern "C" fn(*const c_void, *const c_void, usize) -> c_int;

		const FUNCTIONS: &[(Function, &str)] = &[
			(ft_memcmp, "ft"),
			(memcmp, "std"),
		];

		for (function, name) in FUNCTIONS {
			use {
				criterion::BenchmarkId,
				rand::{rng, rngs::ThreadRng, Rng},
			};

			let mut rng: ThreadRng = rng();
			let s0: &mut [u8] = &mut AlignedBytes::new().0[S0_OFFSET..];
			let s1: &mut [u8] = &mut AlignedBytes::new().0[S1_OFFSET..];

			for i in 0..input_size {
				let byte: u8 = rng.random();

				s0[i] = byte;
				s1[i] = byte;
			}
			group.bench_with_input(BenchmarkId::new(*name, input_size), &(), |b, _| {
				b.iter(|| unsafe { function(s0.as_ptr().cast(), s1.as_ptr().cast(), input_size) })
			});
		}
	}
	group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
