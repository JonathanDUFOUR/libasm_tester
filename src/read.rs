use std::{
	ffi::{c_int, c_void},
	fs::{remove_file, write, File},
	os::fd::AsRawFd,
};

#[link(name = "asm")]
extern "C" {
	fn ft_read(fd: c_int, buf: *mut c_void, count: usize) -> isize;
}

#[inline(always)]
pub fn helper(path: &str, content: &str, count: usize) {
	match write(path, content.as_bytes()) {
		Ok(_) => (),
		Err(_) => panic!("Failed to write to file"),
	};
	let file: File = match File::open(path) {
		Ok(file) => file,
		Err(_) => panic!("Failed to open file"),
	};

	let mut buf: Vec<u8> = vec![0; count];
	let expected: usize = content.len().min(count);
	let ret: isize = unsafe { ft_read(file.as_raw_fd(), buf.as_mut_ptr() as *mut c_void, count) };

	match remove_file(path) {
		Ok(_) => (),
		Err(_) => panic!("Failed to remove file"),
	};
	assert_eq!(ret, expected as isize);
	assert_eq!(buf[..expected], content.as_bytes()[..expected]);
}
