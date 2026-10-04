// original: 0x0086e930 TIMESTEPUNWARPED
/// Script native `TIMESTEPUNWARPED` (hash 0x49283645).
///
/// Takes no script arguments and makes no engine call: writes the constant 0x3C888889 (the fixed frame step, 1/60 second as a float) into the return slot.
export!(cdecl, rw_0086e930(ctx: *const u8) -> u32 {
    unsafe {
        let slot = *(ctx as *const u32) as *mut u32;
        // Fixed frame step: 1/60 s as an f32 bit pattern.
        *slot = 0x3C88_8889;
        slot as u32
    }
});
