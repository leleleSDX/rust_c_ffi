use std::path::{
	Path, PathBuf,
};
const LIB_DIR: &str = "../../c_lib/hello";
const LIB_FILE: &str = "libhello.so";
const LIB_NAME: &str = "hello";
const TARGET_DIR: &str = "../target/debug";
fn main()
{
	//let lib_dir = Path::new(LIB_DIR);
	println!("cargo:rustc-link-search={TARGET_DIR}");
	println!("cargo:rustc-link-lib={LIB_NAME}");
}