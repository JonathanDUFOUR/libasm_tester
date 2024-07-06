use {
	criterion::{
		criterion_group, criterion_main, measurement::WallTime, BenchmarkGroup, BenchmarkId,
		Criterion,
	},
	std::ffi::c_char,
};

#[link(name = "asm")]
extern "C" {
	fn ft_strcpy(dst: *mut c_char, src: *const c_char) -> *mut c_char;
}

extern "C" {
	fn strcpy(dst: *mut c_char, src: *const c_char) -> *mut c_char;
}

fn criterion_benchmark(c: &mut Criterion) {
	use rand::{rngs::ThreadRng, thread_rng, Rng};

	type Fn = unsafe extern "C" fn(*mut c_char, *const c_char) -> *mut c_char;

	const FN_NB: usize = 2;
	const NAMES: [&str; FN_NB] = ["ft", "std"];
	const CALLS: [Fn; FN_NB] = [ft_strcpy, strcpy];
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
	struct AlignedDst([c_char; DST_BUFFER_SIZE]);

	impl AlignedDst {
		fn new() -> Self {
			Self([0; DST_BUFFER_SIZE])
		}
	}

	#[repr(align(4096))]
	struct AlignedSrc([c_char; SRC_BUFFER_SIZE]);

	impl AlignedSrc {
		fn new() -> Self {
			Self([0; SRC_BUFFER_SIZE])
		}
	}

	#[inline(always)]
	fn bench_functions(
		rng: &mut ThreadRng,
		group: &mut BenchmarkGroup<WallTime>,
		dsts: &mut [&mut [c_char]],
		src: &mut [c_char],
		src_len: usize,
	) {
		rng.fill(src[..src_len].as_mut());
		for i in 0..FN_NB {
			group.bench_with_input(BenchmarkId::new(NAMES[i], src_len), &(), |b, _| {
				b.iter(|| unsafe {
					CALLS[i](dsts[i].as_mut_ptr(), src.as_ptr());
				});
			});
		}
	}

	let mut rng: ThreadRng = thread_rng();
	let mut group: BenchmarkGroup<WallTime> = c.benchmark_group("strcpy");
	let mut dsts: [AlignedDst; FN_NB] = [AlignedDst::new(); FN_NB];
	let mut dsts: [&mut [c_char]; FN_NB] = {
		// region: dsts
		let mut tmp: [&mut [c_char]; FN_NB] = Default::default();

		for (i, dst) in dsts.iter_mut().enumerate() {
			tmp[i] = &mut dst.0[DST_OFFSET..DST_OFFSET + MAX_LENGTH];
		}

		tmp
		// endregion: dsts
	};
	let mut src: AlignedSrc = AlignedSrc::new();
	let src: &mut [c_char] = &mut src.0[SRC_OFFSET..SRC_OFFSET + MAX_LENGTH];
	let mut src_len: usize = 1;

	while src_len <= MAX_LENGTH {
		bench_functions(&mut rng, &mut group, &mut dsts, src, src_len);
		src_len *= 2;
	}
	for src_len in (0..MAX_LENGTH).step_by(11) {
		bench_functions(&mut rng, &mut group, &mut dsts, src, src_len);
	}
	group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
