// original: 0x00b944c0 GET_FRAME_TIME
/// Script native `GET_FRAME_TIME` (hash 0x206420A6).
///
/// Forwards one script argument (an out-pointer for the frame time) to the
/// engine. No return slot is written by the handler itself.
/// (The original cleans its one pushed argument with `(an instruction of the original)`; the effect
/// on the stack pointer is identical to the plain cdecl return here.)
export!(cdecl, rw_00b944c0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args)
    }
});
