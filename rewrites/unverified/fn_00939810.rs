// original: 0x00939810 stream_wrap_a (proposed)

/// Notify the owner when this record's flag is set.
///
/// `this` points to the record. When the flag byte at `+0x10` is zero,
/// answers 0 without calling. Otherwise calls the notify routine with
/// object `this+0x8` and argument `this+0x10`, and answers 1 in
/// AL (the upper bits of EAX are the call's leftover). Returns thiscall.
lf_checker_rt::export!(thiscall, rw_00939810(this: u32) -> u32 {
    unsafe {
        const FLAG: u32 = 0x10;
        const TARGET_OFF: u32 = 0x8;
        const NOTIFY: u32 = 1;
        // NOTE: the flag address is formed before the branch, so the quiet
        // path answers it with the low byte cleared, not zero.
        let slot = (this + FLAG) & 0xFFFF_FF00;
        if ((this + FLAG) as *const u8).read() == 0 {
            slot
        } else {
            let r: u32 = lf_checker_rt::callee_thiscall!(NOTIFY, u32, this + TARGET_OFF, this + FLAG);
            (r & 0xFFFF_FF00) | 1
        }
    }
});
