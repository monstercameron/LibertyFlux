// original: 0x00672FF0 get_last_file_info
//! Copy the two shared last-file-info words into the second stack argument.
//! The final word is retained in EAX with its low byte replaced by one, and
//! the stdcall callee removes both stack arguments.
const LAST_FILE_INFO_FIRST_VA: u32 = 0x01BB3898;
const LAST_FILE_INFO_SECOND_VA: u32 = 0x01BB389C;
const RESULT_FLAG: u32 = 1;

#[no_mangle]
pub unsafe extern "stdcall" fn fn_00672FF0(_unused: u32, output: *mut u32) -> u32 {
    let first = unsafe {
        core::ptr::read_volatile(lf_checker_rt::global::<u32>(LAST_FILE_INFO_FIRST_VA))
    };
    unsafe { core::ptr::write_volatile(output, first) };

    let second = unsafe {
        core::ptr::read_volatile(lf_checker_rt::global::<u32>(LAST_FILE_INFO_SECOND_VA))
    };
    unsafe { core::ptr::write_volatile(output.add(1), second) };
    (second & !0xff) | RESULT_FLAG
}
