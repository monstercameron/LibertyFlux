// original: 0x00924e50 view_row_rebuild_scaled (proposed)

/// Rebuild view-row `idx` from its stored scale, then run the row pipeline.
///
/// `idx` selects one 0x110-byte row at `0x119F110 + idx*0x110`; the first
/// stack word is ignored. With `t0` the row's word +0x00, `b0/b1/b2` the
/// words +0xB0/+0xB4/+0xB8, `s = 1/sqrt(1.0)` from the image double at
/// 0xFE89E0 (i.e. 1.0), `k = s * -0.0` (image constant at 0xFE8D1C) and
/// `m = s * -1.0` (image constant at 0xFE8D94), the row is rewritten as:
/// +0x34/+0x50 = s; +0x30/+0x38/+0x40/+0x44/+0x54/+0x58 = k; +0x48 = m;
/// +0x60 = s*t0 + b0; +0x64 = k*t0 + b1; +0x68 = k*t0 + b2. The original then
/// computes three more values (a quadratic form in k, and two scaled sums)
/// that only reach stack slots overwritten before any read; the rewrite does
/// not compute them. Three pipeline calls follow, all with frame-pointer
/// arguments: the combiner (callee 1, cdecl, two struct pointers), the
/// solver (callee 2, thiscall, struct pointers plus the identity matrix read
/// from image blocks 0xFE8E30/0xFE8E40), and the row writer (callee 3,
/// thiscall on the row address +0x70 with one struct pointer). Finally word
/// +0xD0 is copied to +0x04 and returned. Float order is the original's,
/// pinned through black-boxed operands.
///
/// Original: 0x00924e50 (cdecl, two stack words, plain `ret`).
lf_checker_rt::export!(cdecl, rw_00924e50(_ignored: u32, idx: u32) -> u32 {
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
        let t0 = rdf(row);
        let r3 = add(mul(s, t0), rdf(row + 0xB0));
        let r5 = add(mul(k, t0), rdf(row + 0xB4));
        let r4 = add(mul(k, t0), rdf(row + 0xB8));
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
        // Pipeline structs: the solver's first struct is the identity matrix
        // (snapped by the contract); every other struct pointer is skipped,
        // so plain scratch stands in for them.
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
