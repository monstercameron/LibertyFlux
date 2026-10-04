// original: 0x00ba1cf0 SET_CHAR_WILL_REMAIN_ON_BOAT_AFTER_MISSION_ENDS
/// Forward (char, coerced flag) to the boat-persistence engine function.
///
/// Stack-slot boolean coercion: pushed dword is the context address with
/// its low byte replaced by (flag != 0).
rt::export!(cdecl, rw_00ba1cf0(ctx: u32) -> u32 {
    const ARG_ARRAY: usize = 2;
    let c = ctx as *const u32;
    let args = unsafe { *c.add(ARG_ARRAY) } as *const u32;
    let ch = unsafe { *args.add(0) };
    let flag = unsafe { *args.add(1) };
    let coerced = (ctx & 0xFFFF_FF00) | ((flag != 0) as u32);
    rt::callee_cdecl!(1, u32, ch, coerced)
});
