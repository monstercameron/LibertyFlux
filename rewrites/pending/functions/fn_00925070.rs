// original: 0x00925070 view_row_rebuild_zero (proposed)

/// Rebuild view-row `idx` from a zero scale, then run the row pipeline.
///
/// Twin of 0x00924e50: identical except the row's word +0x00 is not read and
/// the scale is +0.0, so with `b0/b1/b2` the row words +0xB0/+0xB4/+0xB8,
/// `s = 1/sqrt(1.0)` (1.0), `k = s * -0.0` and `m = s * -1.0` the row is
/// rewritten as: +0x34/+0x50 = s; +0x30/+0x38/+0x40/+0x44/+0x54/+0x58 = k;
/// +0x48 = m; +0x60 = s*0 + b0; +0x64 = k*0 + b1; +0x68 = k*0 + b2. (The
/// products are still evaluated: s*0 is +0.0 but k*0 with k = -0.0 is -0.0,
/// and an infinite s would make them NaN.) As in the twin, three further
/// values the original computes only reach overwritten stack slots and are
/// not computed here. Same three pipeline calls (combiner, solver with the
/// identity matrix, row writer), same +0xD0 to +0x04 copy and return. Float
/// order is the original's, pinned through black-boxed operands.
///
/// Original: 0x00925070 (cdecl, two stack words, plain `ret`).
lf_checker_rt::export!(cdecl, rw_00925070(_ignored: u32, idx: u32) -> u32 {
    unsafe {
        #[inline(always)]
        unsafe fn rdf(a: u32) -> f32 {
            unsafe { (lf_checker_rt::global::<u32>(a) as *const f32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wrf(a: u32, v: f32) {
            unsafe { (lf_checker_rt::global::<u32>(a) as *mut f32).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { lf_checker_rt::global::<u32>(a).read_unaligned() }
        }
        #[inline(always)]
        fn div(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) / core::hint::black_box(b)
        }
        #[inline(always)]
        fn mul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }
        #[inline(always)]
        fn add(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) + core::hint::black_box(b)
        }

        let row = 0x119F110u32.wrapping_add(idx.wrapping_mul(0x110));
        let dbl = (lf_checker_rt::global::<u64>(0x00FE89E0) as *const f64).read_unaligned();
        let s = div(rdf(0x00FE88E8), dbl.sqrt() as f32);
        let k = mul(s, rdf(0x00FE8D1C));
        let r3 = add(mul(s, 0.0), rdf(row + 0xB0));
        let r5 = add(mul(k, 0.0), rdf(row + 0xB4));
        let r4 = add(mul(k, 0.0), rdf(row + 0xB8));
        wrf(row + 0x30, k);
        wrf(row + 0x38, k);
        wrf(row + 0x34, s);
        let m = mul(s, rdf(0x00FE8D94));
        wrf(row + 0x40, k);
        wrf(row + 0x44, k);
        wrf(row + 0x48, m);
        wrf(row + 0x54, k);
        wrf(row + 0x58, k);
        wrf(row + 0x50, s);
        wrf(row + 0x64, r5);
        wrf(row + 0x60, r3);
        wrf(row + 0x68, r4);
        let mut ident = [0u32; 8];
        (ident.as_mut_ptr() as *mut [u32; 4]).write_unaligned(
            (lf_checker_rt::relocated(0x00FE8E30) as *const [u32; 4]).read_unaligned(),
        );
        (ident.as_mut_ptr().add(4) as *mut [u32; 4]).write_unaligned(
            (lf_checker_rt::relocated(0x00FE8E40) as *const [u32; 4]).read_unaligned(),
        );
        let mut scratch = [0u32; 8];
        let p_ident = ident.as_mut_ptr() as u32;
        let p_dummy = scratch.as_mut_ptr() as u32;
        lf_checker_rt::callee_cdecl!(1, u32, p_dummy, p_ident);
        lf_checker_rt::callee_thiscall!(2, u32, p_dummy, p_ident, p_dummy);
        lf_checker_rt::callee_thiscall!(3, u32, lf_checker_rt::relocated(row.wrapping_add(0x70)), p_dummy);
        let tail = rd32(row + 0xD0);
        lf_checker_rt::global::<u32>(row + 0x04).write_unaligned(tail);
        tail
    }
});
