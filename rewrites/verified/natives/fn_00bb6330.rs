// original: 0x00bb6330 GET_FLOAT_STAT
/// Script native `GET_FLOAT_STAT` (hash 0x1D801FC0).
///
/// Forwards one script argument (a stat id) to the engine, which answers with
/// a 32-bit float, and stores the answer bits into the return slot.
/// Bit-exact: the float is never converted, only moved.
export!(cdecl, rw_00bb6330(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let answer: f32 = callee_cdecl!(1, f32, *args);
        let slot = *(ctx as *const u32) as *mut u32;
        *slot = answer.to_bits();
        slot as u32
    }
});
