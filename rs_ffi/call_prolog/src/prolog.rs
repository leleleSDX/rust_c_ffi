use ::core::{
	ffi::{
		c_char,
		c_int,
	},
};
use ::std::{
	ffi::{
		CString,
	},
};
use crate::{
	ffi::{
		PL_initialize,
	},
	prelude::{
		base::{
			CR,
		},
	},
};

pub struct Prolog;
impl Prolog
{
	pub fn init()-> i32
	{
		let args = vec![
			CString::new("rust-prolog").unwrap(),
			CString::new("-q").unwrap()
		];
		let mut argv: Vec<*mut c_char>
		= args.iter()
			.map(|s| s.as_ptr() as *mut c_char)
			.collect();
		let argc = argv.len() as c_int;
		let ok 
		= unsafe {
				PL_initialize(argc, argv.as_mut_ptr())
		};
		ok
	}
}