// original: 0x00b93e80 ASIN
/// Script native `ASIN` (hash 0x590A6F04).
///
/// Forwards one float bit-pattern (the angle operand) to the engine and
/// stores the engine's floating-point result into the return slot. The
/// engine answers on the x87 stack; the handler converts that to a float
/// store, which this rewrite expresses as an `f32` channel copy.
export!(cdecl, rw_00b93e80(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let answer: f32 = callee_cdecl!(1, f32, *args);
        let slot = *(ctx as *const u32) as *mut f32;
        *slot = answer;
        slot as u32
    }
});
