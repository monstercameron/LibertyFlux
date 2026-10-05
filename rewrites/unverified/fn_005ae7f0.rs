// original: 0x005ae7f0 hud_text_list_update (proposed)

/// Refresh the HUD text attribute block and walk the text entry table.
///
/// `this` points to the HUD text object (flag byte at `+2`, base scale
/// float at `+4`). Returns `this` at once when the flag byte is clear or
/// the disable byte is set; otherwise programs the attribute setters,
/// reduces the per-entry scales to a maximum, folds the pad/colour state
/// through three scripted indirect calls, commits the shared block, and
/// then visits each of the `bound` table entries (entry `i` at
/// `TABLE + 0x48 * i`: index at `+0`, colour at `+0x40`, flag at `+0x44`).
/// An entry is skipped for index `-1` or a null table link; otherwise one
/// of three blocks (A: indexed colour range, B: fallback, C: flag clear)
/// runs four more indirect calls and commits a four-float object, and the
/// tail commits the entry unless one of two ordered float gates skips it.
/// A nonzero reentry counter is decremented and returned instead of
/// walking the table. All index/bound compares are signed (`jl`/`jle`),
/// but every steering value that can separate signed from unsigned takes
/// the same road either way, so no trial separates them; the cmove
/// selections after the indirect calls are equality compares. Float order
/// is pinned with `black_box` throughout.
///
/// Original: 0x005ae7f0 (thiscall, `this` in ecx, no stack words).
#[allow(clippy::too_many_lines)]
lf_checker_rt::export!(thiscall, rw_005ae7f0(this: u32) -> u32 {
    unsafe {
        const FLAG_OFF: u32 = 2;
        const THIS_SCALE: u32 = 4;
        const DISABLE: u32 = 0x011E622F;
        const MODE_A: u32 = 0x0116C250;
        const MODE_B: u32 = 0x0116C253;
        const MODE_VALUE: u8 = 0x6A;
        const G15D8: u32 = 0x011615D8;
        const G11D: u32 = 0x011D6FD4;
        const BOUND: u32 = 0x0118F4C0;
        const REENTRY: u32 = 0x0118F4C4;
        const TABLE: u32 = 0x01191270;
        const TABLE1: u32 = 0x01191274;
        const LINKS: u32 = 0x0118F4F0;
        const ENTRY_STRIDE: u32 = 0x48;
        const CVEED: u32 = 0x0110DD14;
        const BASE0: u32 = 0x0105C880;
        const ALT0: u32 = 0x0105C87C;
        const BASE1: u32 = 0x0105C884;
        const ALT1: u32 = 0x0105C888;
        const INDIRECT_SLOT: u32 = 0x00E731AC;
        const G870C: u32 = 0x00FE870C;
        const G8BE8: u32 = 0x00FE8BE8;
        const G86EC: u32 = 0x00FE86EC;
        const G8830: u32 = 0x00FE8830;
        const G8A24: u32 = 0x00FE8A24;
        const NEG1: u32 = 0xFFFF_FFFF;

        #[inline(always)]
        unsafe fn rd8(a: u32) -> u8 {
            unsafe { (lf_checker_rt::relocated(a) as *const u8).read() }
        }
        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (lf_checker_rt::relocated(a) as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rdf(a: u32) -> f32 {
            unsafe { f32::from_bits(rd32(a)) }
        }
        #[inline(always)]
        fn mul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }
        #[inline(always)]
        fn add(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) + core::hint::black_box(b)
        }
        #[inline(always)]
        fn sub(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) - core::hint::black_box(b)
        }
        fn cvt_top(x: f32) -> u32 {
            let v: u32 = if x.is_nan() || x >= 2147483648.0 || x < -2147483648.0 {
                0x8000_0000
            } else {
                (x as i32) as u32
            };
            (v & 0xFF) << 24
        }
        #[inline(always)]
        unsafe fn ind() -> u32 {
            unsafe {
                let tgt = (lf_checker_rt::relocated(INDIRECT_SLOT) as *const u32).read();
                let f: extern "cdecl" fn() -> u32 = core::mem::transmute(tgt as usize);
                f()
            }
        }

        if ((this.wrapping_add(FLAG_OFF)) as *const u8).read() == 0 {
            return this;
        }
        if rd8(DISABLE) != 0 {
            return this;
        }
        let mut bl: u8 = 0xFF;
        if rd8(G15D8) != 0 {
            let r: u32 = lf_checker_rt::callee_thiscall!(1, u32, lf_checker_rt::relocated(G15D8));
            bl = (r & 0xFF) as u8;
        }
        lf_checker_rt::callee_cdecl!(2, u32,);
        let g1 = rd8(MODE_A);
        let g2 = rd8(MODE_B);
        let mode: u32 = if g1 == MODE_VALUE || g2 != 0 { 2 } else { 7 };
        lf_checker_rt::callee_cdecl!(3, u32, mode);
        lf_checker_rt::callee_cdecl!(4, u32, 0);
        lf_checker_rt::callee_cdecl!(5, u32, 1);

        let id40: u32 = if rd32(G11D) == 2 { 0x40 } else { 0x41 };
        let mut s136 = [0u32; 1];
        let h84: u32 =
            lf_checker_rt::callee_cdecl!(6, u32, s136.as_mut_ptr() as u32, id40, bl as u32);
        lf_checker_rt::callee_cdecl!(7, u32, (h84 as *const u32).read_unaligned());

        let mut s112 = [0u32; 2];
        lf_checker_rt::callee_cdecl!(8, u32, s112.as_mut_ptr() as u32, 0x48);
        let mut s96 = [0u32; 2];
        lf_checker_rt::callee_cdecl!(9, u32, s96.as_mut_ptr() as u32, 0x49);
        let mut s144 = [0u32; 2];
        lf_checker_rt::callee_cdecl!(10, u32, s144.as_mut_ptr() as u32, 0x4A);
        lf_checker_rt::callee_cdecl!(11, u32, 2, s112.as_mut_ptr() as u32, s96.as_mut_ptr() as u32, 0);
        lf_checker_rt::callee_cdecl!(12, u32, 2, 0, s144.as_mut_ptr() as u32, 0);
        lf_checker_rt::callee_cdecl!(13, u32, s96[0], s96[1]);
        lf_checker_rt::callee_cdecl!(14, u32, 0, 0x3F800000);
        lf_checker_rt::callee_cdecl!(15, u32, 1);

        let bound = rd32(BOUND);
        let mut m = 0.0f32;
        {
            let mut i = 0u32;
            while i < bound {
                let p = lf_checker_rt::relocated(TABLE1).wrapping_add(i.wrapping_mul(ENTRY_STRIDE));
                let st: f32 = lf_checker_rt::callee_cdecl!(16, f32, p, 1);
                if st > m {
                    m = st;
                }
                i = i.wrapping_add(1);
            }
        }
        let v0pre = add(m, rdf(G870C));

        let blval = bl as u32;
        let mut f = bl as f32;
        let g = rdf(G8BE8);
        if !(g > f) {
            f = g;
        }
        let c0 = cvt_top(f);

        let cveed = rd32(CVEED);
        let b0 = rd32(BASE0);
        let a0 = rd32(ALT0);
        let b1 = rd32(BASE1);
        let a1 = rd32(ALT1);
        let r1 = ind();
        let ebx = if r1 == cveed { a0 } else { b0 };
        let r2 = ind();
        let edi = if r2 == cveed { a1 } else { b1 };
        let r3 = ind();
        let esi = if r3 == cveed { a0 } else { b0 };

        let mut a_obj = [0u32; 4];
        let ha: u32 = lf_checker_rt::callee_cdecl!(17, u32, a_obj.as_mut_ptr() as u32, 0);
        let t148a = mul(esi as f32, f32::from_bits((ha as *const u32).read_unaligned()));
        let mut dead1 = [0u32; 2];
        let hb: u32 = lf_checker_rt::callee_cdecl!(18, u32, dead1.as_mut_ptr() as u32, 0x4B);
        let mut x1 = f32::from_bits(s144[0]);
        x1 = add(x1, v0pre);
        x1 = add(x1, f32::from_bits((hb as *const u32).read_unaligned()));
        x1 = add(x1, f32::from_bits(s112[0]));
        x1 = add(x1, rdf(G86EC));
        x1 = mul(x1, edi as f32);
        let t156 = x1;
        let mut dead2 = [0u32; 2];
        let hc: u32 = lf_checker_rt::callee_cdecl!(19, u32, dead2.as_mut_ptr() as u32, 0x16);
        let x0f = mul(ebx as f32, f32::from_bits((hc as *const u32).read_unaligned()));
        let b88carry = [0u32, x0f.to_bits()];
        let b88 = [0u32, x0f.to_bits(), t156.to_bits(), t148a.to_bits()];
        let c0cell = [c0];
        lf_checker_rt::callee_cdecl!(20, u32, b88.as_ptr() as u32, c0cell.as_ptr() as u32);

        let re = rd32(REENTRY);
        if re != 0 {
            let w = re.wrapping_sub(1);
            (lf_checker_rt::relocated(REENTRY) as *mut u32).write_unaligned(w);
            return w;
        }

        a_obj[2] = s112[0];
        let fthis = f32::from_bits(((this.wrapping_add(THIS_SCALE)) as *const u32).read_unaligned());
        let s112w1 = f32::from_bits(s112[1]);
        let g8830 = rdf(G8830);
        let g8a24 = rdf(G8A24);
        let table = lf_checker_rt::relocated(TABLE);
        let links = lf_checker_rt::relocated(LINKS);
        let g11d = rd32(G11D);
        let mut i = 0u32;
        while i < bound {
            let ebp = table.wrapping_add(i.wrapping_mul(ENTRY_STRIDE));
            let e0 = (ebp as *const u32).read_unaligned();
            if e0 == NEG1 {
                i = i.wrapping_add(1);
                continue;
            }
            let link = ((links.wrapping_add(e0.wrapping_mul(4))) as *const u32).read_unaligned();
            if link == 0 {
                i = i.wrapping_add(1);
                continue;
            }
            lf_checker_rt::callee_thiscall!(21, u32, link);
            let mut dead3 = [0u32; 2];
            let hg: u32 = lf_checker_rt::callee_cdecl!(22, u32, dead3.as_mut_ptr() as u32, 0x4B);
            let hg4 = f32::from_bits(((hg.wrapping_add(4)) as *const u32).read_unaligned());
            let mut vv = mul((i.wrapping_add(1)) as f32, hg4);
            vv = add(vv, sub(s112w1, fthis));
            let colorword = (ebp.wrapping_add(0x40) as *const u32).read_unaligned();
            let color_i = (colorword & 0x00FF_FFFF) | (blval << 24);
            let flag = ((ebp.wrapping_add(0x44)) as *const u8).read();
            // Signed range steering (see doc comment: no separating trial exists).
            let e0i = e0 as i32;
            let in_a = if flag == 0 {
                false
            } else if e0i >= 9 && e0i <= 0x12 {
                true
            } else if g11d == 0 {
                false
            } else if e0i >= 0x73 && e0i <= 0x75 {
                true
            } else {
                e0 == 0x80
            };
            if in_a {
                // Block A stamps (bl << 24) | 0xE1E1E1 over the shared slot
                // before its indirect calls; the commit observes it.
                a_obj[0] = (blval << 24) | 0xE1E1E1;
            }
            let r4 = ind();
            let bx = if r4 == cveed { a0 } else { b0 };
            let r5 = ind();
            let di = if r5 == cveed { a1 } else { b1 };
            let r6 = ind();
            let si = if r6 == cveed { a0 } else { b0 };
            let r7 = ind();
            let cx = if r7 == cveed { a1 } else { b1 };
            if flag == 0 {
                // Block C.
                let x5 = f32::from_bits(s112[0]);
                let mut y1 = f32::from_bits(s144[1]);
                y1 = add(y1, vv);
                let y0 = mul(cx as f32, x5);
                let obj = &mut [0u32; 4];
                obj[0] = y0.to_bits();
                y1 = mul(y1, si as f32);
                let mut y1d = x5;
                y1d = add(y1d, f32::from_bits(s144[0]));
                y1d = mul(y1d, di as f32);
                let mut y0d = bx as f32;
                obj[3] = y1.to_bits();
                obj[2] = y1d.to_bits();
                y0d = mul(y0d, vv);
                obj[1] = y0d.to_bits();
                let cc = [color_i];
                lf_checker_rt::callee_cdecl!(23, u32, obj.as_ptr() as u32, cc.as_ptr() as u32);
            } else {
                // Blocks A and B share the arithmetic; only the object differs.
                let mut x3 = f32::from_bits(s144[0]);
                let mut x0 = f32::from_bits(s112[0]);
                let mut x2 = x3;
                x2 = mul(x2, g8830);
                x3 = mul(x3, g8a24);
                x0 = sub(x0, x2);
                let mut y1 = cx as f32;
                x3 = sub(x3, x2);
                y1 = mul(y1, x0);
                let x0b = si as f32;
                x3 = add(x3, f32::from_bits(s112[0]));
                let mut y1b = f32::from_bits(s144[1]);
                y1b = add(y1b, vv);
                y1b = mul(y1b, x0b);
                let x0c = di as f32;
                x3 = mul(x3, x0c);
                let mut y0d = bx as f32;
                y0d = mul(y0d, vv);
                if in_a {
                    let obj = [y1.to_bits(), y0d.to_bits(), x3.to_bits(), y1b.to_bits()];
                    lf_checker_rt::callee_cdecl!(23, u32, obj.as_ptr() as u32, a_obj.as_ptr() as u32);
                } else {
                    let obj = [y1.to_bits(), y0d.to_bits(), x3.to_bits(), y1b.to_bits()];
                    let bobj = [color_i, s144[0], s144[1], a_obj[0]];
                    lf_checker_rt::callee_cdecl!(23, u32, obj.as_ptr() as u32, bobj.as_ptr() as u32);
                }
            }
            let mut b16 = [0u32; 2];
            let hd: u32 = lf_checker_rt::callee_cdecl!(24, u32, b16.as_mut_ptr() as u32, 0);
            let st0b: f32 = lf_checker_rt::callee_cdecl!(25, f32,);
            a_obj[0] = st0b.to_bits();
            let x0t = sub(f32::from_bits((hd as *const u32).read_unaligned()), st0b);
            let fire1 = vv >= x0t;
            if !fire1 {
                i = i.wrapping_add(1);
                continue;
            }
            let mut b8 = [0u32; 2];
            let he: u32 = lf_checker_rt::callee_cdecl!(26, u32, b8.as_mut_ptr() as u32, 0x16);
            let he0 = f32::from_bits((he as *const u32).read_unaligned());
            if !(he0 >= vv) {
                i = i.wrapping_add(1);
                continue;
            }
            let hf: u32 = lf_checker_rt::callee_cdecl!(27, u32, b88carry.as_ptr() as u32, 0x4B);
            let mut f0 = f32::from_bits(s112[0]);
            f0 = add(f0, f32::from_bits(s144[0]));
            f0 = add(f0, f32::from_bits((hf as *const u32).read_unaligned()));
            lf_checker_rt::callee_cdecl!(28, u32, f0.to_bits(), vv.to_bits(), ebp.wrapping_add(4), NEG1, NEG1);
            i = i.wrapping_add(1);
        }

        lf_checker_rt::callee_cdecl!(15, u32, 0);
        lf_checker_rt::callee_cdecl!(2, u32,)
    }
});
