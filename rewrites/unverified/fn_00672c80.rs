// original: 0x00672C80 get_file_slot_progress
//! Read the first word of a non-null indexed file-slot object into the output
//! pointer. The table contains four-byte object pointers at `SLOT_OBJECTS_VA`.
//! A null table entry leaves the output untouched and returns zero; otherwise
//! the function returns one in AL while retaining the output pointer's upper
//! return bits. The stdcall callee removes both stack arguments.
const SLOT_OBJECTS_VA: u32 = 0x019F13F0;

#[no_mangle]
pub unsafe extern "stdcall" fn fn_00672C80(slot_index: u32, output: *mut u32) -> u32 {
    let slots = lf_checker_rt::global::<u32>(SLOT_OBJECTS_VA);
    let object = unsafe { core::ptr::read_volatile(slots.add(slot_index as usize)) };
    if object == 0 {
        0
    } else {
        let progress = unsafe { core::ptr::read_volatile(object as *const u32) };
        unsafe { core::ptr::write_volatile(output, progress) };
        ((output as u32) & !0xff) | 1
    }
}
