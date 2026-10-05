// original: 0x00bed170 flag_bit_by_index_bed170
/// Return one flag bit selected by an index, keeping the index high bytes.
///
/// A jump-table switch on `idx`: cases 0-13 each return a single bit of the
/// flag bytes at `[this+0x54]`/`[this+0x55]`/`[this+0x56]` (see the match
/// arms); any other index returns 0 in the low byte. The upper three bytes
/// of EAX keep the incoming index (the original only ever writes AL), so
/// the result is `(idx & 0xffffff00) | bit`. Reads only. Thiscall, one
/// stack argument.
export!(thiscall, rw_00bed170(this: u32, idx: u32) -> u32 {
    unsafe {
        let bit: u32 = (match idx {
            0 => (((this + 0x54) as *const u8).read() >> 0) & 1,
            1 => (((this + 0x54) as *const u8).read() >> 1) & 1,
            2 => (((this + 0x54) as *const u8).read() >> 2) & 1,
            3 => (((this + 0x54) as *const u8).read() >> 3) & 1,
            4 => (((this + 0x54) as *const u8).read() >> 4) & 1,
            5 => (((this + 0x54) as *const u8).read() >> 5) & 1,
            6 => (((this + 0x54) as *const u8).read() >> 6) & 1,
            7 => (((this + 0x54) as *const u8).read() >> 7) & 1,
            8 => (((this + 0x55) as *const u8).read() >> 0) & 1,
            9 => (((this + 0x55) as *const u8).read() >> 1) & 1,
            10 => (((this + 0x55) as *const u8).read() >> 2) & 1,
            11 => (((this + 0x55) as *const u8).read() >> 3) & 1,
            12 => (((this + 0x55) as *const u8).read() >> 4) & 1,
            13 => (((this + 0x55) as *const u8).read() >> 5) & 1,
            _ => 0,
        }) as u32;
        (idx & 0xFFFF_FF00) | bit
    }
});
