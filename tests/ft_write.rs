#[cfg(test)]
mod tests {
	use errno::{errno, Errno};
	use std::{
		ffi::{c_int, c_void},
		fs::{read, remove_file, File},
		os::fd::AsRawFd,
		ptr::null,
	};

	extern "C" {
		fn ft_write(fd: c_int, buf: *const c_void, count: usize) -> isize;
	}

	#[inline(always)]
	fn unit_test_helper(path: &str, content: &str, count: usize) {
		let file: File = match File::create(path) {
			Ok(file) => file,
			Err(_) => panic!("Failed to create file"),
		};

		let fd: c_int = file.as_raw_fd();
		let ret: isize = unsafe { ft_write(fd, content.as_ptr() as *const c_void, count) };
		let wrote: Vec<u8> = match read(path) {
			Ok(content) => content,
			Err(_) => panic!("Failed to read file"),
		};

		match remove_file(path) {
			Ok(_) => (),
			Err(_) => panic!("Failed to remove file"),
		};
		assert_eq!(ret, count as isize);
		assert_eq!(wrote, content.as_bytes()[..count]);
	}

	// region: ft_write_00
	#[test]
	fn ft_write_00() {
		unit_test_helper("ft_write_00.txt", "", 0);
	}
	// endregion

	// region: ft_write_01
	#[test]
	fn ft_write_01() {
		unit_test_helper("ft_write_01.txt", "I don't have the force to write ir...", 0);
	}
	// endregion

	// region: ft_write_02
	#[test]
	fn ft_write_02() {
		unit_test_helper(
			"ft_write_02.txt",
			"42 is the answer to the life, the universe, and everything",
			42,
		);
	}
	// endregion

	// region: ft_write_03
	#[test]
	fn ft_write_03() {
		unit_test_helper("ft_write_03.txt", "I like trains", 13);
	}
	// endregion

	// region: ft_write_04
	#[test]
	fn ft_write_04() {
		assert_eq!(unsafe { ft_write(-1, null(), 0) }, -1);
		assert_eq!(errno(), Errno(9));
	}
	// endregion

	// region: ft_write_05
	#[test]
	fn ft_write_05() {
		assert_eq!(unsafe { ft_write(-1, null(), 11) }, -1);
		assert_eq!(errno(), Errno(9));
	}
	// endregion

	// region: ft_write_06
	#[test]
	fn ft_write_06() {
		const PATH: &str = "ft_write_06.txt";

		match File::create(PATH) {
			Ok(_) => (),
			Err(_) => panic!("Failed to create file"),
		};

		let file: File = match File::open(PATH) {
			Ok(file) => file,
			Err(_) => panic!("Failed to open file"),
		};
		let ret: isize = unsafe { ft_write(file.as_raw_fd(), null(), 0) };

		match remove_file(PATH) {
			Ok(_) => (),
			Err(_) => panic!("Failed to remove file"),
		};
		assert_eq!(ret, -1);
		assert_eq!(errno(), Errno(9));
	}
	// endregion

	// region: ft_write_07
	#[test]
	fn ft_write_07() {
		const PATH: &str = "ft_write_07.txt";

		match File::create(PATH) {
			Ok(_) => (),
			Err(_) => panic!("Failed to create file"),
		};

		let file: File = match File::open(PATH) {
			Ok(file) => file,
			Err(_) => panic!("Failed to open file"),
		};
		let ret: isize = unsafe { ft_write(file.as_raw_fd(), null(), 17) };

		match remove_file(PATH) {
			Ok(_) => (),
			Err(_) => panic!("Failed to remove file"),
		};
		assert_eq!(ret, -1);
		assert_eq!(errno(), Errno(9));
	}
	// endregion

	// region: ft_write_08
	#[test]
	fn ft_write_08() {
		const PATH: &str = "ft_write_08.txt";
		let file: File = match File::create(PATH) {
			Ok(file) => file,
			Err(_) => panic!("Failed to create file"),
		};
		let ret: isize = unsafe { ft_write(file.as_raw_fd(), null(), 1) };

		match remove_file(PATH) {
			Ok(_) => (),
			Err(_) => panic!("Failed to remove file"),
		};
		assert_eq!(ret, -1);
		assert_eq!(errno(), Errno(14));
	}
	// endregion
}
