// original: 0x00a017a0 MAKE_OBJECT_TARGETTABLE
/// Script native `MAKE_OBJECT_TARGETTABLE` (hash 0x228F1801).
///
/// Forwards an object handle and a stack-slot-coerced boolean flag to the
/// engine. No return slot is written.
export!(cdecl, rw_00a017a0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let flag = u32::from(*args.add(1) != 0);
        let quirked = (ctx as u32 & 0xFFFF_FF00) | flag;
        callee_cdecl!(1, u32, *args, quirked)
    }
});
