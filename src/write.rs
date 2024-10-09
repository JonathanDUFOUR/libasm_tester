use {
	errno::{errno, Errno},
	libc::pipe,
	std::{
		ffi::{c_int, c_void},
		fs::File,
		io::Read,
		os::fd::{AsRawFd, FromRawFd},
	},
};

#[link(name = "asm")]
extern "C" {
	fn ft_write(fd: c_int, buf: *const c_void, count: usize) -> isize;
}

pub fn helper(buf: &[u8], count: usize) {
	assert!(count <= buf.len(), "count must be less than or equal to buf.len()");

	let (mut reader, writer) = {
		// region: Pipe creation
		let mut fds: [c_int; 2] = [0; 2];
		assert_ne!(unsafe { pipe(fds.as_mut_ptr()) }, -1, "Failed to create a pipe");

		unsafe { (File::from_raw_fd(fds[0]), File::from_raw_fd(fds[1])) }
		// endregion
	};
	let ret: isize = unsafe { ft_write(writer.as_raw_fd(), buf.as_ptr() as *const c_void, count) };

	drop(writer);
	assert_ne!(ret, -1, "Failed to write to the pipe");
	assert_eq!(ret as usize, count, "Wrong returned value");
	assert_eq!(errno(), Errno(0), "Wrong errno");

	let bytes_written: Vec<u8> = {
		// region: bytes_written
		let mut tmp: Vec<u8> = Default::default();

		match reader.read_to_end(tmp.as_mut()) {
			Ok(total_number_of_bytes_read) => assert_eq!(
				total_number_of_bytes_read, count,
				"Wrong number of bytes effectively written"
			),
			Err(_) => panic!("Failed to read from the pipe"),
		}

		tmp
		// endregion
	};

	drop(reader);
	assert_eq!(buf[..count], bytes_written, "Wrong bytes written");
}
