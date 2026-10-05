// original: 0x00a95230 stream_fill_entry_indirect (proposed)

/// Look an entry up and copy the caller's pointed-to value into it.
///
/// All arguments but the fourth go to the lookup (callee 1, stdcall/6);
/// the fourth is the pointer the found value is copied from. A lookup
/// answer of 0xffff means absent
/// and is returned; else the entry `table[answer*3*8]` (table at `this+0x00`)
/// receives the dword the fourth argument points to, and its flag word is
/// mapped through `(w & 0xefff) | 0x2000`.
///
/// Returns the lookup answer. Thiscall: object in ecx, seven stack words,
/// callee pops 28.
lf_checker_rt::export!(thiscall, rw_00a95230(
    this: u32,
    a0: u32,
    a1: u32,
    a2: u32,
    a3: u32,
    a4: u32,
    a5: u32,
    a6: u32,
) -> u32 {
    unsafe {
        const TABLE: u32 = 0x00;
        const FLAGS_OFF: u32 = 0x0e;
        const ABSENT: u32 = 0xffff;
        const AND_MASK: u16 = 0xefff;
        const OR_MASK: u16 = 0x2000;
        const LOOKUP_CALLEE: u32 = 1;
        let _ = a0;
        let found = lf_checker_rt::callee_stdcall!(LOOKUP_CALLEE, u32, a0, a1, a2, a4, a5, a6);
        if found == ABSENT {
            return ABSENT;
        }
        let elem = ((this + TABLE) as *const u32).read_unaligned()
            + found.wrapping_mul(3).wrapping_mul(8);
        (elem as *mut u32).write_unaligned((a3 as *const u32).read_unaligned());
        let flags = (elem + FLAGS_OFF) as *mut u16;
        flags.write_unaligned((flags.read_unaligned() & AND_MASK) | OR_MASK);
        found
    }
});
