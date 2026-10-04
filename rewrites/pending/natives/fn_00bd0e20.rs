// original: 0x00bd0e20 GET_CHAR_WEAPON_IN_SLOT
// GET_CHAR_WEAPON_IN_SLOT: call the engine with (arg0..arg4). Results are
// written through out-pointers in the args; no return slot used.
export!(cdecl, rw_00bd0e20(ctx: u32) -> u32 {
    unsafe {
        let a = args_of(ctx);
        callee_cdecl!(1, u32, *a, *a.add(1), *a.add(2), *a.add(3), *a.add(4))
    }
});
