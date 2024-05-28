use {
	criterion::{
		criterion_group, criterion_main, measurement::WallTime, BenchmarkGroup, BenchmarkId,
		Criterion,
	},
	rand::{rngs::ThreadRng, Rng},
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
	type Fn = unsafe extern "C" fn(*const c_char) -> usize;

	const FN_NB: usize = 2;
	const NAMES: [&str; FN_NB] = ["ft", "std"];
	const CALLS: [Fn; FN_NB] = [ft_strlen, strlen];
	const STEP: usize = 11;
	const MAX_SIZE: usize = 1_024;

	#[inline(always)]
	fn bench(group: &mut BenchmarkGroup<WallTime>, name: &str, call: Fn, s: &[u8], n: usize) {
		group.bench_with_input(BenchmarkId::new(name, n), &(), |b, _| {
			b.iter(|| unsafe {
				call(s.as_ptr() as *const c_char);
			});
		});
	}

	let mut group: BenchmarkGroup<WallTime> = c.benchmark_group("strlen");
	let mut s: [u8; MAX_SIZE] = [0; MAX_SIZE];
	let mut rng: ThreadRng = rand::thread_rng();
	let mut n: usize = 1;

	while n < MAX_SIZE {
		for c in &mut s[..n] {
			*c = rng.gen_range(1..=255);
		}
		s[n] = 0;
		for i in 0..FN_NB {
			bench(&mut group, NAMES[i], CALLS[i], &s, n);
		}
		n *= 2;
	}
	for n in (0..MAX_SIZE).step_by(STEP) {
		for c in &mut s[..n] {
			*c = rng.gen_range(1..=255);
		}
		s[n] = 0;
		for i in 0..FN_NB {
			bench(&mut group, NAMES[i], CALLS[i], &s, n);
		}
	}

	group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
