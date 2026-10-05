// original: 0x0061D3B0 net_init_channel_fwd_a

/// Init a channel, then copy records from a packed source table.
///
/// Forwards this call to the shared channel initializer, then copies
/// `count` 16-byte records from `src` into the table at `this+0x298`.
/// `this` is the channel object, `a0`..`a6` are passed through to the
/// initializer untouched, `src` points at records laid out with stride
/// 16 (`0x10`-byte payload in each), `count` is a signed count:
/// zero or negative copies nothing. The count is also stored at
/// `this+0x290`. Returns the initializer's answer when nothing is
/// copied, otherwise the end address of the copied range.
/// Original: 0x0061D3B0 (thiscall, nine stack words).
lf_checker_rt::export!(thiscall, rw_0061D3B0(this: u32, a0: u32, a1: u32, a2: u32, a3: u32, a4: u32, a5: u32, a6: u32, src: u32, count: u32) -> u32 {
    unsafe {
        const INIT_CALLEE: u32 = 1;
        const COUNT_SLOT: u32 = 0x290;
        const TABLE_BASE: u32 = 0x298;
        const RECORD_BYTES: u32 = 0x10;
        const SRC_STRIDE: u32 = 0x10;
        let answer =
            lf_checker_rt::callee_thiscall!(INIT_CALLEE, u32, this, a0, a1, a2, a3, a4, a5, a6);
        ((this + COUNT_SLOT) as *mut u32).write_unaligned(count);
        let n = count as i32;
        if n <= 0 {
            return answer;
        }
        let mut from = src;
        let mut to = this + TABLE_BASE;
        let mut left = n as u32;
        while left > 0 {
            let lo = (from as *const u64).read_unaligned();
            let hi = ((from + 8) as *const u64).read_unaligned();
            (to as *mut u64).write_unaligned(lo);
            ((to + 8) as *mut u64).write_unaligned(hi);
            from = from.wrapping_add(SRC_STRIDE);
            to = to.wrapping_add(RECORD_BYTES);
            left -= 1;
        }
        to
    }
});
