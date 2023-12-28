use std::ffi::{c_char, CString};

extern "C" {
	fn ft_strlen(s: *const c_char) -> usize;
}

#[inline(always)]
fn unit_test_helper(s: &str) {
	let s: CString = CString::new(s).unwrap();

	assert_eq!(unsafe { ft_strlen(s.as_ptr()) }, s.as_bytes().len());
}

// region: ft_strlen_00
#[test]
fn ft_strlen_00() {
	unit_test_helper("");
}
// endregion

// region: ft_strlen_01
#[test]
fn ft_strlen_01() {
	unit_test_helper("0");
}
// endregion

// region: ft_strlen_02
#[test]
fn ft_strlen_02() {
	unit_test_helper("42");
}
// endregion

// region: ft_strlen_03
#[test]
fn ft_strlen_03() {
	unit_test_helper("Hello, World!");
}
// endregion

// region: ft_strlen_04
#[test]
fn ft_strlen_04() {
	unit_test_helper(
		"\
	Let's dance in style, let's dance for a while\n\
	Heaven can wait, we're only watching the skies\n\
	Hoping for the best but expecting the worst\n\
	Are you going to drop the bomb or not?\n\
	\n\
	Let us die young or let us live forever\n\
	We don't have the power but we never say never\n\
	Sitting in a sandpit, life is a short trip\n\
	The music's for the sad men\n\
	\n\
	Can you imagine when the race is won\n\
	Turn our golden faces into the sun\n\
	Praising our leaders, we're getting in tune\n\
	The music's played by the, the madman\n\
	\n\
	Forever young, I want to be forever young\n\
	Do you really want to live forever?\n\
	Forever, and ever\n\
	Forever young, I want to be forever young\n\
	Do you really want to live forever?\n\
	Forever young\n\
	\n\
	Some are like water, some are like the heat\n\
	Some are a melody and some are the beat\n\
	Sooner or later, they all will be gone\n\
	Why don't they stay young?\n\
	\n\
	It's so hard to get old without a cause\n\
	I don't want to perish like a fading horse\n\
	Youth's like diamonds in the sun\n\
	And diamonds are forever\n\
	\n\
	So many adventures couldn't happen today\n\
	So many songs we forgot to play\n\
	So many dreams swinging out of the blue\n\
	We'll let them come true\n\
	\n\
	Forever young, I want to be forever young\n\
	Do you really want to live forever?\n\
	Forever, and ever\n\
	Forever young, I want to be forever young\n\
	Do you really want to live forever?\n\
	Forever, and ever\n\
	\n\
	Forever young, I want to be forever young\n\
	Do you really want to live forever? (Forever)\n",
	);
}
// endregion
