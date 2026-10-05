// original: 0x009AAB10 audio_voice_flag_check (proposed)

/// Voice-slot flag check: tests whether indexed slot `index` (0..3) of the
/// audio voice object `this` currently holds the "ready" flag value 2.
///
/// Layout: a 3-entry index table of bytes at `this + 0x540 + index`, and a
/// record array whose entries are 4 bytes apart. The key is
/// `3 * (table_byte + 2 * index + 0x6a)`; the flag byte lives at
/// `this + 4 * key`. Returns 1 when that byte equals 2, else 0.
/// Out-of-range `index` (>= 3) returns 0 without reading memory.
/// Original: 0x009AAB10 (thiscall, one stack word), returns `al`.
lf_checker_rt::export!(thiscall, rw_009AAB10(this: u32, index: u32) -> u8 {
    unsafe {
        const TABLE_BASE: u32 = 0x540;
        const KEY_BIAS: u32 = 0x6a;
        const FLAG_READY: u8 = 2;
        const MAX_INDEX: u32 = 3;
        if index >= MAX_INDEX {
            return 0;
        }
        let b = (this.wrapping_add(index).wrapping_add(TABLE_BASE) as *const u8).read();
        let k = (b as u32)
            .wrapping_add(index.wrapping_mul(2))
            .wrapping_add(KEY_BIAS);
        let k3 = k.wrapping_mul(3);
        let flag = (this.wrapping_add(k3.wrapping_mul(4)) as *const u8).read();
        (flag == FLAG_READY) as u8
    }
});
