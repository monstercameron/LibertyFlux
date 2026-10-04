// original: 0x00b9fcf0 IS_CHAR_RESPONDING_TO_ANY_EVENT
/// Native handler `IS_CHAR_RESPONDING_TO_ANY_EVENT`: forwards one script handle to the engine and stores the low byte of the answer.
///
/// `ctx` points at the script call context: dword 0 is the return slot,
/// dword 2 is the script argument array. The engine call is intercepted
/// by the checker, which compares its arguments.
#[no_mangle]
pub extern "cdecl" fn rn24_is_char_responding_to_any_event(ctx: u32) {
        let args = unsafe { (*(ctx as *const u32).add(2)) as *const u32 };
        let answer: u32 = lf_rn24_rt::callee_cdecl!(1, u32, unsafe { args.read() });
        let slot = unsafe { *(ctx as *const u32) as *mut u32 };
        // Only the low byte is kept (movzx after the call).
        unsafe { slot.write(answer & 0xFF) };
}
