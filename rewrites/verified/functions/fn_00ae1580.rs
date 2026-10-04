// original: 0x00ae1580 ui_time_pair_store
/// Store a time value and its scaled float into the global time record.
///
/// Takes the counter difference from the first source, converts it through the
/// float source (whose stack argument it ignores: the callee reads only
/// globals), and stores both the raw value and the float into the record
/// selected by a global. The float source is `cdecl/1` and the record sink is
/// `thiscall/2` (verified against their `ret` cleanups, which is what balances
/// this function's stack). Returns the sink's answer.
export!(cdecl, rw_00ae1580() -> u32 {
    unsafe {
        let a = callee_cdecl!(1, u32,);
        let fbits = callee_cdecl!(2, u32, a);
        let rec = *global::<u32>(0x1593B6C);
        callee_thiscall!(3, u32, rec, fbits, a)
    }
});
