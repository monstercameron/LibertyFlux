// original: 0x00CC82F0 fragInstGta::vf9

/// Returns the shared virtual-method result unless the receiver or nested
/// record blocks it. If receiver flags at `+0x0a` contain `0x0400`, the
/// original leaves that immediate in EAX and clears only AL, producing
/// `0x400`. If the nested record's byte at `+0x24` intersects `0x1c`, it
/// clears only the low byte of the record pointer. Otherwise the method tail
/// dispatches to the shared thiscall helper with `this` and returns its EAX.
lf_checker_rt::export!(thiscall, rw_00cc82f0(this: u32) -> u32 {
    const RECEIVER_FLAGS: u32 = 0x0a;
    const NESTED_RECORD: u32 = 0x0c;
    const RECORD_FLAGS: u32 = 0x24;
    const RECEIVER_BLOCKED: u16 = 0x0400;
    const RECORD_BLOCKED: u8 = 0x1c;
    const FALSE_EAX: u32 = 0x0400;
    const SHARED_HELPER: u32 = 1;

    unsafe {
        let receiver_flags = (this.wrapping_add(RECEIVER_FLAGS) as *const u16).read_unaligned();
        if receiver_flags & RECEIVER_BLOCKED != 0 {
            return FALSE_EAX;
        }

        let record = (this.wrapping_add(NESTED_RECORD) as *const u32).read_unaligned();
        if record != 0 {
            let flags = (record.wrapping_add(RECORD_FLAGS) as *const u8).read();
            if flags & RECORD_BLOCKED != 0 {
                return record & !0xff;
            }
        }

        lf_checker_rt::callee_thiscall!(SHARED_HELPER, u32, this)
    }
});
