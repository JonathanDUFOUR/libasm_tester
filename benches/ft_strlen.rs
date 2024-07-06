use {
	criterion::{
		criterion_group, criterion_main, measurement::WallTime, BenchmarkGroup, BenchmarkId,
		Criterion,
	},
	std::ffi::c_char,
};

#[link(name = "asm")]
extern "C" {
	fn ft_strlen(s: *const c_char) -> usize;
}

extern "C" {
	fn strlen(s: *const c_char) -> usize;
}

fn criterion_benchmark(c: &mut Criterion) {
	use rand::{rngs::ThreadRng, thread_rng, Rng};

	type Fn = unsafe extern "C" fn(*const c_char) -> usize;

	const FN_NB: usize = 2;
	const NAMES: [&str; FN_NB] = ["ft", "std"];
	const CALLS: [Fn; FN_NB] = [ft_strlen, strlen];
	const MAX_LENGTH: usize = 1_024;
	const OFFSET: usize = 0;
	const BUFFER_SIZE: usize = MAX_LENGTH + OFFSET;
	const ALIGN: usize = std::mem::align_of::<AlignedCChars>();

	assert!(OFFSET < ALIGN, "OFFSET must be less than ALIGN");
	assert_ne!(MAX_LENGTH, 0, "MAX_SIZE must be greater than 0");

	#[repr(align(4_096))]
	struct AlignedCChars([c_char; BUFFER_SIZE]);

	impl AlignedCChars {
		fn new() -> Self {
			Self([0; BUFFER_SIZE])
		}
	}

	#[inline(always)]
	fn bench_functions(
		rng: &mut ThreadRng,
		group: &mut BenchmarkGroup<WallTime>,
		s: &mut [c_char],
		n: usize,
	) {
		rng.fill(s[..n].as_mut());
		for i in 0..FN_NB {
			group.bench_with_input(BenchmarkId::new(NAMES[i], n), &(), |b, _| {
				b.iter(|| unsafe {
					CALLS[i](s.as_ptr());
				});
			});
		}
	}

	let mut rng: ThreadRng = thread_rng();
	let mut group: BenchmarkGroup<WallTime> = c.benchmark_group("strlen");
	let mut s: AlignedCChars = AlignedCChars::new();
	let s: &mut [c_char] = &mut s.0[OFFSET..OFFSET + MAX_LENGTH];
	let mut n: usize = 1;

	while n <= MAX_LENGTH {
		bench_functions(&mut rng, &mut group, s, n);
		n *= 2;
	}
	for n in (0..MAX_LENGTH).step_by(11) {
		bench_functions(&mut rng, &mut group, s, n);
	}

	group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
