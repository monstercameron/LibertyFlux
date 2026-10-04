// original: 0x00bb62a0 CONVERT_METRES_TO_FEET
/// Script native `CONVERT_METRES_TO_FEET` (hash 0x4D2771CE).
///
/// Forwards one script argument (a distance in metres, as raw float bits) to the engine, which returns
/// a float in the x87 top-of-stack register; the handler stores it into
/// the return slot (the distance in feet). Bits are forwarded untouched, so the
/// conversion is bit-exact by construction.
export!(cdecl, rw_00bb62a0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let answer: f32 = callee_cdecl!(1, f32, *args);
        let slot = *(ctx as *const u32) as *mut f32;
        *slot = answer;
        slot as u32
    }
});

