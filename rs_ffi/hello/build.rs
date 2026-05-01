use std::{
	boxed::{
		Box,
	},
	env,
	error::{
		Error as StdE,
	},
	result::{
		Result as StdR,
	},
	path::{
		Path, PathBuf,
	},
};
type SR<T> = StdR<T, Box<dyn StdE>>;
type SR_ = SR<()>;
const OK_: SR_ = Ok(());
const LIB_SRC: &str = "../../c_lib/hello/src";
const LIB_INC: &str = "../../c_lib/hello/include";
const LIB_DIR: &str = "../../c_lib/target";
//const LIB_FILE: &str = "libhello.so";
const LIB_NAME: &str = "hello";
fn main()-> SR_
{
	//let crate_root 
	//= env::var("CARGO_MANIFEST_DIR")?;
	let lib_dir 
	= Path::new(LIB_DIR).canonicalize()?;
	if lib_dir.try_exists()? == false
	{
		return Err("lib dir not found".into());
	}
	println!(
		"cargo:rustc-link-search=native={}",
		lib_dir.display()
	);
	println!(
		"cargo:rustc-link-lib=static={LIB_NAME}"
	);
	let lib_src
	= Path::new(LIB_SRC).canonicalize()?;
	println!(
		"cargo::rerun-if-changed={}",
		lib_src.display()
	);
	let lib_inc
	= Path::new(LIB_INC).canonicalize()?;
	println!(
		"cargo::rerun-if-changed={}",
		lib_inc.display()
	);
	println!("cargo::rerun-if-changed=build.rs");
	OK_
}