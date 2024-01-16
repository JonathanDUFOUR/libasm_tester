#[cfg(test)]
mod tests {
	use libasm_tester::t_list;
	use std::{
		ffi::{c_int, c_void},
		ptr::null_mut,
	};

	#[link(name = "asm")]
	extern "C" {
		fn ft_list_size(list: *const t_list) -> c_int;
	}

	#[inline(always)]
	fn unit_test_helper(data: &[*mut c_void]) {
		assert!(!data.is_empty());

		let mut nodes: Vec<t_list> =
			data.iter().map(|data| t_list { data: *data, next: null_mut() }).collect();

		for i in 0..nodes.len() - 1 {
			nodes[i].next = &mut nodes[i + 1];
		}
		assert_eq!(unsafe { ft_list_size(nodes.as_ptr()) }, nodes.len() as c_int);
	}

	// region: ft_list_size_00
	#[test]
	fn ft_list_size_00() {
		assert_eq!(unsafe { ft_list_size(null_mut()) }, 0);
	}
	// endregion

	// region: ft_list_size_01
	#[test]
	fn ft_list_size_01() {
		unit_test_helper(&[null_mut()]);
	}
	// endregion

	// region: ft_list_size_02
	#[test]
	fn ft_list_size_02() {
		unit_test_helper(&[
			&mut 0xa0542bcdu32 as *mut _ as *mut c_void,
			&mut 0xf5921bd8u32 as *mut _ as *mut c_void,
		]);
	}
	// endregion

	// region: ft_list_size_03
	#[test]
	fn ft_list_size_03() {
		unit_test_helper(&[
			&mut "Hi" as *mut _ as *mut c_void,
			&mut "there" as *mut _ as *mut c_void,
			&mut "How" as *mut _ as *mut c_void,
			&mut "are" as *mut _ as *mut c_void,
			&mut "you" as *mut _ as *mut c_void,
		]);
	}
	// endregion

	// region: ft_list_size_04
	#[test]
	fn ft_list_size_04() {
		unit_test_helper(&[
			&mut 242u8 as *mut _ as *mut c_void,
			&mut 17299u16 as *mut _ as *mut c_void,
			&mut 2371597596u32 as *mut _ as *mut c_void,
			&mut 7492647612764487385u64 as *mut _ as *mut c_void,
			&mut 84379664544895219780130836660816119438u128 as *mut _ as *mut c_void,
			&mut -26i8 as *mut _ as *mut c_void,
			&mut -30084i16 as *mut _ as *mut c_void,
			&mut -686473075i32 as *mut _ as *mut c_void,
			&mut -4281136400791984104i64 as *mut _ as *mut c_void,
			&mut -24202283039553026342690646794959372026i128 as *mut _ as *mut c_void,
			&mut 55938292887.209505257986582117620f32 as *mut _ as *mut c_void,
			&mut 19194834770157109268519.099941875444714699041f64 as *mut _ as *mut c_void,
		]);
	}
	// endregion
}
