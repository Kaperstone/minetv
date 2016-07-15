extern crate winapi;
extern crate kernel32;
extern crate user32;
extern crate minhook;

mod watcher;

#[allow(non_snake_case, unused_variables)]
#[no_mangle]
pub extern "stdcall" fn DllMain(module: u32, reason_for_call: u32, reserved: u32) -> bool { 
	match reason_for_call { 
		1 => {
			std::thread::spawn(move || {
				let watcher = watcher::Watcher::new();
				watcher.enable();

				loop {

				}
			});
		},
		_ => ()
	};
	true
}