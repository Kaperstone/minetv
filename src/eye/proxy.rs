use std::mem::transmute;
use std::collections::HashMap;

use minhook::Hook;
use eye::Eye;
use eye::game_api;

static mut EYE_PTR: *const Eye = 0 as *const Eye;

pub fn create_hooks() -> HashMap<&'static str, Hook<extern "stdcall" fn()>> {
	let mut hooks: HashMap<&'static str, Hook<extern "stdcall" fn()>> = HashMap::new();
	
	unsafe {
		hooks.insert("game_start", Hook::create(*game_api::game_start, on_game_start_proxy).unwrap());
		hooks.insert("game_over", transmute(Hook::create(*game_api::game_over, on_game_over_proxy).unwrap()));
		hooks.insert("box_up", transmute(Hook::create(*game_api::push_box_up, on_push_box_up_proxy).unwrap()));
		hooks.insert("box_down", transmute(Hook::create(*game_api::push_box_down, on_push_box_down_proxy).unwrap()));
		hooks.insert("display_cell", transmute(Hook::create(*game_api::display_cell, on_display_cell_proxy).unwrap()));
	}

	hooks
}

pub fn enable_hooks(eye: &Eye) {
	unsafe {
		EYE_PTR = eye as *const Eye;
	}
	
	for (_, hook) in eye.hooks.iter() {
		hook.enable();
	}
}

extern "stdcall" fn on_game_start_proxy() {
	unsafe {
		(*EYE_PTR).on_game_start();
	}
}

extern "stdcall" fn on_game_over_proxy(result: u32) -> u32 {
	unsafe {
		(*EYE_PTR).on_game_over(result)
	}
}

extern "stdcall" fn on_push_box_up_proxy(x: u32, y: u32) {
	unsafe {
		(*EYE_PTR).on_push_box_up(x, y);
	}
}

extern "stdcall" fn on_push_box_down_proxy(x: u32, y: u32) {
	unsafe {
		(*EYE_PTR).on_push_box_down(x, y);
	}
}

extern "stdcall" fn on_display_cell_proxy(x: u32, y: u32) {
	unsafe {
		(*EYE_PTR).on_display_cell(x, y);
	}
}