extern crate winapi;
extern crate kernel32;
extern crate user32;
extern crate libc;
extern crate minhook;

mod mine;

fn main() {
	if let Some(game) = mine::MineGame::new("Winmine__XP.exe") {
		game.inject_dll("./target/debug/watcher.dll");
		game.resume();

		loop {

		}
	} else {
		println!("cannot run winmine!");
	}
}