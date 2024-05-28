use {
	criterion::{
		criterion_group, criterion_main, measurement::WallTime, BenchmarkGroup, BenchmarkId,
		Criterion,
	},
	rand::{rngs::ThreadRng, Rng},
	std::ffi::c_void,
};

#[link(name = "asm")]
extern "C" {
	fn ft_memcpy(dst: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
}

extern "C" {
	fn memcpy(dst: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
}

fn criterion_benchmark(c: &mut Criterion) {
	type Fn = unsafe extern "C" fn(*mut c_void, *const c_void, usize) -> *mut c_void;

	const FN_NB: usize = 2;
	const NAMES: [&str; FN_NB] = ["ft", "std"];
	const CALLS: [Fn; FN_NB] = [ft_memcpy, memcpy];
	const STEP: usize = 11;
	const MAX_SIZE: usize = 1_024;

	#[inline(always)]
	fn bench(
		group: &mut BenchmarkGroup<WallTime>,
		name: &str,
		call: Fn,
		dst: &mut [u8],
		src: &[u8],
		n: usize,
	) {
		group.bench_with_input(BenchmarkId::new(name, n), &(), |b, _| {
			b.iter(|| unsafe {
				call(dst.as_mut_ptr() as *mut c_void, src.as_ptr() as *const c_void, n);
			});
		});
	}

	let mut group: BenchmarkGroup<WallTime> = c.benchmark_group("memcpy");
	let mut dsts: [[u8; MAX_SIZE]; FN_NB] = [[0; MAX_SIZE]; FN_NB];
	let mut src: [u8; MAX_SIZE] = [0; MAX_SIZE];
	let mut rng: ThreadRng = rand::thread_rng();
	let mut n: usize = 1;

	while n <= MAX_SIZE {
		rng.fill(&mut src[..n]);
		for i in 0..FN_NB {
			bench(&mut group, NAMES[i], CALLS[i], &mut dsts[i], &src, n);
		}
		n *= 2;
	}
	for n in (0..MAX_SIZE).step_by(STEP) {
		rng.fill(&mut src[..n]);
		for i in 0..FN_NB {
			bench(&mut group, NAMES[i], CALLS[i], &mut dsts[i], &src, n);
		}
	}

	group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
