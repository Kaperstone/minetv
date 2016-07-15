use std::mem::transmute;
use std::collections::HashMap;

use minhook::Hook;
use watcher::Watcher;

static mut WATCHER_PTR: *const Watcher = 0 as *const Watcher;

pub fn create_hooks() -> HashMap<&'static str, Hook<extern "stdcall" fn()>> {
	let mut hooks: HashMap<&'static str, Hook<extern "stdcall" fn()>> = HashMap::new();
	
	unsafe {
		let game_start: extern "stdcall" fn() = transmute(0x0100367A);
		let game_over: extern "stdcall" fn(u32) -> u32 = transmute(0x0100347C);

		hooks.insert("game_start", Hook::create(game_start, on_game_start_proxy).unwrap());
		hooks.insert("game_over", transmute(Hook::create(game_over, on_game_over_proxy).unwrap()));
	}

	hooks
}

pub fn enable_hooks(watcher: &Watcher) {
	unsafe {
		WATCHER_PTR= watcher as *const Watcher;
	}
	
	for (_, hook) in watcher.hooks.iter() {
		hook.enable();
	}
}

extern "stdcall" fn on_game_start_proxy() {
	unsafe {
		(*WATCHER_PTR).on_game_start();
	}
}

extern "stdcall" fn on_game_over_proxy(result: u32) -> u32 {
	unsafe {
		(*WATCHER_PTR).on_game_over(result)
	}
}