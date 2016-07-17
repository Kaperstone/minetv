use std::collections::HashMap;
use std::mem::transmute;
use minhook::Hook;

mod proxy;
mod broadcast;
mod watcher;
mod game_api;

pub struct Eye {
	pub hooks: HashMap<&'static str, Hook<extern "stdcall" fn()>>,
	broadcast: Option<broadcast::Broadcast>,
	watcher: Option<watcher::Watcher>,
}

impl Eye {
	pub fn new() -> Eye {
		Eye {
			hooks: proxy::create_hooks(),
			broadcast: None,
			watcher: None,
		}
	}

	pub fn enable(&self) {
		proxy::enable_hooks(self);
	}

	pub fn on_game_start(&self) {
		trace!("game is starting");

		if let Some(ref broadcast) = self.broadcast {
			//
		}

		unsafe {
			self.hooks.get(&"game_start").unwrap().trampoline()();
		}
	}

	pub fn on_game_over(&self, result: u32) -> u32 {
		trace!("game is over with result {}", result);

		unsafe {
			let origin_fn: extern "stdcall" fn(u32) -> u32 = transmute(self.hooks.get(&"game_over").unwrap().trampoline());
			origin_fn(result);

			return 1;
		}
	}

	pub fn on_display_cell(&self, x: u32, y: u32) {
		trace!("display cell: {} {}", x, y);

		if let Some(ref broadcast) = self.broadcast {
			//
			let cell_id = x + y * 32;
			let cell = (0x01005340 + cell_id) as *mut u8;
		}

		unsafe {
			let origin_fn: extern "stdcall" fn(u32, u32) = transmute(self.hooks.get(&"display_cell").unwrap().trampoline());
			origin_fn(x, y);
		}
	}

	pub fn on_push_box_up(&self, x: u32, y: u32) {
		unsafe {
			let origin_fn: extern "stdcall" fn(u32, u32) = transmute(self.hooks.get(&"box_up").unwrap().trampoline());
			origin_fn(x, y);
		}
	}

	pub fn on_push_box_down(&self, x: u32, y: u32) {
		unsafe {
			let origin_fn: extern "stdcall" fn(u32, u32) = transmute(self.hooks.get(&"box_down").unwrap().trampoline());
			origin_fn(x, y);
		}
	}
}