#[cfg(test)]
mod write {
	use {
		errno::{errno, Errno},
		libasm_tester::write::helper,
		libc::pipe,
		std::{
			ffi::{c_int, c_void},
			fs::File,
			os::fd::{AsRawFd, FromRawFd},
			ptr::null,
		},
	};

	const EBADFD: c_int = 9;
	const EFAULT: c_int = 14;
	const EPIPE: c_int = 32;

	#[link(name = "asm")]
	extern "C" {
		fn ft_write(fd: c_int, buf: *const c_void, count: usize) -> isize;
	}

	// region: valid_fd_valid_addr_00_00
	#[test]
	fn valid_fd_valid_addr_00_00() {
		helper(&[], 0);
	}
	// endregion
	// region: valid_fd_valid_addr_37_00
	#[test]
	fn valid_fd_valid_addr_37_00() {
		helper(
			&[
				0x77, 0xD7, 0x3F, 0x9E, 0x15, 0x35, 0xA9, 0xCC, 0x13, 0xB8, 0xC2, 0x31, 0x1C, 0xDE,
				0x7D, 0xE0, 0xE9, 0x29, 0x50, 0xD7, 0xA0, 0xAC, 0xAC, 0x16, 0x81, 0x7C, 0x7C, 0x90,
				0x2A, 0xA7, 0xE6, 0xAF, 0xF1, 0xE1, 0xE3, 0xFF, 0x90,
			],
			0,
		);
	}
	// endregion
	// region: valid_fd_valid_addr_58_13
	#[test]
	fn valid_fd_valid_addr_58_13() {
		helper(
			&[
				0xF5, 0xD1, 0x55, 0x39, 0x1E, 0x1E, 0xBC, 0xA5, 0xEE, 0xD0, 0x19, 0x96, 0x64, 0x80,
				0x16, 0x0F, 0x3F, 0xB7, 0x88, 0xD8, 0x81, 0x78, 0x6B, 0x0F, 0xBB, 0x69, 0x2D, 0x83,
				0x1A, 0x57, 0x32, 0x5B, 0xB3, 0x53, 0x83, 0x2F, 0x81, 0xB8, 0x84, 0x44, 0xB6, 0x86,
				0xE8, 0xA0, 0x64, 0x26, 0xC9, 0xA7, 0xC9, 0x88, 0x57, 0x7B, 0xCF, 0x09, 0xDE, 0x09,
				0xD4, 0x09,
			],
			13,
		);
	}
	// endregion
	// region: valid_fd_valid_addr_42_42
	#[test]
	fn valid_fd_valid_addr_42_42() {
		helper(
			&[
				0x65, 0x00, 0x22, 0x17, 0xE2, 0x27, 0x7F, 0x94, 0x29, 0x84, 0x67, 0xC3, 0x99, 0x76,
				0x97, 0xFF, 0xD2, 0x30, 0x15, 0x04, 0xD7, 0xD6, 0xD4, 0xD8, 0xF2, 0x19, 0x6C, 0x9E,
				0xA7, 0xCA, 0x40, 0x57, 0xD1, 0x27, 0x9A, 0xC8, 0x6D, 0x1B, 0xB6, 0x1C, 0x18, 0x6A,
			],
			42,
		);
	}
	// endregion
	// region: valid_fd_wrong_addr_xx_00
	#[test]
	fn valid_fd_wrong_addr_xx_00() {
		let (_, writer) = {
			// region: Pipe creation
			let mut fds: [c_int; 2] = [0; 2];
			assert_ne!(unsafe { pipe(fds.as_mut_ptr()) }, -1, "Failed to create a pipe");

			unsafe { (File::from_raw_fd(fds[0]), File::from_raw_fd(fds[1])) }
			// endregion
		};

		assert_eq!(unsafe { ft_write(writer.as_raw_fd(), null(), 0) }, 0, "Wrong returned value");
		assert_eq!(errno(), Errno(0), "Wrong errno");
	}
	// endregion
	// region: valid_fd_wrong_addr_xx_21
	#[test]
	fn valid_fd_wrong_addr_xx_21() {
		let (_reader, writer) = {
			// region: Pipe creation
			let mut fds: [c_int; 2] = [0; 2];
			assert_ne!(unsafe { pipe(fds.as_mut_ptr()) }, -1, "Failed to create a pipe");

			unsafe { (File::from_raw_fd(fds[0]), File::from_raw_fd(fds[1])) }
			// endregion
		};

		assert_eq!(unsafe { ft_write(writer.as_raw_fd(), null(), 21) }, -1, "Wrong returned value");
		assert_eq!(errno(), Errno(EFAULT), "Wrong errno");
	}
	// endregion
	// region: wrong_fd_valid_addr_00_00
	#[test]
	fn wrong_fd_valid_addr_00_00() {
		const BUFFER: [u8; 0] = [];
		const COUNT: usize = 0;

		assert_eq!(
			unsafe { ft_write(-1, BUFFER.as_ptr() as *const c_void, COUNT) },
			-1,
			"Wrong returned value"
		);
		assert_eq!(errno(), Errno(EBADFD), "Wrong errno");
	}
	// endregion
	// region: wrong_fd_valid_addr_00_17
	#[test]
	fn wrong_fd_valid_addr_00_17() {
		const BUFFER: [u8; 0] = [];
		const COUNT: usize = 17;

		assert_eq!(
			unsafe { ft_write(-1, BUFFER.as_ptr() as *const c_void, COUNT) },
			-1,
			"Wrong returned value"
		);
		assert_eq!(errno(), Errno(EBADFD), "Wrong errno");
	}
	// endregion
	// region: wrong_fd_valid_addr_33_00
	#[test]
	fn wrong_fd_valid_addr_33_00() {
		const BUFFER: [u8; 33] = [
			// region: BUFFER
			0xD2, 0x6C, 0xDA, 0xCE, 0xF4, 0x77, 0x94, 0x7E, 0x70, 0x9A, 0x5D, 0x6C, 0x9E, 0x61,
			0x1A, 0xF7, 0xB7, 0xFB, 0x85, 0xB8, 0xA9, 0x87, 0xC6, 0x9E, 0xC6, 0x04, 0xF1, 0x89,
			0xB1, 0x8F, 0xB9, 0x23, 0x87,
			// endregion
		];
		const COUNT: usize = 0;

		assert_eq!(
			unsafe { ft_write(-1, BUFFER.as_ptr() as *const c_void, COUNT) },
			-1,
			"Wrong returned value"
		);
		assert_eq!(errno(), Errno(EBADFD), "Wrong errno");
	}
	// endregion
	// region: wrong_fd_valid_addr_27_15
	#[test]
	fn wrong_fd_valid_addr_27_15() {
		const BUFFER: [u8; 27] = [
			// region: BUFFER
			0xBC, 0x73, 0x5F, 0x32, 0xDA, 0xED, 0xAB, 0xCD, 0x8A, 0x3B, 0x3A, 0x7D, 0xD3, 0xDF,
			0xBF, 0x4A, 0xD6, 0x26, 0x52, 0xAB, 0xE2, 0xCD, 0x62, 0x18, 0xCD, 0x6A,
			0xA4,
			// endregion
		];
		const COUNT: usize = 15;

		assert_eq!(
			unsafe { ft_write(-1, BUFFER.as_ptr() as *const c_void, COUNT) },
			-1,
			"Wrong returned value"
		);
		assert_eq!(errno(), Errno(EBADFD), "Wrong errno");
	}
	// endregion
	// region: wrong_fd_valid_addr_11_11
	#[test]
	fn wrong_fd_valid_addr_11_11() {
		const BUFFER: [u8; 11] = [
			0x8B, 0xCF, 0xBE, 0xFB, 0xEA, 0xE3, 0x8D, 0xEA, 0xB8, 0xF6, 0xBC,
		];
		const COUNT: usize = 11;

		assert_eq!(
			unsafe { ft_write(-1, BUFFER.as_ptr() as *const c_void, COUNT) },
			-1,
			"Wrong returned value"
		);
		assert_eq!(errno(), Errno(EBADFD), "Wrong errno");
	}
	// endregion
	// region: wrong_fd_wrong_addr_xx_00
	#[test]
	fn wrong_fd_wrong_addr_xx_00() {
		assert_eq!(unsafe { ft_write(-1, null(), 0) }, -1, "Wrong returned value");
		assert_eq!(errno(), Errno(EBADFD));
	}
	// endregion
	// region: wrong_fd_wrong_addr_xx_14
	#[test]
	fn wrong_fd_wrong_addr_xx_14() {
		assert_eq!(unsafe { ft_write(-1, null(), 14) }, -1, "Wrong returned value");
		assert_eq!(errno(), Errno(EBADFD));
	}
	// endregion
	// region: broken_pipe_valid_addr_00_00
	#[test]
	fn broken_pipe_valid_addr_00_00() {
		const BUFFER: [u8; 0] = [];
		const COUNT: usize = 0;

		let (reader, writer) = {
			// region: Pipe creation
			let mut fds: [c_int; 2] = [0; 2];
			assert_ne!(unsafe { pipe(fds.as_mut_ptr()) }, -1, "Failed to create a pipe");

			unsafe { (File::from_raw_fd(fds[0]), File::from_raw_fd(fds[1])) }
			// endregion
		};

		drop(reader);
		assert_eq!(
			unsafe { ft_write(writer.as_raw_fd(), BUFFER.as_ptr() as *const c_void, COUNT) },
			0,
			"Wrong returned value"
		);
		assert_eq!(errno(), Errno(0), "Wrong errno");
	}
	// endregion
	// region: broken_pipe_valid_addr_00_08
	#[test]
	fn broken_pipe_valid_addr_00_08() {
		const BUFFER: [u8; 0] = [];
		const COUNT: usize = 8;

		let (reader, writer) = {
			// region: Pipe creation
			let mut fds: [c_int; 2] = [0; 2];
			assert_ne!(unsafe { pipe(fds.as_mut_ptr()) }, -1, "Failed to create a pipe");

			unsafe { (File::from_raw_fd(fds[0]), File::from_raw_fd(fds[1])) }
			// endregion
		};

		drop(reader);
		assert_eq!(
			unsafe { ft_write(writer.as_raw_fd(), BUFFER.as_ptr() as *const c_void, COUNT) },
			-1,
			"Wrong returned value"
		);
		assert_eq!(errno(), Errno(EPIPE), "Wrong errno");
	}
	// endregion
	// region: broken_pipe_valid_addr_23_00
	#[test]
	fn broken_pipe_valid_addr_23_00() {
		const BUFFER: [u8; 23] = [
			// region: BUFFER
			0xBE, 0xD4, 0xF8, 0xB0, 0xEC, 0x29, 0xB3, 0x56, 0xE9, 0x66, 0x27, 0xBE, 0x0C, 0x95,
			0x73, 0x36, 0x07, 0x0E, 0x54, 0x77, 0x97, 0xF0, 0xEE,
			// endregion
		];
		const COUNT: usize = 0;

		let (reader, writer) = {
			// region: Pipe creation
			let mut fds: [c_int; 2] = [0; 2];
			assert_ne!(unsafe { pipe(fds.as_mut_ptr()) }, -1, "Failed to create a pipe");

			unsafe { (File::from_raw_fd(fds[0]), File::from_raw_fd(fds[1])) }
			// endregion
		};

		drop(reader);
		assert_eq!(
			unsafe { ft_write(writer.as_raw_fd(), BUFFER.as_ptr() as *const c_void, COUNT) },
			0,
			"Wrong returned value"
		);
		assert_eq!(errno(), Errno(0), "Wrong errno");
	}
	// endregion
	// region: broken_pipe_valid_addr_39_25
	#[test]
	fn broken_pipe_valid_addr_39_25() {
		const BUFFER: [u8; 39] = [
			// region: BUFFER
			0x36, 0xA1, 0x36, 0x50, 0xAE, 0x83, 0x2D, 0x64, 0x54, 0x0D, 0x5D, 0x99, 0x15, 0x8C,
			0x91, 0x27, 0xEF, 0xBA, 0x4F, 0x53, 0x9E, 0xAE, 0x6D, 0x4A, 0x31, 0xCD, 0xC4, 0x77,
			0x7E, 0x35, 0xF0, 0x41, 0x35, 0xED, 0xB4, 0x29, 0xF3, 0x60, 0x5A,
			// endregion
		];
		const COUNT: usize = 25;

		let (reader, writer) = {
			// region: Pipe creation
			let mut fds: [c_int; 2] = [0; 2];
			assert_ne!(unsafe { pipe(fds.as_mut_ptr()) }, -1, "Failed to create a pipe");

			unsafe { (File::from_raw_fd(fds[0]), File::from_raw_fd(fds[1])) }
			// endregion
		};

		drop(reader);
		assert_eq!(
			unsafe { ft_write(writer.as_raw_fd(), BUFFER.as_ptr() as *const c_void, COUNT) },
			-1,
			"Wrong returned value"
		);
		assert_eq!(errno(), Errno(EPIPE), "Wrong errno");
	}
	// endregion
	// region: broken_pipe_valid_addr_10_10
	#[test]
	fn broken_pipe_valid_addr_10_10() {
		const BUFFER: [u8; 10] = [
			// region: BUFFER
			0x58, 0x1F, 0x28, 0xA1, 0x21, 0xDD, 0x15, 0xEF, 0x2C, 0x5E,
			// endregion
		];
		const COUNT: usize = 10;

		let (reader, writer) = {
			// region: Pipe creation
			let mut fds: [c_int; 2] = [0; 2];
			assert_ne!(unsafe { pipe(fds.as_mut_ptr()) }, -1, "Failed to create a pipe");

			unsafe { (File::from_raw_fd(fds[0]), File::from_raw_fd(fds[1])) }
			// endregion
		};

		drop(reader);
		assert_eq!(
			unsafe { ft_write(writer.as_raw_fd(), BUFFER.as_ptr() as *const c_void, COUNT) },
			-1,
			"Wrong returned value"
		);
		assert_eq!(errno(), Errno(EPIPE), "Wrong errno");
	}
	// endregion
	// region: broken_pipe_wrong_addr_xx_00
	#[test]
	fn broken_pipe_wrong_addr_xx_00() {
		let (reader, writer) = {
			// region: Pipe creation
			let mut fds: [c_int; 2] = [0; 2];
			assert_ne!(unsafe { pipe(fds.as_mut_ptr()) }, -1, "Failed to create a pipe");

			unsafe { (File::from_raw_fd(fds[0]), File::from_raw_fd(fds[1])) }
			// endregion
		};

		drop(reader);
		assert_eq!(unsafe { ft_write(writer.as_raw_fd(), null(), 0) }, 0, "Wrong returned value");
		assert_eq!(errno(), Errno(0), "Wrong errno");
	}
	// endregion
	// region: broken_pipe_wrong_addr_xx_31
	#[test]
	fn broken_pipe_wrong_addr_xx_31() {
		let (reader, writer) = {
			// region: Pipe creation
			let mut fds: [c_int; 2] = [0; 2];
			assert_ne!(unsafe { pipe(fds.as_mut_ptr()) }, -1, "Failed to create a pipe");

			unsafe { (File::from_raw_fd(fds[0]), File::from_raw_fd(fds[1])) }
			// endregion
		};

		drop(reader);
		assert_eq!(unsafe { ft_write(writer.as_raw_fd(), null(), 31) }, -1, "Wrong returned value");
		assert_eq!(errno(), Errno(EPIPE), "Wrong errno");
	}
	// endregion
}
