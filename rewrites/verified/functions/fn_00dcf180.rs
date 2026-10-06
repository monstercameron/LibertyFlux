// original: 0x00dcf180 csv_at_end (proposed)

/// End-of-input test for the csv reader: true once the adjusted read
/// position reaches the limit.
///
/// `this` points to the reader object. The flag byte at `+0x40c` is zero
/// until the buffer has been filled; while it is zero the answer is false.
/// Otherwise the value is `(pos - len + base) >= limit`, where `pos` is the
/// read position at `+0x414`, `len` the buffer length at `+0x410`, `base` the
/// carried file offset at `+0x8` and `limit` at `+0x4`. The subtraction and
/// addition wrap, and the final comparison is SIGNED (the original's `jge`).
///
/// Original: 0x00DCF180 (thiscall, no stack arguments, byte result).
lf_checker_rt::export!(thiscall, rw_00dcf180(this: u32) -> u32 {
    unsafe {
        /// Flag byte: nonzero once the buffer has been filled.
        const FLAG: u32 = 0x40c;
        /// Read position (index of current char in the buffer).
        const POS: u32 = 0x414;
        /// Buffer length (bytes).
        const LEN: u32 = 0x410;
        /// Base offset carried from the file read.
        const BASE: u32 = 0x8;
        /// Limit the adjusted position is compared against (signed).
        const LIMIT: u32 = 0x4;
        if ((this + FLAG) as *const u8).read() == 0 {
            return 0;
        }
        let pos = ((this + POS) as *const i32).read_unaligned();
        let len = ((this + LEN) as *const i32).read_unaligned();
        let base = ((this + BASE) as *const i32).read_unaligned();
        let limit = ((this + LIMIT) as *const i32).read_unaligned();
        // Original: sub then add (wrapping), then SIGNED jge against limit.
        let v = pos.wrapping_sub(len).wrapping_add(base);
        (v >= limit) as u32
    }
});
