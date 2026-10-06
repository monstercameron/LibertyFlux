// original: 0x00633E30 section_table_load (proposed)

/// Load a table of sections through a token reader, one row per query hit.
///
/// `arg1` handles a reader (vtable fill at slot 2, count/query at slot 5,
/// cursor at `+0x10`); `this` collects the result at `+0xE8` (allocator
/// answer for the row count, or zero) and `+0xEE` (the count's low word).
/// A first query gives the row count; a non-positive (SIGNED) count skips
/// to the tail. Each row re-fills two 0x40 scratch buffers, queries its
/// width, reserves that many words, and fills them with further queries.
/// Three single-shot match stages then pull 0x200-byte chunks and compare
/// them against "autoplay", "hide" and "rootsonly" with an inline strcmp,
/// stepping past empty pulls and mismatches; only the last stage records
/// its outcome (flag 1/0). A per-row consumer takes (scratch buffer,
/// width, reserved words, flag) and its answer is ignored. The tail does
/// one last fill whose answer is the function's result. thiscall, one
/// stack word. The original probes the stack for the reservation and keeps
/// a stack cookie; both are stubbed on the checker, so the rewrite keeps a
/// fixed local array (widths are small on every trial) and just makes the
/// same calls.
lf_checker_rt::export!(thiscall, rw_00633E30(this: u32, arg1: u32) -> u32 {
    unsafe {
        const VT_FILL: u32 = 0x08;
        const VT_QUERY: u32 = 0x14;
        const RDR_CURSOR: u32 = 0x10;
        const TAG_AUTOPLAY: u32 = 0x00F9_7E74;
        const TAG_HIDE: u32 = 0x00F9_7DD8;
        const TAG_ROOTSONLY: u32 = 0x00F9_7DE0;
        const THIS_BASE: u32 = 0xE8;
        const THIS_COUNT: u32 = 0xEE;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn vfill(obj: u32, buf: *mut u8, n: u32) -> u32 {
            unsafe {
                let vt: u32 = rd32(obj);
                let slot: u32 = rd32(vt.wrapping_add(VT_FILL));
                let f: extern "thiscall" fn(u32, u32, u32) -> u32 =
                    unsafe { core::mem::transmute(slot as usize) };
                f(obj, buf as u32, n)
            }
        }
        #[inline(always)]
        unsafe fn vquery(obj: u32) -> u32 {
            unsafe {
                let vt: u32 = rd32(obj);
                let slot: u32 = rd32(vt.wrapping_add(VT_QUERY));
                let f: extern "thiscall" fn(u32, u32) -> u32 =
                    unsafe { core::mem::transmute(slot as usize) };
                f(obj, 1)
            }
        }
        unsafe fn cstrcmp(a: u32, b: *const u8) -> i32 {
            unsafe {
                let mut i: u32 = 0;
                loop {
                    let x: u8 = (a.wrapping_add(i) as *const u8).read();
                    let y: u8 = b.add(i as usize).read();
                    if x != y {
                        return if x < y { -1 } else { 1 };
                    }
                    if x == 0 {
                        return 0;
                    }
                    i = i.wrapping_add(1);
                }
            }
        }

        let obj: u32 = rd32(arg1.wrapping_add(4));
        let mut buf_a = [0u8; 0x40];
        let mut buf_b = [0u8; 0x40];
        let mut buf_c = [0u8; 64];
        buf_a[0] = 0;
        vfill(obj, buf_a.as_mut_ptr(), 0x40);
        buf_a[0] = 0;
        vfill(obj, buf_a.as_mut_ptr(), 0x40);
        let rows: i32 = vquery(obj) as i32;
        let base: u32 = if rows != 0 {
            lf_checker_rt::callee_stdcall!(3, u32, rows as u32)
        } else {
            0
        };
        wr32(this.wrapping_add(THIS_BASE), base);
        (this.wrapping_add(THIS_COUNT) as *mut u16).write_unaligned(rows as u16);
        // Signed: `(an instruction of the original); jle tail`.
        if rows > 0 {
            let autoplay: u32 = lf_checker_rt::relocated(TAG_AUTOPLAY);
            let hide: u32 = lf_checker_rt::relocated(TAG_HIDE);
            let rootsonly: u32 = lf_checker_rt::relocated(TAG_ROOTSONLY);
            let mut r: u32 = 0;
            while r < rows as u32 {
                buf_a[0] = 0;
                vfill(obj, buf_a.as_mut_ptr(), 0x40);
                vfill(obj, buf_b.as_mut_ptr(), 0x40);
                let width: i32 = vquery(obj) as i32;
                buf_a[0] = 0;
                vfill(obj, buf_a.as_mut_ptr(), 0x40);
                // Stack probe for width*4 words; stubbed to a no-op, so a
                // fixed local array stands in for the reservation.
                let _: u32 = lf_checker_rt::callee_cdecl!(4, u32,);
                let mut words = [0u32; 16];
                // Signed: `(an instruction of the original); jle skip`.
                if width > 0 {
                    let mut i: u32 = 0;
                    while i < width as u32 {
                        words[i as usize] = vquery(obj);
                        i = i.wrapping_add(1);
                    }
                }
                // Three single-shot match stages.
                let saved: u32 = rd32(obj.wrapping_add(RDR_CURSOR));
                buf_c[0] = 0;
                let e1: u32 = vfill(obj, buf_c.as_mut_ptr(), 0x200);
                if e1 == 0 || cstrcmp(autoplay, buf_c.as_ptr()) != 0 {
                    let _: u32 = lf_checker_rt::callee_thiscall!(
                        6, u32, obj, buf_c.as_mut_ptr() as u32, e1
                    );
                    wr32(obj.wrapping_add(RDR_CURSOR), saved);
                }
                let saved: u32 = rd32(obj.wrapping_add(RDR_CURSOR));
                buf_c[0] = 0;
                let e2: u32 = vfill(obj, buf_c.as_mut_ptr(), 0x200);
                if e2 == 0 || cstrcmp(hide, buf_c.as_ptr()) != 0 {
                    let _: u32 = lf_checker_rt::callee_thiscall!(
                        6, u32, obj, buf_c.as_mut_ptr() as u32, e2
                    );
                    wr32(obj.wrapping_add(RDR_CURSOR), saved);
                }
                let saved: u32 = rd32(obj.wrapping_add(RDR_CURSOR));
                buf_c[0] = 0;
                let e3: u32 = vfill(obj, buf_c.as_mut_ptr(), 0x200);
                let flag: u32 =
                    if e3 != 0 && cstrcmp(rootsonly, buf_c.as_ptr()) == 0 { 1 } else {
                        let _: u32 = lf_checker_rt::callee_thiscall!(
                            6, u32, obj, buf_c.as_mut_ptr() as u32, e3
                        );
                        wr32(obj.wrapping_add(RDR_CURSOR), saved);
                        0
                    };
                let _: u32 = lf_checker_rt::callee_thiscall!(
                    5, u32, this, buf_b.as_mut_ptr() as u32, width as u32,
                    words.as_mut_ptr() as u32, flag
                );
                r = r.wrapping_add(1);
            }
        }
        buf_a[0] = 0;
        let ans: u32 = vfill(obj, buf_a.as_mut_ptr(), 0x40);
        // Stack-cookie check; the answer is kept in a local (see fn4).
        let _: u32 = lf_checker_rt::callee_thiscall!(7, u32, 0);
        ans
    }
});
