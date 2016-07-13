use kernel32::*;
use winapi::processthreadsapi::*;
use winapi::winbase::*;
use winapi::winnt::*;
use std::ptr::null_mut;
use std::ffi::CString;
use std::mem::transmute;
use std::os::raw::c_void;

pub struct MineGame {
	process_info: PROCESS_INFORMATION,
}

impl MineGame {
	pub fn new(game_path: &str) -> Option<MineGame> {
		unsafe {
			let mut process_info = PROCESS_INFORMATION {
				hProcess: 0 as HANDLE,
				hThread: 0 as HANDLE,
				dwProcessId: 0,
				dwThreadId: 0,
			};

			let mut startup_info = STARTUPINFOA {
				cb: 0,
    			lpReserved: null_mut(),
     			lpDesktop: null_mut(),
    			lpTitle: null_mut(),
    			dwX: 0,
    			dwY: 0,
    			dwXSize: 0,
    			dwYSize: 0,
    			dwXCountChars: 0,
    			dwYCountChars: 0,
    			dwFillAttribute: 0,
    			dwFlags: 0,
    			wShowWindow: 0,
    			cbReserved2: 0,
    			lpReserved2: null_mut(),
    			hStdInput: 0 as HANDLE,
    			hStdOutput: 0 as HANDLE,
    			hStdError: 0 as HANDLE,
			};

			let name = CString::new(game_path);

        	let result = CreateProcessA(name.unwrap().as_ptr(), null_mut(), null_mut(), null_mut(), 0, DETACHED_PROCESS | CREATE_SUSPENDED, null_mut(), null_mut(), &mut startup_info, &mut process_info);

			if result == 1 {
				if process_info.dwProcessId == 0 {
					TerminateProcess(process_info.hProcess, 0);
					None
				} else {
					Some(MineGame {
						process_info: process_info,
					})
				}
			} else {
				None
			}
		}
	}

	pub fn inject_dll(&self, dll_name: &str) -> bool {
		unsafe {
			let handle = OpenProcess(PROCESS_ALL_ACCESS, 0, self.process_info.dwProcessId);

			if handle as u32 == 0 {
				false
			} else {
				let lib_name = CString::new("kernel32.dll");
				let func_name = CString::new("LoadLibraryA");

				let address = GetProcAddress(GetModuleHandleA(lib_name.unwrap().as_ptr()), func_name.unwrap().as_ptr());
				let allocated = VirtualAllocEx(handle, null_mut(), dll_name.len() as u32, MEM_COMMIT | MEM_RESERVE, PAGE_READWRITE);
			
				WriteProcessMemory(handle, allocated, dll_name.as_ptr() as *const _, dll_name.len() as u32, null_mut());
				
				let f_address: unsafe extern "system" fn(*mut c_void) -> u32 = transmute(address);

				let remote_thread = CreateRemoteThread(handle, null_mut(), 0, Some(f_address), allocated, 0, null_mut());
			
				WaitForSingleObject(remote_thread, INFINITE);
			
				VirtualFreeEx(handle, allocated, dll_name.len() as u32, MEM_RELEASE);
			
				CloseHandle(remote_thread);
				CloseHandle(handle);

				true
			}
		}
	}

	pub fn resume(&self) {
		unsafe {
			ResumeThread(self.process_info.hThread);
		}
	}
}