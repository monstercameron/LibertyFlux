// original: 0x00620FF0 net_record_copy_32 (proposed)

/// Copy up to 32 fixed-size records from an object's pointer table into a
/// dense caller buffer, merging the flag bits of the last byte.
///
/// `this` points to an object holding a record-pointer table at `+0x1d94`
/// (one dword per record) and a record count at `+0x1e14`. `dest` points to
/// the output buffer, laid out as records of `0x70` bytes back to back. The
/// second stack argument is never read.
///
/// For each index `i` in `0..min(count, 32)` (both bounds compared
/// UNSIGNED: the entry test is `jbe` against 0, the cap test is `jae`
/// against `0x20`, the loop test is `jb` against the count), the record at
/// `table[i]` is copied to `dest + i * 0x70`: dwords over `[0, 0x18)`,
/// then a 16-bit word at `0x18`, a dword at `0x1c`, a word at `0x20`, a
/// dword at `0x24`, a word at `0x28`, dwords over `[0x2c, 0x5c)`, bytes
/// over `[0x5c, 0x6d)`, and byte `0x6d` by bit merge (bits 0-1 taken from
/// the source, bits 2-7 kept from the destination). The word pairs leave
/// three two-byte holes (`0x1a`, `0x22`, `0x2a`) that are never written,
/// and bytes `[0x6e, 0x70)` of each destination record are left untouched.
///
/// Returns the number of records copied (`min(count, 32)` as u32).
///
/// Edge cases: a count of 0 copies nothing and returns 0; counts above 32,
/// including values with the high bit set (`0x80000000`, `0xFFFFFFFF`),
/// still copy exactly 32 records, because the cap comparison is unsigned.
///
/// Original: 0x00620FF0 (thiscall, ecx = this, two stack words of which the
/// second is unread; no outgoing calls; moves data through vector registers
/// but performs no floating-point arithmetic).
lf_checker_rt::export!(thiscall, rw_00620FF0(this: u32, dest: u32, _unused: u32) -> u32 {
    unsafe {
        const TABLE_OFF: u32 = 0x1d94;
        const COUNT_OFF: u32 = 0x1e14;
        const MAX_RECORDS: u32 = 32;
        const STRIDE: u32 = 0x70;
        const HEAD_DWORDS: u32 = 6; // dwords covering [0, 0x18)
        const MIXED_OFF: u32 = 0x18; // word,dword,word,dword,word over [0x18, 0x2c)
        const BODY_START: u32 = 0x2c; // dwords covering [0x2c, 0x5c)
        const BODY_END: u32 = 0x5c;
        const TAIL_START: u32 = 0x5c;
        const TAIL_BYTES: u32 = 0x11; // bytes covering [0x5c, 0x6d)
        const MERGE_OFF: u32 = 0x6d;
        const MERGE_SRC_MASK: u8 = 0x03;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn rd16(a: u32) -> u16 {
            unsafe { (a as *const u16).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr16(a: u32, v: u16) {
            unsafe { (a as *mut u16).write_unaligned(v) }
        }

        let count = rd32(this.wrapping_add(COUNT_OFF));
        // Unsigned minimum: the original caps with jae/jb, so counts with
        // the high bit set still yield 32 records, never 0.
        let n = if count > MAX_RECORDS { MAX_RECORDS } else { count };
        let mut i = 0u32;
        while i < n {
            let src = rd32(
                this.wrapping_add(TABLE_OFF)
                    .wrapping_add(i.wrapping_mul(4)),
            );
            let d = dest.wrapping_add(i.wrapping_mul(STRIDE));
            let mut w = 0u32;
            while w < HEAD_DWORDS {
                let off = w.wrapping_mul(4);
                wr32(d.wrapping_add(off), rd32(src.wrapping_add(off)));
                w += 1;
            }
            // Mixed word/dword fields; the two bytes after each word are
            // padding the original never writes.
            let m = MIXED_OFF;
            wr16(d.wrapping_add(m), rd16(src.wrapping_add(m)));
            wr32(d.wrapping_add(m + 4), rd32(src.wrapping_add(m + 4)));
            wr16(d.wrapping_add(m + 8), rd16(src.wrapping_add(m + 8)));
            wr32(d.wrapping_add(m + 12), rd32(src.wrapping_add(m + 12)));
            wr16(d.wrapping_add(m + 16), rd16(src.wrapping_add(m + 16)));
            let mut o = BODY_START;
            while o < BODY_END {
                wr32(d.wrapping_add(o), rd32(src.wrapping_add(o)));
                o += 4;
            }
            let mut b = 0u32;
            while b < TAIL_BYTES {
                let sa = src.wrapping_add(TAIL_START).wrapping_add(b);
                let da = d.wrapping_add(TAIL_START).wrapping_add(b);
                (da as *mut u8).write((sa as *const u8).read());
                b += 1;
            }
            let sflag = (src.wrapping_add(MERGE_OFF) as *const u8).read();
            let dflag = (d.wrapping_add(MERGE_OFF) as *const u8).read();
            (d.wrapping_add(MERGE_OFF) as *mut u8)
                .write((dflag & !MERGE_SRC_MASK) | (sflag & MERGE_SRC_MASK));
            i += 1;
        }
        n
    }
});
