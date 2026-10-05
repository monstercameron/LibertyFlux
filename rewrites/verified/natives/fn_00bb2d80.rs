// original: 0x00bb2d80 SET_WANTED_MULTIPLIER
/// Script native `SET_WANTED_MULTIPLIER` (hash 0x51E14C1B).
///
/// Forwards one float script argument (the wanted multiplier) to the
/// engine as raw bits, so the forward is bit-exact. No return slot is
/// written.
/// (The original cleans its one pushed argument with `(an instruction of the original)`; the effect
/// on the stack pointer is identical to the plain cdecl return here.)
export!(cdecl, rw_00bb2d80(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args)
    }
});
