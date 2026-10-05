// original: 0x00B83750 row_insert_indexed
/// Insert into block `blk`'s row set, finding or creating the row for `val`.
///
/// Each block holds 26 row slots (`STRIDE` bytes apart, key at `+0x54`).
/// When no slot holds `val`, the first slot keyed -1 is claimed for it.
/// Callee 1 then receives the row with `(p2, p3, p4, p5)`.
///
/// Original: 0x00B83750 (thiscall, six stack arguments, no return value).
lf_checker_rt::export!(thiscall, rw_00B83750(this: u32, blk: u32, val: u32, p2: u32, p3: u32, p4: u32, p5: u32) -> u32 {
    unsafe {
        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn rd8(a: u32) -> u8 {
            unsafe { (a as *const u8).read() }
        }
        #[inline(always)]
        unsafe fn wr8(a: u32, v: u8) {
            unsafe { (a as *mut u8).write(v) }
        }
        const BLOCK: u32 = 0x8FC;
        const SLOTS: u32 = 0x1A;
        const STRIDE: u32 = 0x58;
        const KEY: u32 = 0x54;
        let base = this.wrapping_add(4).wrapping_add(blk.wrapping_mul(BLOCK));
        let mut i = 0u32;
        let mut at = 0u32;
        let mut hit = false;
        while i < SLOTS {
            if rd32(base.wrapping_add(KEY).wrapping_add(i.wrapping_mul(STRIDE))) == val {
                at = i;
                hit = true;
                break;
            }
            i += 1;
        }
        let mut obj = base.wrapping_add(at.wrapping_mul(STRIDE));
        if !hit {
            let mut j = 0u32;
            let mut at2 = 0u32;
            let mut hit2 = false;
            while j < SLOTS {
                if rd32(base.wrapping_add(KEY).wrapping_add(j.wrapping_mul(STRIDE))) == 0xFFFF_FFFF {
                    at2 = j;
                    hit2 = true;
                    break;
                }
                j += 1;
            }
            obj = if hit2 { base.wrapping_add(at2.wrapping_mul(STRIDE)) } else { 0 };
            wr32(obj.wrapping_add(KEY), val);
        }
        if obj == 0 {
            return 0;
        }
        lf_checker_rt::callee_thiscall!(1, u32, obj, p2, p3, p4, p5);
        0
    }
});
