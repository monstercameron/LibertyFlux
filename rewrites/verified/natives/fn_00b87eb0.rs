// original: 0x00b87eb0 UNPOINT_CAM
/// Script native `UNPOINT_CAM` (hash 0x212B4014).
///
/// Forwards one script argument (a camera handle) to the engine. No return slot is written. (The original cleans its one pushed argument with `(an instruction of the original)`; the effect on the stack pointer is identical to the plain cdecl return here.)
export!(cdecl, rw_00b87eb0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args)
    }
});
