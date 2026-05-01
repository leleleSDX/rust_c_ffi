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
fn hello_c()
{
	unsafe{
		hello();
	}
}
fn hello_name_c(
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
fn give_back_c(num: i32)
{
	println!("rust> giving {num} to C");	
	let got_num
	= unsafe{
		give_back(num)
	};
	println!("rust> got back {got_num} from C");
}
fn main()-> SR_
{
	give_back_c(12);
	OK_
}
