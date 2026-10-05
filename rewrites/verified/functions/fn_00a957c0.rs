// original: 0x00a957c0 stream_reopen_entry_by_flag (proposed)

/// Re-run a streaming entry's open step with the argument its flag selects.
///
/// The entry is `table[idx*3*8]` where `table` is the pointer at `this+0x00`.
/// The entry is opened (callee 1, thiscall/1 on the entry) with `this+0x18`
/// when the flag byte at entry `+0x0e` has any of the bits 0xc6 set, else
/// with `this+0x08`; then its state step runs (callee 2, thiscall/1 on the
/// entry with 1).
///
/// Returns the state step's answer. Thiscall: object in ecx, one stack
/// word, callee pops 4.
lf_checker_rt::export!(thiscall, rw_00a957c0(this: u32, idx: u32) -> u32 {
    unsafe {
        const TABLE: u32 = 0x00;
        const ALT_ARG: u32 = 0x08;
        const MAIN_ARG: u32 = 0x18;
        const FLAG_OFF: u32 = 0x0e;
        const FLAG_TEST: u8 = 0xc6;
        const STATE_ARG: u32 = 1;
        const OPEN_CALLEE: u32 = 1;
        const STATE_CALLEE: u32 = 2;
        let elem = ((this + TABLE) as *const u32).read_unaligned()
            + idx.wrapping_mul(3).wrapping_mul(8);
        let src = if ((elem + FLAG_OFF) as *const u8).read() & FLAG_TEST != 0 {
            MAIN_ARG
        } else {
            ALT_ARG
        };
        lf_checker_rt::callee_thiscall!(
            OPEN_CALLEE,
            u32,
            elem,
            ((this + src) as *const u32).read_unaligned()
        );
        lf_checker_rt::callee_thiscall!(STATE_CALLEE, u32, elem, STATE_ARG)
    }
});
