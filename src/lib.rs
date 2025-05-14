use std::ffi::c_void;

#[derive(Clone, Copy)]
#[repr(C)]
pub struct Node {
	pub data: *mut c_void,
	pub next: *mut Node,
}

pub const MAX_BYTES_PER_LINE: usize = 42;

#[macro_export]
macro_rules! print_bytes_of {
	($bytes:ident) => {
		let modulo: usize = $bytes.len() % MAX_BYTES_PER_LINE;

		for i in (0..$bytes.len() - modulo).step_by(MAX_BYTES_PER_LINE) {
			println!("{:02X?}", &$bytes[i..i + MAX_BYTES_PER_LINE]);
		}
		if modulo > 0 {
			println!("{:02X?}", &$bytes[$bytes.len() - modulo..]);
		}
	};
}
