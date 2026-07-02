#![cfg_attr(
	feature = "proto", 
	allow(unused, dead_code)
)]
use call_prolog::{
	prelude::{
		box_r::*,
	},
	prolog::{
		Prolog,
	},
};

fn main()
{
 let prolog = Prolog::init();
 dbg!(&prolog);
}