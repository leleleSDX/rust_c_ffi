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
const LIB_DIR: &str = "";
//const LIB_FILE: &str = "libhello.so";
const LIB_NAME: &str = "hello";
fn main()-> SR_
{
	let crate_root 
	= env::var("CARGO_MANIFEST_DIR")?;
	let lib_dir 
	= Path::new(&crate_root).join(LIB_DIR);
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
	OK_
}