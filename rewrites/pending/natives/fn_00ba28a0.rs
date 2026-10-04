// original: 0x00BA28A0 SET_SPECIFIC_PASSENGER_INDEX_TO_USE_IN_GROUPS
// SET_SPECIFIC_PASSENGER_INDEX_TO_USE_IN_GROUPS: forward (group, index).
export!(cdecl, rw_00BA28A0(ctx: u32) -> u32 {
    unsafe {
        let a = args_of(ctx);
        callee_cdecl!(1, u32, *a, *a.add(1))
    }
});
