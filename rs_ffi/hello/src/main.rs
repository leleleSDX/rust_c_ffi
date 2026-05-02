#![cfg_attr(
	debug_assertions, 
	allow(
		dead_code,
		unused,
	)
)]
use std::{
	ffi::{
		CString,
		c_char,
		c_int,
	},
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
}
unsafe extern "C"
{
	unsafe fn hello_name(name: *const c_char);
}
unsafe extern "C"
{
	unsafe fn give_back(num: c_int)-> c_int;
}
unsafe extern "C"
{
	unsafe fn modify_num(num: *mut c_int);
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
fn c_modify_num(num: i32)-> i32
{
	let mut num = num;
	let num_ptr
	= &mut num;
	unsafe{
		modify_num(num_ptr);
	};
	return num;
}
fn main()-> SR_
{
	let num = c_modify_num(-14);
	println!("rust> got {num} back from C");
	OK_
}
