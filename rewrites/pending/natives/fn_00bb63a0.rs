// original: 0x00bb63a0 GET_PROGRESS_PERCENTAGE
/// Native handler `GET_PROGRESS_PERCENTAGE`: takes no arguments and stores the float the engine returns on the x87 stack.
///
/// `ctx` points at the script call context: dword 0 is the return slot,
/// dword 2 is the script argument array. The engine call is intercepted
/// by the checker, which compares its arguments.
#[no_mangle]
pub extern "cdecl" fn rn24_get_progress_percentage(ctx: u32) {
        let value: f32 = lf_rn24_rt::callee_cdecl!(1, f32,);
        let slot = unsafe { *(ctx as *const u32) as *mut f32 };
        unsafe { slot.write(value) };
}
