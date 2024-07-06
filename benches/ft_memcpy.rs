use {
	criterion::{
		criterion_group, criterion_main, measurement::WallTime, BenchmarkGroup, BenchmarkId,
		Criterion,
	},
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
	use rand::{rngs::ThreadRng, thread_rng, Rng};

	type Fn = unsafe extern "C" fn(*mut c_void, *const c_void, usize) -> *mut c_void;

	const FN_NB: usize = 2;
	const NAMES: [&str; FN_NB] = ["ft", "std"];
	const CALLS: [Fn; FN_NB] = [ft_memcpy, memcpy];
	const MAX_LENGTH: usize = 1_024;
	const DST_OFFSET: usize = 0;
	const SRC_OFFSET: usize = 0;
	const DST_BUFFER_SIZE: usize = MAX_LENGTH + DST_OFFSET;
	const SRC_BUFFER_SIZE: usize = MAX_LENGTH + SRC_OFFSET;
	const ALIGN: usize = std::mem::align_of::<AlignedDst>();

	assert!(DST_OFFSET < ALIGN, "DST_OFFSET must be less than ALIGN");
	assert!(SRC_OFFSET < ALIGN, "SRC_OFFSET must be less than ALIGN");
	assert_ne!(MAX_LENGTH, 0, "MAX_SIZE must be greater than 0");

	#[repr(align(4_096))]
	#[derive(Clone, Copy)]
	struct AlignedDst([u8; DST_BUFFER_SIZE]);

	impl AlignedDst {
		fn new() -> Self {
			Self([0; DST_BUFFER_SIZE])
		}
	}

	#[repr(align(4096))]
	struct AlignedSrc([u8; SRC_BUFFER_SIZE]);

	impl AlignedSrc {
		fn new() -> Self {
			Self([0; SRC_BUFFER_SIZE])
		}
	}

	#[inline(always)]
	fn bench_functions(
		rng: &mut ThreadRng,
		group: &mut BenchmarkGroup<WallTime>,
		dsts: &mut [&mut [u8]],
		src: &mut [u8],
		n: usize,
	) {
		rng.fill(src[..n].as_mut());
		for i in 0..FN_NB {
			group.bench_with_input(BenchmarkId::new(NAMES[i], n), &(), |b, _| {
				b.iter(|| unsafe {
					CALLS[i](dsts[i].as_mut_ptr() as *mut c_void, src.as_ptr() as *const c_void, n);
				});
			});
		}
	}

	let mut rng: ThreadRng = thread_rng();
	let mut group: BenchmarkGroup<WallTime> = c.benchmark_group("memcpy");
	let mut dsts: [AlignedDst; FN_NB] = [AlignedDst::new(); FN_NB];
	let mut dsts: [&mut [u8]; FN_NB] = {
		// region: dsts
		let mut tmp: [&mut [u8]; FN_NB] = Default::default();

		for (i, dst) in dsts.iter_mut().enumerate() {
			tmp[i] = &mut dst.0[DST_OFFSET..DST_OFFSET + MAX_LENGTH];
		}

		tmp
		// endregion: dsts
	};
	let mut src: AlignedSrc = AlignedSrc::new();
	let src: &mut [u8] = &mut src.0[SRC_OFFSET..SRC_OFFSET + MAX_LENGTH];
	let mut n: usize = 1;

	while n <= MAX_LENGTH {
		bench_functions(&mut rng, &mut group, &mut dsts, src, n);
		n *= 2;
	}
	for n in (0..MAX_LENGTH).step_by(11) {
		bench_functions(&mut rng, &mut group, &mut dsts, src, n);
	}
	group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
