extern crate winapi;
extern crate kernel32;
extern crate user32;
extern crate libc;
extern crate minhook;

mod mine;

use std::mem::size_of;
use minhook::Hook;

fn main() {
	if let Some(game) = mine::MineGame::new("Winmine__XP.exe") {
		game.inject_dll("./target/debug/watcher.dll");
		game.resume();

		loop {

		}
	} else {
		println!("canot run winemine!");
	}
}