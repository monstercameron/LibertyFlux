// original: 0x00a7e860 task_lookup_flag_append
/// Look up a task entry and, when found, flag and queue it on this list.
///
/// Resolves an entry from the first and fourth arguments; a null result
/// is a no-op. Otherwise the entry's flag word (`+8`) keeps its upper
/// bits while the low nibble becomes the masked bits of the other two
/// arguments, and the entry is appended to this list. Returns nothing.
lf_checker_rt::export!(thiscall, rw_00a7e860(this: u32, key0: u32, flag_a: u32, flag_b: u32, key1: u32) -> u32 {
    let found = lf_checker_rt::callee_stdcall!(1, u32, key0, key1);
    if found == 0 {
        return 0;
    }
    unsafe {
        let slot = (found as *mut u32).add(2);
        let merged = slot.read_unaligned() & 0xfffffff0
            | ((flag_b & 7) << 1) | (flag_a & 1);
        slot.write_unaligned(merged);
    }
    lf_checker_rt::callee_thiscall!(2, u32, this, found);
    0
});
