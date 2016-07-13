use std::mem;

use minhook::Hook;
use watcher::Watcher;

static mut GAMESTART_ORIGIN: Option<unsafe fn()> = None;
static mut WATCHER_PTR: Option<*const Watcher> = None;

pub fn init_hooks(watcher: &Watcher) -> Hook<fn()> {
	unsafe {
		WATCHER_PTR = Some(watcher);
		let game_start: fn() = mem::transmute(0x0100367A);
		let hook = Hook::create(game_start, on_game_start_proxy).unwrap();

		GAMESTART_ORIGIN = Some(hook.trampoline());
		
		hook
	}
}

pub fn enable_hooks() {
	
}

fn on_game_start_proxy() {
	unsafe {
		if GAMESTART_ORIGIN.is_some() && WATCHER_PTR.is_some() {
			let watcher_ptr = WATCHER_PTR.take();
			
			(*watcher_ptr.unwrap()).on_game_start();
			WATCHER_PTR = watcher_ptr;
			
			GAMESTART_ORIGIN.unwrap()();
		}
	}
}