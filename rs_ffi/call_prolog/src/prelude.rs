pub mod format
{
	pub use core::fmt::{
		Display, Debug,
		Formatter as Fmt, Result as FmtR,
	};
}
pub mod base
{
	pub use core::{
		result::{
			Result as CR,
		},
		error::{
			Error as CE,
		},
		option::{
			Option as CO,
		},
	};
}
pub mod box_r
{
	use core::{
		error::{
			Error as TraitError,
		}
	};
	pub use std::{
		boxed::{
			Box,
		},
	};
	pub type BE = Box<dyn TraitError>;
	pub type BR<T> = Result<T, BE>;
	pub type BR_ = BR<()>;
	pub const BOK_: BR_ = Ok(());
}