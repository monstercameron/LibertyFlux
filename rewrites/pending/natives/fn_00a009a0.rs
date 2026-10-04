// original: 0x00a009a0 DOES_OBJECT_HAVE_PHYSICS
/// Script native `DOES_OBJECT_HAVE_PHYSICS` (hash 0x39587D51).
///
/// Forwards arg0 to the engine.
/// Writes the zero-extended low byte of the engine answer into the return slot.
export!(cdecl, rw_00a009a0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let ans = callee_cdecl!(1, u32, *args.add(0));
        let slot = (*(ctx as *const u32)) as *mut u32;
        *slot = ans & 0xFF;
        slot as u32
    }
});
