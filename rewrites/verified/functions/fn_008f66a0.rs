// original: 0x008F66A0 BufferedAscii_Lookup

/// Look up entry `idx` of the 16-bit table at `this + TABLE_OFF` and store it
/// through `out`.
///
/// `*out` is zeroed first, always. Then: when the pending flag byte at `+0`
/// is 0 the function fails and returns 0; when `idx` is at least `MAX_INDEX`
/// **compared as signed** it fails and returns `idx` with its low byte
/// cleared (the original only clears AL, leaving the upper 24 bits of the
/// index in place); when the fetched word is 0 it fails and returns 0.
/// Otherwise `*out` holds the word and the result is the word with its low
/// byte forced to 1 (again only AL is written). Convention: thiscall, two
/// stack words `(idx, out)`.
lf_checker_rt::export!(thiscall, rw_008f66a0(this: u32, idx: u32, out: u32) -> u32 {
    unsafe {
        const MAX_INDEX: i32 = 30;
        const TABLE_OFF: u32 = 8;
        (out as *mut u16).write_unaligned(0);
        if (this as *const u8).read() == 0 {
            return 0;
        }
        if (idx as i32) >= MAX_INDEX {
            return idx & 0xFFFF_FF00;
        }
        let addr = this
            .wrapping_add(idx.wrapping_mul(2))
            .wrapping_add(TABLE_OFF);
        let v = (addr as *const u16).read_unaligned();
        (out as *mut u16).write_unaligned(v);
        if v == 0 {
            0
        } else {
            ((v as u32) & 0xFF00) | 1
        }
    }
});
