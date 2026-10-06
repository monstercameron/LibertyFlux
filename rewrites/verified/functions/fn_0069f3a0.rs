// original: 0x0069F3A0 write_slot_if_key_matches
/// Write one table slot when the index is in range and the key matches.
///
/// Reads `count` from `this+0`. When `index >= count` as signed (`jge`) it
/// stores nothing; otherwise it loads the key slot at `this+index*4+0x528`
/// and stores nothing unless it equals `key`. On a match it writes `v0` to
/// `this+index*4+8` and `(v2 << 24) | v1` to `this+index*4+0x298`. All
/// address arithmetic wraps 32-bit. Returns nothing meaningful (eax is a
/// leftover on two of the three paths), so the contract compares no return
/// value; the stores are observed through the heap comparison.
/// Original: thiscall, five stack words, callee cleanup 0x14, no calls.
lf_checker_rt::export!(thiscall, rw_0069f3a0(this: u32, idx: u32, v0: u32, v1: u32, key: u32, v2: u32) -> u32 {
    unsafe {
        const COUNT: u32 = 0;
        const KEYS: u32 = 0x528;
        const VALS: u32 = 8;
        const TAGS: u32 = 0x298;
        let count = (this.wrapping_add(COUNT) as *const u32).read_unaligned();
        if (idx as i32) >= (count as i32) {
            return 0;
        }
        let slot = (this.wrapping_add(idx.wrapping_mul(4)).wrapping_add(KEYS) as *const u32)
            .read_unaligned();
        if slot != key {
            return 0;
        }
        (this.wrapping_add(idx.wrapping_mul(4)).wrapping_add(VALS) as *mut u32).write_unaligned(v0);
        let combined = v2.wrapping_shl(24) | v1;
        (this.wrapping_add(idx.wrapping_mul(4)).wrapping_add(TAGS) as *mut u32)
            .write_unaligned(combined);
        0
    }
});