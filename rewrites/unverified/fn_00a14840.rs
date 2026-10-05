// original: 0x00a14840 id_slot_assign (proposed)
/// Assign the next identity number to a freshly resolved entry.
///
/// Resolves `key` to an entry. When the entry exists and its identity word
/// at `+0x3c4` is still -1, it takes the current value of the global counter
/// and the counter advances by one. No return value is set. Cdecl.
export!(cdecl, rw_00a14840(key: u32) -> u32 {
    unsafe {
        const RESOLVE: u32 = 1;
        const LOOKUP: u32 = 2;
        const ID_OFF: u32 = 0x3c4;
        const COUNTER: u32 = 0x012bd1b0;
        let tmp = callee_cdecl!(RESOLVE, u32, key);
        let ent = callee_cdecl!(LOOKUP, u32, tmp, 0);
        if ent == 0 {
            return 0;
        }
        if ((ent + ID_OFF) as *const u16).read_unaligned() as i16 != -1 {
            return 0;
        }
        let n = *global::<u16>(COUNTER);
        ((ent + ID_OFF) as *mut u16).write_unaligned(n);
        let c = global::<u32>(COUNTER);
        c.write(c.read().wrapping_add(1));
        0
    }
});
