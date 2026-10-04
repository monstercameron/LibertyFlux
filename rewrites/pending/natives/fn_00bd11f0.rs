// original: 0x00bd11f0 SET_CURRENT_CHAR_WEAPON
/// Forward (char, weapon, coerced flag) to the weapon engine function.
///
/// Stack-slot boolean coercion on the third argument.
rt::export!(cdecl, rw_00bd11f0(ctx: u32) -> u32 {
    const ARG_ARRAY: usize = 2;
    let c = ctx as *const u32;
    let args = unsafe { *c.add(ARG_ARRAY) } as *const u32;
    let ch = unsafe { *args.add(0) };
    let weapon = unsafe { *args.add(1) };
    let flag = unsafe { *args.add(2) };
    let coerced = (ctx & 0xFFFF_FF00) | ((flag != 0) as u32);
    rt::callee_cdecl!(1, u32, ch, weapon, coerced)
});
