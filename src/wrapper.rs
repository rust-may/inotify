use libc;
use libc::{
	c_int,
	c_void };
use std::c_str::CString;
use std::mem;
use std::io::{
	EndOfFile,
	IoError,
	IoResult
};
use std::os;
use std::ptr;

use ffi;
use ffi::inotify_event;


pub type Watch = c_int;


pub struct INotify {
	pub fd: c_int
}

impl INotify {
	pub fn init() -> IoResult<INotify> {
		INotify::init_with_flags(0)
	}

	pub fn init_with_flags(flags: int) -> IoResult<INotify> {
		let fd = unsafe { ffi::inotify_init1(flags as c_int) };

		match fd {
			-1 => Err(IoError::last_error()),
			_  => Ok(INotify { fd: fd })
		}
	}

	pub fn add_watch(&self, path_name: &str, mask: u32) -> IoResult<Watch> {
		let wd = unsafe {
			let c_path_name = path_name.to_c_str().unwrap();
			ffi::inotify_add_watch(self.fd, c_path_name, mask)
		};

		match wd {
			-1 => Err(IoError::last_error()),
			_  => Ok(wd)
		}
	}

	pub fn rm_watch(&self, watch: Watch) -> IoResult<()> {
		let result = unsafe { ffi::inotify_rm_watch(self.fd, watch) };
		match result {
			0  => Ok(()),
			-1 => Err(IoError::last_error()),
			_  => fail!(
				"unexpected return code from inotify_rm_watch ({})", result)
		}
	}

	pub fn event(&self) -> IoResult<inotify_event> {
		let event = inotify_event {
			wd    : 0,
			mask  : 0,
			cookie: 0,
			len   : 0,
			name  : ptr::null()
		};

		let event_size = mem::size_of::<inotify_event>();

		let result = unsafe {
			ffi::read(
				self.fd,
				&event as *inotify_event as *c_void,
				event_size as u64)
		};

		match result {
			0  => Err(IoError {
				kind  : EndOfFile,
				desc  : "end of file",
				detail: None
			}),
			-1 => Err(IoError::last_error()),
			_  => Ok(event)
		}
	}

	pub fn close(&self) -> Result<(), ~str> {
		let result = ffi::close(self.fd as int);
		match result {
			0 => Ok(()),
			_ => Err(last_error())
		}
	}
}


fn last_error() -> ~str {
	unsafe {
		let c_error = libc::strerror(os::errno() as i32);
		CString::new(c_error, false)
			.as_str()
			.expect("failed to convert C error message into Rust string")
			.to_owned()
	}
}
