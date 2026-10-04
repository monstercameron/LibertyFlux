// original: 0x00b98280 GET_CONTROL_VALUE
/// Script native `GET_CONTROL_VALUE` (hash 0x06285788).
///
/// Forwards 2 script arguments (a control index and an input index) to the engine and
/// stores its full 32-bit answer (the control value) into the return slot.
export!(cdecl, rw_00b98280(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let answer = callee_cdecl!(1, u32, *args, *args.add(1));
        let slot = *(ctx as *const u32) as *mut u32;
        *slot = answer;
        answer
    }
});

