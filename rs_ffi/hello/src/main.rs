unsafe extern "C"
{
	unsafe fn hello();
}
fn hello_c()
{
	unsafe{
		hello();
	}
}
fn main() {
	println!("hello from rust");
	hello_c();
}
