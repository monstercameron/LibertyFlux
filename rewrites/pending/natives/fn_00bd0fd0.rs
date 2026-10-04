// original: 0x00bd0fd0 GET_WEAPONTYPE_SLOT
/// Forward (weapon, slot-out) to the weapon-slot engine function.
export!(cdecl, rw_00bd0fd0(ctx: u32) -> u32 {
    const ARG_ARRAY: usize = 2;
    let c = ctx as *const u32;
    let args = unsafe { *c.add(ARG_ARRAY) } as *const u32;
    let weapon = unsafe { *args.add(0) };
    let slot_out = unsafe { *args.add(1) };
    callee_cdecl!(1, u32, weapon, slot_out)
});
