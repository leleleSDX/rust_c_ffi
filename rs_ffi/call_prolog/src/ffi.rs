use ::core::{
	ffi::{
		c_char, c_int,
	},
};
#[link(name = "swipl")]
unsafe extern "C"
{
	pub unsafe fn PL_initialize(
		argc: c_int, 
		argv: *mut *mut c_char
	)-> c_int;
}
