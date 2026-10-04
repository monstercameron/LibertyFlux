// original: 0x00b93c90 ABSF
/// Script native `ABSF` (hash 0x067640F3).
///
/// Takes one float script argument, forwards its bits to the engine absolute-value routine, and stores the float answer (returned on the x87 stack) into the return slot.
export!(cdecl, rw_00b93c90(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let slot = *(ctx as *const u32) as *mut f32;
        let value: f32 = callee_cdecl!(1, f32, *args);
        *slot = value;
        slot as u32
    }
});
