// original: 0x00a9eac0 stream_reset_all (proposed)

/// Reset every entry of the 256-entry table and clear the header cursors.
///
/// `this` points to the table object. Each of the 256 entries of 0x1c bytes
/// starting at `+0x0` is passed (in ECX) to the entry reset routine, then
/// the two header lists at `+0x1c00` and `+0x1c0c` are reset the same way.
/// The words at `+0x1c1c`, `+0x1c18` and `+0x74f4` are zeroed and `this`
/// itself is returned.
///
/// Original: 0x00a9eac0 (thiscall, no stack arguments; 258 outgoing calls,
/// so the contract raises the call-log cap — see `narrowed`).
lf_checker_rt::export!(thiscall, rw_00a9eac0(this: u32) -> u32 {
    unsafe {
        const ENTRY_STRIDE: u32 = 0x1c;
        const ENTRY_COUNT: u32 = 0x100;
        const LIST_A_OFF: u32 = 0x1c00;
        const LIST_B_OFF: u32 = 0x1c0c;
        const ENTRY_RESET: u32 = 1;
        const LIST_RESET: u32 = 2;
        let mut i = 0u32;
        while i < ENTRY_COUNT {
            lf_checker_rt::callee_thiscall!(
                ENTRY_RESET,
                u32,
                this.wrapping_add(i.wrapping_mul(ENTRY_STRIDE))
            );
            i += 1;
        }
        lf_checker_rt::callee_thiscall!(LIST_RESET, u32, this.wrapping_add(LIST_A_OFF));
        lf_checker_rt::callee_thiscall!(LIST_RESET, u32, this.wrapping_add(LIST_B_OFF));
        ((this + 0x1c1c) as *mut u32).write_unaligned(0);
        ((this + 0x1c18) as *mut u32).write_unaligned(0);
        ((this + 0x74f4) as *mut u32).write_unaligned(0);
        this
    }
});
