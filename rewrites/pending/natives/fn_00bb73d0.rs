// original: 0x00bb73d0 HAVE_ANIMS_LOADED
/// Script native `HAVE_ANIMS_LOADED` (hash 0x1D3F681D).
///
/// Forwards one script argument (an animation-set name) to the engine and
/// stores the low byte of its answer (zero-extended) into the return slot.
export!(cdecl, rw_00bb73d0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let answer = callee_cdecl!(1, u32, *args);
        let slot = *(ctx as *const u32) as *mut u32;
        *slot = answer & 0xFF;
        slot as u32
    }
});
