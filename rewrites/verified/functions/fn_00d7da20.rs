// original: 0x00d7da20 node_link_frame_query (proposed)

/// Query two linked node tables and write two 4-float frames.
///
/// `a`, `b`, `c` are composite ids: the low 16 bits select a row base from
/// the node table, the high 16 bits select a 32-byte node inside that base.
/// Each node carries three signed 16-bit coordinates at `+0x14`/`+0x16`
/// (scaled by 0.125) and `+0x18` (scaled by 0.015625). A link callee maps an
/// id pair to a link index; the link table holds 8-byte rows whose bytes at
/// `+5` (two 3-bit fields) and `+6` (low nibble, top bit) steer blending.
///
/// Main path (`c` valid): two direction vectors between node positions are
/// normalised (a zero-length vector normalises to zero, not NaN), passed
/// through a 3-vector callee twice, combined with a clamped dot product, and
/// mixed with a second callee's two output words into `out1` (16 bytes) and
/// `out2` (16 bytes). When `c` is absent (`c & 0xffff == 0xffff`) or its node
/// base is null, a shorter fall-back path uses only the `a`/`b` direction and
/// the link flags of `a`. Both paths copy one uninitialised stack word into
/// the fourth float of each frame (zero under the checker's defined fill).
/// Returns `out2`.
///
/// Original: 0x00d7da20 (cdecl, five stack words).
lf_checker_rt::export!(cdecl, rw_00d7da20(a: u32, b: u32, c: u32, out1: u32, out2: u32) -> u32 {
    unsafe {
        const NODE_TABLE: u32 = 0x1178284;
        const LINK_TABLE: u32 = 0x1178384;
        const GRAPH_OBJ: u32 = 0x1177a80;
        const NODE_STRIDE: u32 = 32;
        const LINK_STRIDE: u32 = 8;
        const K_POS: u32 = 0xfe87a4; // 0.125
        const K_Z: u32 = 0xfe8720; // 0.015625
        const K_TWO: u32 = 0xfe8a24; // 2.0
        const K_ZERO: u32 = 0xfe8628; // 0.0
        const K_ONE: u32 = 0xfe88e8; // 1.0
        const SIGN_BIT: u32 = 0x8000_0000;

        #[inline(always)]
        unsafe fn rd32(addr: u32) -> u32 {
            unsafe { (addr as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rd16(addr: u32) -> u16 {
            unsafe { (addr as *const u16).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rd8(addr: u32) -> u8 {
            unsafe { (addr as *const u8).read() }
        }
        #[inline(always)]
        unsafe fn rdf(addr: u32) -> f32 {
            unsafe { f32::from_bits(rd32(addr)) }
        }
        #[inline(always)]
        unsafe fn wrf(addr: u32, v: f32) {
            unsafe { (addr as *mut u32).write_unaligned(v.to_bits()) }
        }
        #[inline(always)]
        unsafe fn gf(va: u32) -> f32 {
            unsafe { f32::from_bits(rd32(lf_checker_rt::relocated(va))) }
        }
        #[inline(always)]
        fn mul(x: f32, y: f32) -> f32 {
            core::hint::black_box(x) * core::hint::black_box(y)
        }
        #[inline(always)]
        fn add(x: f32, y: f32) -> f32 {
            core::hint::black_box(x) + core::hint::black_box(y)
        }
        #[inline(always)]
        fn sub(x: f32, y: f32) -> f32 {
            core::hint::black_box(x) - core::hint::black_box(y)
        }
        #[inline(always)]
        fn neg_bits(x: f32) -> f32 {
            f32::from_bits(x.to_bits() ^ SIGN_BIT)
        }
        /// 1/sqrt(len2), or 0 when len2 is zero (the original's
        /// ucomiss/lahf/test/jp idiom: only an ordered-equal zero takes the
        /// zero path; NaN flows through the division).
        #[inline(always)]
        fn inv_len(len2: f32, one: f32) -> f32 {
            if len2 == 0.0 {
                0.0
            } else {
                core::hint::black_box(one) / core::hint::black_box(len2.sqrt())
            }
        }
        #[inline(always)]
        unsafe fn node_base(id_lo: u32) -> u32 {
            unsafe { rd32(lf_checker_rt::relocated(NODE_TABLE) + id_lo * 4) }
        }
        #[inline(always)]
        unsafe fn link_base(id_lo: u32) -> u32 {
            unsafe { rd32(lf_checker_rt::relocated(LINK_TABLE) + id_lo * 4) }
        }

        let k_pos = gf(K_POS);
        // Node rows for a and b, read in the original's order.
        let s1 = node_base(a & 0xffff).wrapping_add((a >> 16) * NODE_STRIDE);
        let d1 = node_base(b & 0xffff).wrapping_add((b >> 16) * NODE_STRIDE);
        let s1_x = rd16(s1 + 0x14);
        let s1_y = rd16(s1 + 0x16);
        let p1x = mul((s1_x as i16 as i32) as f32, k_pos);
        let d1_x = rd16(d1 + 0x14);
        let p1y = mul((s1_y as i16 as i32) as f32, k_pos);
        let d1_y = rd16(d1 + 0x16);
        let q1x = mul((d1_x as i16 as i32) as f32, k_pos);
        let d1_z = rd16(d1 + 0x18);
        let q1y = mul((d1_y as i16 as i32) as f32, k_pos);
        let r1 = mul((d1_z as i16 as i32) as f32, gf(K_Z));

        let graph = lf_checker_rt::relocated(GRAPH_OBJ);
        let idx1: u32 = lf_checker_rt::callee_thiscall!(1, u32, graph, a, b);
        let link_a = link_base(a & 0xffff);
        let flag_a6 = rd8(link_a + idx1.wrapping_mul(LINK_STRIDE) + 6);
        let top_bit = (flag_a6 >> 7) as u32;

        let c_lo = c & 0xffff;
        let c_base = if c_lo == 0xffff {
            0
        } else {
            node_base(c_lo)
        };
        if c_lo == 0xffff || c_base == 0 {
            // Fall-back path: only the a/b direction and a's link flags.
            let flag_a5 = rd8(link_a + idx1.wrapping_mul(LINK_STRIDE) + 5);
            let dx = sub(q1x, p1x);
            let dy = sub(q1y, p1y);
            let len2 = add(mul(dx, dx), mul(dy, dy));
            let inv = inv_len(len2, gf(K_ONE));
            let nx = mul(inv, dx);
            let ny = mul(inv, dy);
            let f4 = ((flag_a6 & 0xf) as i32) as f32;
            let mut w3 = 0.0f32;
            let mut w4 = 0.0f32;
            let _ret: u32 = lf_checker_rt::callee_cdecl!(
                3,
                u32,
                (flag_a5 & 7) as u32,
                ((flag_a5 >> 3) & 7) as u32,
                f4.to_bits(),
                &mut w3 as *mut f32 as u32,
                &mut w4 as *mut f32 as u32,
                top_bit
            );
            let two = gf(K_TWO);
            let zero = gf(K_ZERO);
            let m3 = sub(w3, two);
            let m4 = sub(w4, two);
            let nnx = neg_bits(nx);
            let t0 = mul(ny, m3);
            let o0 = sub(q1x, t0);
            let t1 = mul(m3, nnx);
            let o1 = sub(q1y, t1);
            let o2 = sub(r1, mul(m3, zero));
            let t7 = add(mul(ny, m4), q1x);
            let o4x = add(mul(m4, nnx), q1y);
            wrf(out1 + 8, o2);
            wrf(out1, o0);
            wrf(out1 + 4, o1);
            wrf(out1 + 12, uninit_word());
            wrf(out2, t7);
            wrf(out2 + 4, o4x);
            wrf(out2 + 12, uninit_word());
            wrf(out2 + 8, add(mul(m4, 0.0), r1));
            return out2;
        }

        // Main path.
        let s3 = c_base.wrapping_add((c >> 16) * NODE_STRIDE);
        let s3_x = rd16(s3 + 0x14);
        let s3_y = rd16(s3 + 0x16);
        let p3x = mul((s3_x as i16 as i32) as f32, k_pos);
        let p3y = mul((s3_y as i16 as i32) as f32, k_pos);
        let idx2: u32 = lf_checker_rt::callee_thiscall!(1, u32, graph, b, c);
        let link_b = link_base(b & 0xffff);
        let flag_b5 = rd8(link_b + idx2.wrapping_mul(LINK_STRIDE) + 5);
        let link_a2 = link_base(a & 0xffff);
        let flag_a5 = rd8(link_a2 + idx1.wrapping_mul(LINK_STRIDE) + 5);
        let lo_min = ((flag_b5 & 7) as u32).min((flag_a5 & 7) as u32);
        let hi_min = (((flag_b5 >> 3) & 7) as u32).min(((flag_a5 >> 3) & 7) as u32);
        let fa = ((rd8(link_a2 + idx1.wrapping_mul(LINK_STRIDE) + 6) & 0xf) as i32) as f32;
        let fb = ((rd8(link_b + idx2.wrapping_mul(LINK_STRIDE) + 6) & 0xf) as i32) as f32;
        let fmin = if fb > fa { fa } else { fb };

        let one = gf(K_ONE);
        let dx1 = sub(q1x, p1x);
        let dy1 = sub(q1y, p1y);
        let inv1 = inv_len(add(mul(dx1, dx1), mul(dy1, dy1)), one);
        let ndy1 = mul(dy1, inv1);
        let ndx1 = mul(dx1, inv1);
        let dx2 = sub(p3x, q1x);
        let dy2 = sub(p3y, q1y);
        let inv2 = inv_len(add(mul(dx2, dx2), mul(dy2, dy2)), one);
        let ndx2 = mul(inv2, dx2);
        let ndy2 = mul(inv2, dy2);

        let mut buf1 = [0.0f32; 3];
        let mut buf2 = [0.0f32; 3];
        let _r1: u32 = lf_checker_rt::callee_thiscall!(
            2,
            u32,
            buf1.as_mut_ptr() as u32,
            (&ndx2 as *const f32) as u32,
            1
        );
        let _r2: u32 = lf_checker_rt::callee_thiscall!(
            2,
            u32,
            buf2.as_mut_ptr() as u32,
            (&ndx1 as *const f32) as u32,
            1
        );
        // Dot product in the original's order: (b2[0]*b1[0] + b2[1]*b1[1]) + b2[2]*b1[2].
        let mut dot = mul(buf2[0], buf1[0]);
        dot = add(dot, mul(buf2[1], buf1[1]));
        dot = add(dot, mul(buf2[2], buf1[2]));
        if dot < 0.0 {
            dot = 0.0;
        }
        let comb1 = add(ndx1, sub(ndx2, mul(ndx2, dot)));
        let comb2 = add(ndy1, sub(ndy2, mul(ndy2, dot)));

        let mut w3 = 0.0f32;
        let mut w4 = 0.0f32;
        let _r3: u32 = lf_checker_rt::callee_cdecl!(
            3,
            u32,
            lo_min,
            hi_min,
            fmin.to_bits(),
            &mut w3 as *mut f32 as u32,
            &mut w4 as *mut f32 as u32,
            top_bit
        );
        let two = gf(K_TWO);
        let zero = gf(K_ZERO);
        let m3 = sub(w3, two);
        let m4 = sub(w4, two);
        let ncomb1 = neg_bits(comb1);
        let o0 = sub(q1x, mul(comb2, m3));
        let o1 = sub(q1y, mul(ncomb1, m3));
        let o2 = sub(r1, mul(m3, zero));
        let g6 = add(mul(comb2, m4), q1x);
        let g7 = add(mul(ncomb1, m4), q1y);
        wrf(out1 + 8, o2);
        wrf(out1, o0);
        wrf(out1 + 4, o1);
        wrf(out1 + 12, uninit_word());
        wrf(out2, g6);
        wrf(out2 + 4, g7);
        wrf(out2 + 12, uninit_word());
        wrf(out2 + 8, add(mul(m4, zero), r1));
        out2
    }
});

/// The original copies one stack word it never wrote into the fourth float of
/// each frame. That word is whatever the stack fill is (zero under this
/// contract's defined fill); a rewrite cannot read "its own" equivalent slot
/// because its frame holds its own leftovers there, so it emits the fill
/// value the original observably reads.
#[inline(always)]
fn uninit_word() -> f32 {
    0.0
}
