// original: 0x00c90150 fn_00c90150_supported_task_category

#[inline(always)]
unsafe fn read_u32(address: u32) -> u32 {
    unsafe { (address as *const u32).read_unaligned() }
}

#[inline(always)]
unsafe fn write_u32(address: u32, value: u32) {
    unsafe { (address as *mut u32).write_unaligned(value) }
}

#[inline(always)]
unsafe fn read_global_f32(file_va: u32) -> f32 {
    let bits = unsafe { read_u32(lf_checker_rt::relocated(file_va)) };
    f32::from_bits(bits)
}

/// Return true for one of the three supported low-bit task categories when
/// its state word is zero, after invoking the category setup helper. The
/// function is cdecl, takes one object pointer, and returns a boolean in AL.
const TASK_CATEGORY_MASK: u32 = 7;
const TASK_STATE_OFFSET: u32 = 0x10;
const TASK_SETUP_CALLEE: u32 = 1;

unsafe fn supported_task_category(object: u32, wrong_state_test: bool) -> u8 {
    let category = unsafe { read_u32(object) } & TASK_CATEGORY_MASK;
    if !(1..=3).contains(&category) {
        return 0;
    }
    let state = unsafe { read_u32(object + TASK_STATE_OFFSET) };
    if if wrong_state_test { state == 0 } else { state != 0 } {
        return 0;
    }
    let _ = lf_checker_rt::callee_cdecl!(TASK_SETUP_CALLEE, u32, object);
    1
}

lf_checker_rt::export!(cdecl, rw_00c90150_supported_task_category(object: u32) -> u8 {
    unsafe { supported_task_category(object, false) }
});
