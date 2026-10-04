// original: 0x00bc6eb0 IS_VEH_WINDOW_INTACT
/// Script native `IS_VEH_WINDOW_INTACT` (hash 0x1D0B131A).
///
/// Forwards two script arguments (a vehicle handle and a window index) to
/// the engine and stores the low byte of its answer (zero-extended) into
/// the return slot.
export!(cdecl, rw_00bc6eb0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let answer = callee_cdecl!(1, u32, *args, *args.add(1));
        let slot = *(ctx as *const u32) as *mut u32;
        *slot = answer & 0xFF;
        slot as u32
    }
});
