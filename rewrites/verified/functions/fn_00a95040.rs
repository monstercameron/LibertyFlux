// original: 0x00a95040 stream_fill_entry_value (proposed)

/// Look an entry up and store the caller's value into it.
///
/// Six arguments are forwarded to the lookup (callee 1, stdcall/6): all but
/// the fourth. A lookup answer of 0xffff means absent and is returned; else
/// the entry `table[answer*3*8]` (table at `this+0x00`) receives the fourth
/// argument in its value word, its flag word is ORed with 0x1000 unless the
/// third and fourth arguments are equal, and ANDed with 0xdfff.
///
/// Returns the lookup answer. Thiscall: object in ecx, six stack words,
/// callee pops 24.
lf_checker_rt::export!(thiscall, rw_00a95040(
    this: u32,
    a0: u32,
    a1: u32,
    a2: u32,
    a3: u32,
    a4: u32,
    a5: u32,
) -> u32 {
    unsafe {
        const TABLE: u32 = 0x00;
        const FLAGS_OFF: u32 = 0x0e;
        const ABSENT: u32 = 0xffff;
        const OR_MASK: u16 = 0x1000;
        const AND_MASK: u16 = 0xdfff;
        const LOOKUP_CALLEE: u32 = 1;
        let found = lf_checker_rt::callee_stdcall!(LOOKUP_CALLEE, u32, a0, a1, a2, a4, a5, 0);
        if found == ABSENT {
            return ABSENT;
        }
        let elem = ((this + TABLE) as *const u32).read_unaligned()
            + found.wrapping_mul(3).wrapping_mul(8);
        ((elem) as *mut u32).write_unaligned(a3);
        if a2 != a3 {
            let flags = (elem + FLAGS_OFF) as *mut u16;
            flags.write_unaligned(flags.read_unaligned() | OR_MASK);
        }
        let flags = (elem + FLAGS_OFF) as *mut u16;
        flags.write_unaligned(flags.read_unaligned() & AND_MASK);
        found
    }
});
