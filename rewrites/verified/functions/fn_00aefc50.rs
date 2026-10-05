// original: 0x00AEFC50 stream_table_fetch (proposed)

/// Fetch a key's payload from the stream table into the out word.
///
/// Probes the table state (the probe callee's low byte is stamped into
/// this call's own dead key stack slot), then lower-bounds `key` over the
/// table (`[this]`, count at `this+4` words of 8 bytes) with the key
/// passed through this call's own scratch slot. When the bound equals the
/// table end or points at a different key, answers 0; otherwise stores the
/// bound entry's payload through `out` and answers 1.
///
/// Original: 0x00AEFC50 (thiscall, two stack words, two direct callees; the
/// search takes a frame-pointer key address).
lf_checker_rt::export!(thiscall, rw_00aefc50(this: u32, key: u32, out: u32) -> u32 {
    unsafe {
        const PROBE_CALLEE: u32 = 1;
        const SEARCH_CALLEE: u32 = 2;
        const BASE_OFF: u32 = 0;
        const COUNT_OFF: u32 = 4;
        const ENTRY: u32 = 8;
        let probe: u32 = lf_checker_rt::callee_cdecl!(PROBE_CALLEE, u32, 0, 0);
        let base = ((this.wrapping_add(BASE_OFF)) as *const u32).read_unaligned();
        let count = ((this.wrapping_add(COUNT_OFF)) as *const u16).read_unaligned() as u32;
        let end = base.wrapping_add(count.wrapping_mul(ENTRY));
        let kslot = key;
        let stamped = (key & 0xFFFF_FF00) | (probe & 0xFF);
        let lb: u32 = lf_checker_rt::callee_cdecl!(
            SEARCH_CALLEE, u32, base, end,
            core::ptr::addr_of!(kslot) as u32, stamped, 0);
        if lb == end {
            return 0;
        }
        if ((lb) as *const u32).read_unaligned() != key {
            return 0;
        }
        ((out) as *mut u32).write_unaligned(((lb.wrapping_add(4)) as *const u32).read_unaligned());
        1
    }
});
