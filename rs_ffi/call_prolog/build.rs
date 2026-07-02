use ::std::{
	path::{
		self,
	},
};
const LIB_DIR: &str
= "/usr/lib/swipl/lib/x86_64-linux";
const LIB: &str = "swipl";

fn main()
{
	let lib_dir = path::Path::new(LIB_DIR)
		.canonicalize()
		.expect("{LIB_DIR} exists");
	println!("cargo:rerun-if-changed=build.rs");
	println!("cargo:rustc-link-search=native={}", lib_dir.display());
	println!("cargo:rustc-link-lib={LIB}");
}
