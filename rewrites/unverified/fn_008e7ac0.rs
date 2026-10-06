// original: 0x008E7AC0 count_matching_rows_callback (proposed)

/// Count the rows accepted by a per-row callback.
///
/// Same traversal as `count_flagged_rows`: `desc` gives the iteration count
/// (low 4 bits of `DESC_ITERS`), the unsigned base key (`DESC_KEY`) and the
/// signed start offset (`DESC_OFF`); each row word's low 16 bits index the
/// entry table at `ROW_TABLE`, null entries are skipped. For a live entry
/// the callback (callee 1, thiscall on `obj`) is invoked with the entry
/// pointer plus the row word's high 16 bits scaled by `HI_SCALE`, and the
/// row is counted when the callback's low byte is non-zero. Returns the
/// count.
///
/// Original: 0x008E7AC0 (thiscall, one stack argument).
lf_checker_rt::export!(thiscall, rw_008E7AC0(obj: u32, desc: u32) -> u32 {
    unsafe {
        /// Offset of the entry-pointer table (indexed by row word low 16).
        const ROW_TABLE: u32 = 0x804;
        /// Offset of the row-base table (indexed by descriptor key).
        const BASE_TABLE: u32 = 0x904;
        /// Descriptor offset of the unsigned 16-bit base key.
        const DESC_KEY: u32 = 8;
        /// Descriptor offset of the signed 16-bit start offset.
        const DESC_OFF: u32 = 0x12;
        /// Descriptor offset of the iteration-count byte.
        const DESC_ITERS: u32 = 0x1E;
        /// Low bits of the count byte that form the count.
        const ITERS_MASK: u8 = 0x0F;
        /// Row stride in bytes.
        const ROW_STRIDE: u32 = 8;
        /// Scale applied to the row word's high 16 bits.
        const HI_SCALE: u32 = 5;
        /// Per-row callback callee id.
        const CB: u32 = 1;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rd16(a: u32) -> u16 {
            unsafe { (a as *const u16).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rd8(a: u32) -> u8 {
            unsafe { (a as *const u8).read() }
        }

        let iters = u32::from(rd8(desc.wrapping_add(DESC_ITERS)) & ITERS_MASK);
        if iters == 0 {
            return 0;
        }
        let key = u32::from(rd16(desc.wrapping_add(DESC_KEY)));
        let off = rd16(desc.wrapping_add(DESC_OFF)) as i16 as i32;
        let base = rd32(obj.wrapping_add(BASE_TABLE).wrapping_add(key.wrapping_mul(4)));
        let mut cur = base.wrapping_add((off.wrapping_mul(ROW_STRIDE as i32)) as u32);
        let mut n = iters;
        let mut count: u32 = 0;
        while n != 0 {
            let v = rd32(cur);
            let entry = rd32(
                obj.wrapping_add(ROW_TABLE)
                    .wrapping_add((v & 0xFFFF).wrapping_mul(4)),
            );
            if entry != 0 {
                let arg = entry.wrapping_add((v >> 16) << HI_SCALE);
                let ans: u32 = lf_checker_rt::callee_thiscall!(CB, u32, obj, arg);
                if (ans & 0xFF) != 0 {
                    count = count.wrapping_add(1);
                }
            }
            cur = cur.wrapping_add(ROW_STRIDE);
            n = n.wrapping_sub(1);
        }
        count
    }
});
