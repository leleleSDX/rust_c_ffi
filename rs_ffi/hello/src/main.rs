#![cfg_attr(
	debug_assertions, 
	allow(
		dead_code,
		unused,
	)
)]
use std::{
	borrow::Cow, 
  ffi::{
    CStr,
		CString,
		c_char,
		c_int,
	}
};
use std::{
	boxed::{
		Box,
	},
	error::{
		Error as StdE,
	},
	result::{
		Result as StdR,
	},
};
type SR<T> = StdR<T, Box<dyn StdE>>;
type SR_ = SR<()>;
const OK_: SR_ = Ok(());
unsafe extern "C"
{
	unsafe fn hello();

	unsafe fn hello_name(name: *const c_char);

	unsafe fn give_back(num: c_int)-> c_int;

	unsafe fn modify_num(num: *mut c_int);
	unsafe fn modify_name(name: *mut c_char)-> *mut c_char;
	fn shorten_str(c_str: *mut c_char);
	fn free_str(c_str: *mut c_char);
  fn give_str(gift: *const c_char)-> *mut c_char;
}
fn c_shorten_str(text: &str)
{
	//let cs_ptr = c_str.into_raw();
	unsafe {
		shorten_str(text.as_ptr() as *mut c_char);
	}
}
fn c_free_str(text: &str) -> SR_
{
	//let c_str
	//= CString::new(text)?;
	//let raw_c_ctr = c_str.into_raw();
	unsafe {
		free_str(text.as_ptr() as *mut c_char);
	}
	/*
	let c_str 
	= unsafe {
		CString::from_raw(raw_c_ctr)
	};
	*/
	OK_
}
fn c_hello()
{
	unsafe{
		hello();
	}
}
fn c_hello_name(
	str: impl AsRef<str>
)-> SR_
{
	let name
	= CString::new(str.as_ref())?;
	unsafe{
		hello_name(name.as_ptr());
	};
	OK_
}
fn c_give_back(num: i32)
{
	println!("rust> giving {num} to C");	
	let got_num
	= unsafe{
		give_back(num)
	};
	println!("rust> got back {got_num} from C");
}
fn c_modify_num(mut num: i32)-> i32
{
	let num_ptr
	= &mut num;
	unsafe{
		modify_num(num_ptr);
	};
	return num;
}
fn c_modify_name(
	str: impl AsRef<str>
)-> SR<String>
{
	let mut name
	= CString::new(str.as_ref())?;
	let raw_name
	= unsafe{
		modify_name(name.into_raw())
	};
	let modified
	= unsafe{
		CString::from_raw(raw_name)
	}.into_string()?;
	Ok(modified)
}
fn c_give_str(gift: &str)-> CString
{
	let c_gift
	= CString::new(gift)
		.unwrap_or_default();
	let raw_c_str = unsafe {
		give_str(c_gift.as_ptr())	
	};
	let c_str = unsafe {
		CString::from_raw(raw_c_str)
	};
	c_str
}
fn main()-> SR_
{
	//let text = format!("hello from Rust");
	let text = c_give_str("hello this is gift");
	dbg!(&text);
	OK_
}
