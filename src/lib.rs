extern crate winapi;
extern crate kernel32;
extern crate user32;
extern crate minhook;

#[macro_use]
extern crate log;
extern crate fern;
extern crate time;
#[macro_use]
extern crate lazy_static;

#[allow(non_upper_case_globals)]
mod eye;

#[allow(non_snake_case, unused_variables)]
#[no_mangle]
pub extern "stdcall" fn DllMain(module: u32, reason_for_call: u32, reserved: u32) -> bool { 
	match reason_for_call { 
		1 => {
			let logger_config = fern::DispatchConfig {
    			format: Box::new(|msg: &str, level: &log::LogLevel, _location: &log::LogLocation| {
        			format!("[{}][{}] {}", time::now().strftime("%Y-%m-%d][%H:%M:%S").unwrap(), level, msg)
    			}),
    			output: vec![fern::OutputConfig::stdout(), fern::OutputConfig::file("output.log")],
    			level: log::LogLevelFilter::Trace,
			};

			fern::init_global_logger(logger_config, log::LogLevelFilter::Trace);

			std::thread::spawn(move || {
				let eye = eye::Eye::new();
				eye.enable();

				loop {

				}
			});
		},
		_ => ()
	};
	true
}