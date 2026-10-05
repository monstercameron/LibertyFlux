// original: 0x00a01a30 SET_COLLECTABLE1_TOTAL
/// Script native `SET_COLLECTABLE1_TOTAL` (hash 0x79574B3B).
///
/// Forwards one script argument (the new total) to the engine. No return
/// slot is written. (The original cleans its one pushed argument with
/// `(an instruction of the original)`; the effect on the stack pointer is identical to the plain
/// cdecl return here.)
export!(cdecl, rw_00a01a30(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args)
    }
});
