use errno::{errno, Errno};
use std::{
	ffi::{c_int, c_void},
	fs::{remove_file, write, File},
	io::Write,
	os::fd::AsRawFd,
};

extern "C" {
	fn ft_read(fd: c_int, buf: *mut c_void, count: usize) -> isize;
}

#[inline(always)]
fn unit_test_helper(path: &str, content: &str, count: usize) {
	match write(path, content.as_bytes()) {
		Ok(_) => (),
		Err(_) => panic!("Failed to write to file"),
	};
	let file: File = match File::open(path) {
		Ok(file) => file,
		Err(_) => panic!("Failed to open file"),
	};

	let fd: c_int = file.as_raw_fd();
	let mut buf: Vec<u8> = vec![0; count];
	let expected: usize = content.len().min(count);
	let ret: isize = unsafe { ft_read(fd, buf.as_mut_ptr() as *mut c_void, count) };

	match remove_file(path) {
		Ok(_) => (),
		Err(_) => panic!("Failed to remove file"),
	};
	assert_eq!(ret, expected as isize);
	assert_eq!(buf[..expected], content.as_bytes()[..expected]);
}

// region: ft_read_00
#[test]
fn ft_read_00() {
	unit_test_helper("ft_read_00.txt", "", 0);
}
// endregion

// region: ft_read_01
#[test]
fn ft_read_01() {
	unit_test_helper("ft_read_01.txt", "This time, the file won't be empty", 0);
}
// endregion

// region: ft_read_02
#[test]
fn ft_read_02() {
	unit_test_helper(
		"ft_read_02.txt",
		"42 is the answer to the life, the universe, and everything",
		42,
	);
}
// endregion

// region: ft_read_03
#[test]
fn ft_read_03() {
	unit_test_helper("ft_read_03.txt", "Do you know the way?", 125);
}
// endregion

// region: ft_read_04
#[test]
fn ft_read_04() {
	unit_test_helper("ft_read_04.txt", "I like trains", 13);
}
// endregion

// region: ft_read_05
#[test]
fn ft_read_05() {
	assert_eq!(unsafe { ft_read(-1, std::ptr::null_mut(), 0) }, -1);
	assert_eq!(errno(), Errno(9));
}
// endregion

// region: ft_read_06
#[test]
fn ft_read_06() {
	const PATH: &str = "ft_read_06.txt";
	let mut file: File = match File::create(PATH) {
		Ok(file) => file,
		Err(_) => panic!("Failed to create file"),
	};

	match file.write_all(b"Can't touch this") {
		Ok(_) => (),
		Err(_) => panic!("Failed to write to file"),
	};

	let ret: isize = unsafe { ft_read(file.as_raw_fd(), std::ptr::null_mut(), 0) };

	match remove_file(PATH) {
		Ok(_) => (),
		Err(_) => panic!("Failed to remove file"),
	};
	assert_eq!(ret, -1);
	assert_eq!(errno(), Errno(9));
}
// endregion

// region: ft_read_07
#[test]
fn ft_read_07() {
	let file: File = match File::open("./") {
		Ok(file) => file,
		Err(_) => panic!("Failed to open file"),
	};

	assert_eq!(unsafe { ft_read(file.as_raw_fd(), std::ptr::null_mut(), 0) }, -1);
	assert_eq!(errno(), Errno(21));
}
// endregion

// region: ft_read_08
#[test]
fn ft_read_08() {
	const PATH: &str = "ft_read_08.txt";

	match write(PATH, "Omae wa mou shindeiru") {
		Ok(_) => (),
		Err(_) => panic!("Failed to write to file"),
	};

	let file: File = match File::open(PATH) {
		Ok(file) => file,
		Err(_) => panic!("Failed to open file"),
	};

	let ret: isize = unsafe { ft_read(file.as_raw_fd(), std::ptr::null_mut(), 1) };

	match remove_file(PATH) {
		Ok(_) => (),
		Err(_) => panic!("Failed to remove file"),
	};

	assert_eq!(ret, -1);
	assert_eq!(errno(), Errno(14));
}
// endregion
