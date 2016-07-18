use std::ptr::*;
use std::mem::transmute;

lazy_static! {
	pub static ref game_start: extern "stdcall" fn() = unsafe {
		transmute(0x0100367A)
	};
	pub static ref game_over: extern "stdcall" fn(u32) -> u32 = unsafe {
		transmute(0x0100347C)
	};
	pub static ref push_box_up: extern "stdcall" fn(u32, u32) = unsafe {
		transmute(0x010031A0)
	};
	pub static ref push_box_down: extern "stdcall" fn(u32, u32) = unsafe {
		transmute(0x0100316B)
	};
	pub static ref display_cell: extern "stdcall" fn(u32, u32) = unsafe {
		transmute(0x01002646)
	};
	pub static ref window_proc: extern "stdcall" fn(u32, u32, u32, u32) -> u32 = unsafe {
		transmute(0x01001BC9)
	};
}

pub fn get_field(without_bombs: bool) -> Vec<u8> {
	let p_bombs_array = 0x01005340 as *mut u8;
	let bombs = unsafe {
		Vec::from_raw_parts(p_bombs_array, 864, 1)
	};

	if without_bombs {
		bombs.into_iter().map(|cell| if cell == 0x8F { 0x0F } else { cell }).collect()
	} else {
		bombs
	}
}

pub fn get_field_size() -> (u32, u32) {
	unsafe {
		(read(0x010056A8 as _), read(0x010056AC as _))
	}
}

pub fn get_max_bomb_count() -> u32 {
	unsafe {
		read(0x010056A4 as _)
	}
}

pub fn get_preferences() -> u32 {
	unsafe {
		read(0x010056A0 as _)
	}
}

pub fn set_field_size(height: u32, width: u32) {
	unsafe {
		write(0x010056A8 as _, height);
		write(0x010056AC as _, width);
	}
}

pub fn set_max_bomb_count(count: u32) {
	unsafe {
		write(0x010056A4 as _, count);
	}
}

pub fn get_cell(x: u32, y: u32) -> u8 {
	unsafe {
		read((0x01005340 + x + y * 32) as _)
	}
}

pub fn set_cell(x: u32, y: u32, value: u8) {
	unsafe {
		write((0x01005340 + x + y * 32) as _, value)
	}
}