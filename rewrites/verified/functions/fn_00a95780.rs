// original: 0x00a95780 stream_touch_entry_maybe_open (proposed)

/// Mark a streaming entry touched, opening it when its mode selects open.
///
/// The entry is `table[idx*3*8]` where `table` is the pointer at `this+0x00`.
/// Bit 1 is set in the flag word at entry `+0x0e`; when the low two bits of
/// the byte at entry `+0x08` equal 1 the entry is first unlinked (callee 1,
/// thiscall/0 on the entry) and then opened (callee 2, thiscall/1 on the
/// entry with `this+0x18`).
///
/// Returns the open call's answer, or when the mode differs the table
/// pointer with its low byte replaced by the masked mode (as the original's
/// `(an instruction of the original)` leaves it). Thiscall: object in ecx, one stack word, callee
/// pops 4.
lf_checker_rt::export!(thiscall, rw_00a95780(this: u32, idx: u32) -> u32 {
    unsafe {
        const TABLE: u32 = 0x00;
        const OPEN_ARG: u32 = 0x18;
        const FLAGS_OFF: u32 = 0x0e;
        const MODE_OFF: u32 = 0x08;
        const TOUCH_BIT: u16 = 2;
        const MODE_MASK: u8 = 3;
        const OPEN_MODE: u8 = 1;
        const UNLINK_CALLEE: u32 = 1;
        const OPEN_CALLEE: u32 = 2;
        let base = ((this + TABLE) as *const u32).read_unaligned();
        let elem = base + idx.wrapping_mul(3).wrapping_mul(8);
        let flags = (elem + FLAGS_OFF) as *mut u16;
        flags.write_unaligned(flags.read_unaligned() | TOUCH_BIT);
        let mode = ((elem + MODE_OFF) as *const u8).read() & MODE_MASK;
        if mode != OPEN_MODE {
            return (base & 0xffffff00) | (mode as u32);
        }
        lf_checker_rt::callee_thiscall!(UNLINK_CALLEE, u32, elem);
        lf_checker_rt::callee_thiscall!(
            OPEN_CALLEE,
            u32,
            elem,
            ((this + OPEN_ARG) as *const u32).read_unaligned()
        )
    }
});
