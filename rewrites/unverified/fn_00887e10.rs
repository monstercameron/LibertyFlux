// original: 0x00887E10 stream_rebase (proposed)

/// Rebase the stream's row tables from relative offsets to addresses.
///
/// `s` (at `this + 0x3c`) holds two row tables as self-relative offsets:
/// the words at `s` and `s + 8` are first relocated by adding `s` itself.
/// Each of the `count1` rows at `[s]` (16 bytes) is then relocated by the
/// table end, and the object it points to is relocated by `s + [s+0x18]`,
/// with the secondary pointer at `+0x20` relocated too when flag 0x400 is
/// set at `+0x1c`. Each of the `count2` rows at `[s+8]` is relocated by
/// its table end. The answer is `s`.
///
/// Original: 0x00887E10 (thiscall, no stack words).
lf_checker_rt::export!(thiscall, rw_00887E10(this: u32) -> u32 {
    unsafe {
        const STATE: u32 = 0x3c;
        const TAB0: u32 = 0x00;
        const TAB1: u32 = 0x08;
        const COUNT0: u32 = 0x10;
        const COUNT1: u32 = 0x14;
        const RELOC: u32 = 0x18;
        const ROW: u32 = 16;
        const FLAG_OFF: u32 = 0x1c;
        const FLAG_BIT: u32 = 0x400;
        const SUB_OFF: u32 = 0x20;
        let s = ((this + STATE) as *const u32).read_unaligned();
        let t0 = ((s + TAB0) as *const u32).read_unaligned();
        ((s + TAB0) as *mut u32).write_unaligned(t0.wrapping_add(s));
        let t0n = ((s + TAB0) as *const u32).read_unaligned();
        let t1 = ((s + TAB1) as *const u32).read_unaligned();
        ((s + TAB1) as *mut u32).write_unaligned(t1.wrapping_add(s));
        let n0 = ((s + COUNT0) as *const u32).read_unaligned();
        let end0 = n0.wrapping_mul(ROW).wrapping_add(t0n);
        let mut i = 0u32;
        while i < n0 {
            let at = t0n.wrapping_add(i.wrapping_mul(ROW));
            let v = ((at) as *const u32).read_unaligned();
            ((at) as *mut u32).write_unaligned(v.wrapping_add(end0));
            let p = ((at) as *const u32).read_unaligned();
            let r = ((s + RELOC) as *const u32).read_unaligned();
            let pv = ((p) as *const u32).read_unaligned();
            ((p) as *mut u32).write_unaligned(pv.wrapping_add(r.wrapping_add(s)));
            let f = ((p + FLAG_OFF) as *const u32).read_unaligned();
            if f & FLAG_BIT != 0 {
                let sv = ((p + SUB_OFF) as *const u32).read_unaligned();
                ((p + SUB_OFF) as *mut u32)
                    .write_unaligned(sv.wrapping_add(p));
            }
            i = i.wrapping_add(1);
        }
        let n1 = ((s + COUNT1) as *const u32).read_unaligned();
        let t1b = ((s + TAB1) as *const u32).read_unaligned();
        let end1 = n1.wrapping_mul(ROW).wrapping_add(t1b);
        let mut j = 0u32;
        while j < n1 {
            let t = ((s + TAB1) as *const u32).read_unaligned();
            let at = t.wrapping_add(j.wrapping_mul(ROW));
            let v = ((at) as *const u32).read_unaligned();
            ((at) as *mut u32).write_unaligned(v.wrapping_add(end1));
            j = j.wrapping_add(1);
        }
        s
    }
});
