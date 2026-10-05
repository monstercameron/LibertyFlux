// original: 0x00a9f4d0 stream_entry_lookup_or_alloc (proposed)

/// Look up a 16-byte entry by index, or allocate the first free one.
///
/// `this` points to an object with an allocation counter at `+0x38b0` and
/// 128 entries of 16 bytes at `+0x38b4`. Each entry holds an id word at
/// `+0`, a flag byte at `+4`, a zero word at `+8` and a `-1` word at `+12`.
///
/// When `idx` is above 0x7f (0xffffffff included, compared unsigned) the
/// table is scanned for the first entry whose flag byte is 0; if none is
/// free the result is null. The winner's flag becomes 1, its id becomes the
/// old counter biased by 0x10000000, words `+8`/`+12` are set to 0/`-1`,
/// the counter is incremented, and the entry address is returned. Otherwise
/// entry `idx` is returned when its flag byte is exactly 1, else null.
///
/// Original: 0x00a9f4d0 (thiscall, one stack word).
lf_checker_rt::export!(thiscall, rw_00a9f4d0(this: u32, idx: u32) -> u32 {
    unsafe {
        const TABLE_OFF: u32 = 0x38b4;
        const FLAG_DELTA: u32 = 4;
        const ENTRY_STRIDE: u32 = 16;
        const ENTRY_COUNT: u32 = 128;
        const MAX_INDEX: u32 = 0x7f;
        const ALLOC_COUNT_OFF: u32 = 0x38b0;
        const ID_BIAS: u32 = 0x10000000;
        if idx > MAX_INDEX {
            let mut i = 0u32;
            while i < ENTRY_COUNT {
                let e = this
                    .wrapping_add(TABLE_OFF)
                    .wrapping_add(i.wrapping_mul(ENTRY_STRIDE));
                if ((e + FLAG_DELTA) as *const u8).read() == 0 {
                    ((e + FLAG_DELTA) as *mut u8).write(1);
                    let n = ((this + ALLOC_COUNT_OFF) as *const u32).read_unaligned();
                    (e as *mut u32).write_unaligned(n.wrapping_add(ID_BIAS));
                    ((e + 8) as *mut u32).write_unaligned(0);
                    ((e + 12) as *mut u32).write_unaligned(0xffff_ffff);
                    ((this + ALLOC_COUNT_OFF) as *mut u32).write_unaligned(n.wrapping_add(1));
                    return e;
                }
                i += 1;
            }
            return 0;
        }
        let e = this
            .wrapping_add(TABLE_OFF)
            .wrapping_add(idx.wrapping_mul(ENTRY_STRIDE));
        if ((e + FLAG_DELTA) as *const u8).read() == 1 {
            e
        } else {
            0
        }
    }
});
