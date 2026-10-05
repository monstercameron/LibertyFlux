// original: 0x00681680 euphoria_deserialize_build (proposed)

/// Load or store this object's fields through a stream, then build or walk
/// its two element lists.
///
/// `this` carries a flags word at `+6`, small counts at `+8`/`+0xa`, a dword
/// at `+0xc`, list B (array at `+0x14`, counts at `+0x18` / `+0x1a`), list A
/// (array at `+0x20`, count at `+0x24`, high word at `+0x26`) and a word at
/// `+0x28`. The `edi` object carries only the direction bit at `+0` (set:
/// load, clear: store), a version at `+2` and the stream at `+4`. Six
/// (pointer, length) pairs are
/// transferred through the stream helpers (callee ids 1-6 load, 7-12
/// store): a flags temp, `+0x28`, `+8`, `+0xc`, `+0xa` and a count temp.
/// When the version is below 9 three global floats are passed to the vector
/// helper (id 19); below 7 the `+0xa` field is forced to `0xffff` instead of
/// being transferred.
///
/// List A then runs for the count: loading allocates each element through
/// the TLS allocator chain (slot 0, `+8` slot, called with (0x10, 0x10, 0);
/// id 13), initialises it and appends it while the stored count advances;
/// storing reads each element from the existing array. Every element gets a
/// row allocation (id 15, skipped when the row count is 0) and a visit call
/// (id 18). The second half runs on load only: list B is allocated (id 16)
/// unless already present, each entry is allocated (id 13 again), zeroed and
/// given a row (id 17) and a selector word derived from the counts (one
/// past the inner quotient while the outer index is below it, otherwise one
/// past the remainder of (count − 1) by the divisor, or one past the divisor
/// when that remainder is 0), and each row is filled by copying one
/// column out of the source array's rows. All divisions are exact signed
/// `idiv` semantics; every divisor is pinned or scripted nonzero (see the
/// contract), matching the original's fault-free paths.
///
/// The stub writes full words even for two-byte fields, so a load clobbers
/// the two bytes past a short field; both sides see the same stub writes.
/// The TLS allocator answers per call (a fresh block each time, like a real
/// allocator): list-A and list-B elements must not share, because list B
/// zeroes fields list A later reads back as row pointers. The row and table
/// allocators answer per trial (shared across iterations); rows are only
/// written, so sharing is invisible but for the final heap. The fresh-block
/// nonzero check is dead (the block was just zeroed) and never takes, on
/// either side.
///
/// Original: 0x00681680 (thiscall, ECX + one stack word, callee pops 4,
/// no meaningful return).
lf_checker_rt::export!(thiscall, rw_00681680(ecx: u32, edi_obj: u32) -> u32 {
    unsafe {
        const R_FLAG: u32 = 1;
        const R_P28: u32 = 2;
        const R_N: u32 = 3;
        const R_DW: u32 = 4;
        const R_D: u32 = 5;
        const R_CNT: u32 = 6;
        const W_FLAG: u32 = 7;
        const W_P28: u32 = 8;
        const W_N: u32 = 9;
        const W_DW: u32 = 10;
        const W_D: u32 = 11;
        const W_CNT: u32 = 12;
        const MALLOC: u32 = 13;
        const ALLOC_A: u32 = 14;
        const ALLOC_ROW: u32 = 15;
        const ALLOC_B: u32 = 16;
        const ALLOC_F: u32 = 17;
        const VISIT: u32 = 18;
        const VEC3: u32 = 19;
        const VTABLE_MALLOC_SLOT: u32 = 8;
        const G0: u32 = 0x1b4b2a0;
        const G1: u32 = 0x1b4b2a4;
        const G2: u32 = 0x1b4b2a8;
        const ELEM_MAGIC: u32 = 0xffff_0fff;
        const ELEM_MAGIC2: u32 = 0x7f;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rd16(a: u32) -> u32 {
            unsafe { (a as *const u16).read_unaligned() as u32 }
        }
        #[inline(always)]
        unsafe fn rd8(a: u32) -> u8 {
            unsafe { (a as *const u8).read() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn wr16(a: u32, v: u16) {
            unsafe { (a as *mut u16).write_unaligned(v) }
        }

        /// One stream transfer: load id when `dir`, else store id.
        #[inline(always)]
        unsafe fn io(rd: u32, wr: u32, dir: bool, stream: u32, ptr: u32, len: u32) {
            unsafe {
                if dir {
                    let _: u32 = lf_checker_rt::callee_thiscall!(rd, u32, stream, ptr, len);
                } else {
                    let _: u32 = lf_checker_rt::callee_thiscall!(wr, u32, stream, ptr, len);
                }
            }
        }

        /// Allocate 0x10 bytes through the TLS chain: slot 0, `+8` slot.
        #[inline(always)]
        unsafe fn tls_alloc(tls: u32) -> u32 {
            unsafe {
                let ctx = rd32(tls + 8);
                let table = rd32(ctx);
                let alloc: extern "thiscall" fn(u32, u32, u32, u32) -> u32 =
                    core::mem::transmute(rd32(table + VTABLE_MALLOC_SLOT) as usize);
                alloc(ctx, 0x10, 0x10, 0)
            }
        }

        /// `((n - 2) / d) + 1` masked to a word, exact `idiv` semantics.
        #[inline(always)]
        fn div_count(n: u32, d: u32) -> u32 {
            ((((n as i32) - 2) / (d as i32) + 1) & 0xffff) as u32
        }

        let esi = ecx;
        let edi = edi_obj;
        let stream = rd32(edi + 4);
        let dir = rd8(edi) & 1 != 0;
        let tls = lf_checker_rt::tls_slot(0);

        let mut t_flag = rd16(esi + 6) & 0xf8ff;
        io(R_FLAG, W_FLAG, dir, stream, &mut t_flag as *mut u32 as u32, 2);
        if dir {
            wr16(esi + 6, (t_flag & 0xf8ff) as u16);
        }
        io(R_P28, W_P28, dir, stream, esi + 0x28, 2);
        io(R_N, W_N, dir, stream, esi + 8, 2);
        io(R_DW, W_DW, dir, stream, esi + 0x0c, 4);

        if rd16(edi + 2) < 9 {
            let mut v = [
                rd32(lf_checker_rt::relocated(G0)),
                rd32(lf_checker_rt::relocated(G1)),
                rd32(lf_checker_rt::relocated(G2)),
            ];
            let _: u32 =
                lf_checker_rt::callee_thiscall!(VEC3, u32, edi, v.as_mut_ptr() as u32);
        }
        let a_ptr = esi + 0x0a;
        if rd16(edi + 2) < 7 {
            wr16(esi + 0x0a, 0xffff);
        } else {
            io(R_D, W_D, dir, stream, esi + 0x0a, 2);
        }

        let n = rd16(esi + 8);
        let cnt2 = if n > 1 { div_count(n, rd16(a_ptr)) } else { 1 };

        let mut t_cnt = rd16(esi + 0x24);
        io(R_CNT, W_CNT, dir, stream, &mut t_cnt as *mut u32 as u32, 2);
        let count = t_cnt & 0xffff;

        if dir {
            if count != 0 {
                let arr = lf_checker_rt::callee_stdcall!(ALLOC_A, u32, count);
                wr32(esi + 0x20, arr);
            } else {
                wr32(esi + 0x20, 0);
            }
            wr16(esi + 0x26, count as u16);
        }

        let mut i = 0u32;
        while i < count {
            let elem: u32;
            if dir {
                let b = tls_alloc(tls);
                elem = if b != 0 {
                    wr32(b, ELEM_MAGIC);
                    wr32(b + 4, ELEM_MAGIC2);
                    wr32(b + 8, 0);
                    wr32(b + 0x0c, 0);
                    b
                } else {
                    0
                };
                let idx = rd16(esi + 0x24);
                wr16(esi + 0x24, idx.wrapping_add(1) as u16);
                wr32(rd32(esi + 0x20) + idx * 4, elem);
                let e2 = cnt2 & 0xffff;
                if e2 != 0 {
                    let q = lf_checker_rt::callee_stdcall!(ALLOC_ROW, u32, e2);
                    wr16(elem + 0x0e, e2 as u16);
                    wr32(elem + 8, q);
                } else {
                    wr16(elem + 0x0e, 0);
                    wr32(elem + 8, 0);
                }
            } else {
                elem = rd32(rd32(esi + 0x20) + i * 4);
            }
            let _: u32 = lf_checker_rt::callee_thiscall!(VISIT, u32, elem, edi);
            i += 1;
        }

        if dir {
            let cnt2b = cnt2 & 0xffff;
            if rd16(esi + 0x1a) == 0 {
                wr16(esi + 0x1a, cnt2b as u16);
                if cnt2b != 0 {
                    let arrb = lf_checker_rt::callee_stdcall!(ALLOC_B, u32, cnt2b);
                    wr32(esi + 0x14, arrb);
                } else {
                    wr32(esi + 0x14, 0);
                }
            }
            // Runs on both sides of the +0x1a check (the skip jumps here).
            wr16(esi + 0x18, cnt2b as u16);
            let mut s2 = 0u32;
            while s2 < cnt2b {
                let b = tls_alloc(tls);
                let e = if b != 0 {
                    wr32(b, 0);
                    wr32(b + 4, 0);
                    wr32(b + 8, 0);
                    wr16(b + 0x0c, 0);
                    b
                } else {
                    0
                };
                wr32(rd32(esi + 0x14) + s2 * 4, e);
                let e2 = rd32(rd32(esi + 0x14) + s2 * 4);
                let cnt_a = count;
                if rd16(e2 + 6) == 0 {
                    wr16(e2 + 6, (cnt_a & 0xffff) as u16);
                    if cnt_a != 0 {
                        let f = lf_checker_rt::callee_stdcall!(ALLOC_F, u32, cnt_a);
                        wr32(e2, f);
                    } else {
                        wr32(e2, 0);
                    }
                }
                wr16(e2 + 4, (cnt_a & 0xffff) as u16);
                let n2 = rd16(esi + 8);
                let q2 = if n2 > 1 {
                    div_count(n2, rd16(esi + 0x0a))
                } else {
                    1
                };
                let cval = if s2 < q2.wrapping_sub(1) {
                    rd16(esi + 0x0a).wrapping_add(1) & 0xffff
                } else {
                    // The dividend is the stale count n2, not the address:
                    // the load of a_ptr feeds only the divisor read, then
                    // eax is reloaded from cx (which still holds n2).
                    let d = rd16(a_ptr) as i32;
                    let rem = (n2 as i32).wrapping_sub(1) % d;
                    let r = (rem & 0xffff) as u32;
                    if r != 0 {
                        r.wrapping_add(1) & 0xffff
                    } else {
                        rd16(esi + 0x0a).wrapping_add(1) & 0xffff
                    }
                };
                wr16(e2 + 0x0c, cval as u16);
                let mut k = 0u32;
                while k < cnt_a {
                    let src_obj = rd32(rd32(esi + 0x20) + k * 4);
                    let src_row = rd32(src_obj + 8);
                    let dst_obj = rd32(rd32(esi + 0x14) + s2 * 4);
                    let dst_row = rd32(dst_obj);
                    wr32(dst_row + k * 4, rd32(src_row + s2 * 4));
                    k += 1;
                }
                s2 += 1;
            }
        }
        0
    }
});
