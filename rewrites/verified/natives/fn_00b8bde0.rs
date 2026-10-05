// original: 0x00b8bde0 CHANGE_PICKUP_BLIP_COLOUR
/// Script native `CHANGE_PICKUP_BLIP_COLOUR` (hash 0x65D949B7).
///
/// Forwards one script argument to the engine worker. (The original drops
/// its pushed word with `(an instruction of the original)`; the effect on the stack pointer is
/// identical to the plain cdecl return here.) No return slot is written.
export!(cdecl, rw_00b8bde0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args)
    }
});
