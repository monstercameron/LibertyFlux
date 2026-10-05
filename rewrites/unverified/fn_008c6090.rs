// original: 0x008C6090 stream_slot_match_fe
/// Compare a caller key against a streaming slot's entry, when enabled.
///
/// `this` is the streaming control block, `key` an opaque caller value and
/// `idx` a slot index. When the flag byte at `this + idx + 0xFE` is clear
/// the result is 0 without calling out. Otherwise the entry pointer
/// `this + idx * 8 + 0x1EC` and the key go to the comparison callee, and
/// the result is 1 exactly when it reports equality (0). Only the low
/// result byte is behaviour. Original: thiscall, two stack words.
lf_checker_rt::export!(thiscall, rw_008c6090(this: u32, key: u32, idx: u32) -> u32 {
    unsafe {
        const FLAG_BASE: u32 = 0xFE;
        const ENTRY_BASE: u32 = 0x1EC;
        const ENTRY_STRIDE: u32 = 8;
        const CMP_CALLEE: u32 = 1;
        let flag = this.wrapping_add(idx).wrapping_add(FLAG_BASE);
        if (flag as *const u8).read() == 0 {
            return 0;
        }
        let entry = this.wrapping_add(idx.wrapping_mul(ENTRY_STRIDE))
            .wrapping_add(ENTRY_BASE);
        let diff: u32 = lf_checker_rt::callee_cdecl!(CMP_CALLEE, u32, key, entry);
        (diff == 0) as u32
    }
});
