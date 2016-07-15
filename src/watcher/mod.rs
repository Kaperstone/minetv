use std::collections::HashMap;
use std::mem::transmute;
use minhook::Hook;

mod proxy;

pub struct Watcher {
	pub hooks: HashMap<&'static str, Hook<extern "stdcall" fn()>>,
}

impl Watcher {
	pub fn new() -> Watcher {
		Watcher {
			hooks: proxy::create_hooks(),
		}
	}

	pub fn enable(&self) {
		proxy::enable_hooks(self);
	}

	pub fn on_game_start(&self) {
		let text = ::std::ffi::CString::new("game is starting!").unwrap().as_ptr();
		let caption = ::std::ffi::CString::new("watcher").unwrap().as_ptr();
		unsafe {
			::user32::MessageBoxA(::std::ptr::null_mut(), text, caption, 0);
			
			self.hooks.get(&"game_start").unwrap().trampoline()();
		}
	}

	pub fn on_game_over(&self, result: u32) -> u32 {
		let text = ::std::ffi::CString::new(format!("{}", result)).unwrap().as_ptr();
		let caption = ::std::ffi::CString::new("watcher").unwrap().as_ptr();
		unsafe {
			::user32::MessageBoxA(::std::ptr::null_mut(), text, caption, 0);

			let origin_fn: extern "stdcall" fn(u32) -> u32 = transmute(self.hooks.get(&"game_over").unwrap().trampoline());
			origin_fn(result)
		}
	}
}