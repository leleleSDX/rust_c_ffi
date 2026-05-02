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
	path::{
		Path, 
		//PathBuf,
	},
};
type SR<T> = StdR<T, Box<dyn StdE>>;
type SR_ = SR<()>;
const OK_: SR_ = Ok(());
const C_LIB: &str = "../../c_lib";
const LIB_SRC: &str = "src";
const LIB_INC: &str = "include";
const LIB_DIR: &str = "target";
//const LIB_FILE: &str = "libhello.so";
const LIB_NAME: &str = "hello";
fn main()-> SR_
{
	let lib_dir 
	= Path::new(C_LIB).join(LIB_DIR).canonicalize()?;
	println!(
		"cargo:rustc-link-search=native={}",
		lib_dir.display()
	);
	println!(
		"cargo:rustc-link-lib=static={LIB_NAME}"
	);
	let lib_src
	= Path::new(C_LIB).join(LIB_NAME).join(LIB_SRC).canonicalize()?;
	println!(
		"cargo::rerun-if-changed={}",
		lib_src.display()
	);
	let lib_inc
	= Path::new(C_LIB).join(LIB_NAME).join(LIB_INC).canonicalize()?;
	println!(
		"cargo::rerun-if-changed={}",
		lib_inc.display()
	);
	println!("cargo::rerun-if-changed=build.rs");
	OK_
}