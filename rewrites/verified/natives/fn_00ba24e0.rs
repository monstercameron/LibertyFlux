// original: 0x00ba24e0 SET_PED_INSTANT_BLENDS_WEAPON_ANIMS
/// Script native `SET_PED_INSTANT_BLENDS_WEAPON_ANIMS` (hash 0x2CB572B5).
///
/// placeholder-never-matches
export!(cdecl, rw_00ba24e0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let flag = u32::from(*args.add(1) != 0);
        let quirked = flag;
        callee_cdecl!(1, u32, *args, quirked)
    }
});
