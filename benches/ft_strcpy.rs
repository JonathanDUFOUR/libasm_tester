use {
	criterion::{
		criterion_group, criterion_main, measurement::WallTime, BenchmarkGroup, BenchmarkId,
		Criterion,
	},
	rand::{rngs::ThreadRng, Rng},
	std::ffi::c_char,
};

type Fn = unsafe extern "C" fn(*mut c_char, *const c_char) -> *mut c_char;

#[link(name = "asm")]
extern "C" {
	fn ft_strcpy(dst: *mut c_char, src: *const c_char) -> *mut c_char;
	// fn ft_strcpy_alt(dst: *mut c_char, src: *const c_char) -> *mut c_char;
}

extern "C" {
	fn strcpy(dst: *mut c_char, src: *const c_char) -> *mut c_char;
}

fn criterion_benchmark(c: &mut Criterion) {
	const MAX_SIZE: usize = 1_024;
	const FN_NB: usize = 2;
	const STEP: usize = 11;

	#[inline(always)]
	fn bench(
		group: &mut BenchmarkGroup<WallTime>,
		name: &str,
		call: Fn,
		dst: &mut [c_char],
		src: &[c_char],
		n: usize,
	) {
		group.bench_with_input(BenchmarkId::new(name, n), &(), |b, _| {
			b.iter(|| unsafe {
				call(dst.as_mut_ptr(), src.as_ptr());
			});
		});
	}

	let mut group: BenchmarkGroup<WallTime> = c.benchmark_group("ft_strcpy");
	let names: [&str; FN_NB] = ["main", "std"];
	let calls: [Fn; FN_NB] = [ft_strcpy, strcpy];
	let mut dsts: [[c_char; MAX_SIZE]; FN_NB] = [[0; MAX_SIZE]; FN_NB];
	let mut src: [c_char; MAX_SIZE] = [0; MAX_SIZE];
	let mut rng: ThreadRng = rand::thread_rng();
	let mut n: usize = 1;

	while n <= MAX_SIZE {
		rng.fill(&mut src[..n - 1]);
		for i in 0..FN_NB {
			bench(&mut group, names[i], calls[i], &mut dsts[i], &src, n);
		}
		n *= 2;
	}

	for n in (STEP..MAX_SIZE).step_by(STEP) {
		rng.fill(&mut src[..n - 1]);
		for i in 0..FN_NB {
			bench(&mut group, names[i], calls[i], &mut dsts[i], &src, n);
		}
	}

	group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
