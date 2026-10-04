// original: 0x00621740 net_entry_remove_by_id_pair
/// Remove the table entry matching an id pair, if present.
///
/// Looks up the entry for `a0`/`a1` and, when found, removes it from the
/// table. Returns the lookup result (null when absent) or the removal
/// result otherwise.
export!(thiscall, rw_00621740(this: u32, a0: u32, a1: u32) -> u32 {
    unsafe {
        let found: u32 = callee_thiscall!(1, u32, this, a0, a1);
        if found != 0 {
            callee_thiscall!(2, u32, this, found)
        } else {
            0
        }
    }
});
