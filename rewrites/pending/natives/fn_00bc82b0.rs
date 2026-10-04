// original: 0x00bc82b0 SYNCH_AMBIENT_PLANES
/// Synchronise ambient planes: pass the handle plus the second script
/// word (a float, bitwise) to the engine. No return value.
export!(cdecl, rw_00bc82b0(ctx: *const u32) -> u32 {
    unsafe {
        let args = *ctx.add(2) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1));
        0
    }
});
