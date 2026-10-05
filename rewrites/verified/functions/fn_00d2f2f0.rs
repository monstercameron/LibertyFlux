// original: 0x00d2f2f0 task_route_endpoint_advance (proposed)

/// Advance the task's endpoint along its route, or re-anchor it from its object.
///
/// `this` points to the task object: dword at `+0x50` is a route handle, and
/// `+0x30..0x40` receives the new endpoint (three floats and a word). `arg`
/// points to an object whose dword at `+0x20` is a second object carrying
/// fallback coordinates. Three callees do the route work: a validity check
/// (cdecl, handle in, nonzero low byte means valid), a route query (cdecl,
/// handle plus four out-words: entry count, float-array pointer, a five-word
/// scratch vector whose last word travels to the endpoint, and an in/out
/// word), and a route release (cdecl, handle in).
///
/// If the handle is invalid the result is 0 and nothing else happens. If the
/// query reports nonzero, the handle is released (set to 0) and the result is
/// 2. Otherwise the count decides: more than one entry extends the last entry
/// by twice the normalized (second-to-last minus last) direction (a zero
/// direction extends by zero, exactly as the original's zero-compare does);
/// exactly one entry (or fewer but nonzero) copies that entry's four words;
/// zero entries sums the fallback object's coordinate pairs. The scratch
/// vector's last word becomes the endpoint's fourth word on the extend and
/// sum paths. The handle is then released and the result is 1. Array entries
/// are 16 bytes; all float operations run in the original's operand order.
/// The frame word the original aligns-and-tags for the query's in/out struct
/// is never read back, so the rewrite passes a plain zero buffer instead.
///
/// Original: 0x00d2f2f0 (thiscall: object in ECX, one stack word, callee pops
/// 4, 0/1/2 in EAX).
lf_checker_rt::export!(thiscall, rw_00d2f2f0(this: u32, arg: u32) -> u32 {
    unsafe {
        const HANDLE: u32 = 0x50;
        const ENDPOINT: u32 = 0x30;
        const OBJ_LINK: u32 = 0x20;
        const ENTRY_STRIDE: i32 = 16;
        const CHECK: u32 = 0;
        const QUERY: u32 = 1;
        const RELEASE: u32 = 2;
        const ONE: f32 = 1.0;
        const TWO: f32 = 2.0;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rdf(a: u32) -> f32 {
            unsafe { f32::from_bits(rd32(a)) }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn wrf(a: u32, v: f32) {
            unsafe { wr32(a, v.to_bits()) }
        }
        #[inline(always)]
        fn sub(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) - core::hint::black_box(b)
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
        fn div(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) / core::hint::black_box(b)
        }
        #[inline(always)]
        fn sqrt(a: f32) -> f32 {
            core::hint::black_box(a).sqrt()
        }

        let valid: u32 = lf_checker_rt::callee_cdecl!(CHECK, u32, rd32(this + HANDLE));
        if valid & 0xff == 0 {
            return 0;
        }
        // Out-words for the query: count, array, five-word vector, in/out.
        let mut obuf = [0u32; 8];
        let qb: u32 = lf_checker_rt::callee_cdecl!(
            QUERY,
            u32,
            rd32(this + HANDLE),
            obuf.as_mut_ptr() as u32,
            obuf.as_mut_ptr().add(1) as u32,
            obuf.as_mut_ptr().add(2) as u32,
            obuf.as_mut_ptr().add(7) as u32
        );
        if qb != 0 {
            let _: u32 = lf_checker_rt::callee_cdecl!(RELEASE, u32, rd32(this + HANDLE));
            wr32(this + HANDLE, 0);
            return 2;
        }
        let n = obuf[0] as i32;
        if n > 1 {
            let arr = obuf[1];
            let e0 = arr.wrapping_add((n.wrapping_sub(2).wrapping_mul(ENTRY_STRIDE)) as u32);
            let e1 = arr.wrapping_add((n.wrapping_sub(1).wrapping_mul(ENTRY_STRIDE)) as u32);
            let dx = sub(rdf(e0), rdf(e1));
            let dy = sub(rdf(e0 + 4), rdf(e1 + 4));
            let dz = sub(rdf(e0 + 8), rdf(e1 + 8));
            let len2 = add(add(mul(dy, dy), mul(dx, dx)), mul(dz, dz));
            let inv = if len2 == 0.0 {
                0.0
            } else {
                div(ONE, sqrt(len2))
            };
            let g0 = rdf(e1);
            let g1 = rdf(e1 + 4);
            let g2 = rdf(e1 + 8);
            wrf(this + ENDPOINT, add(g0, mul(mul(dx, inv), TWO)));
            wrf(this + ENDPOINT + 4, add(g1, mul(mul(dy, inv), TWO)));
            wrf(this + ENDPOINT + 8, add(g2, mul(mul(dz, inv), TWO)));
            wr32(this + ENDPOINT + 12, obuf[6]);
        } else if n == 0 {
            let obj = rd32(arg + OBJ_LINK);
            wrf(this + ENDPOINT, add(rdf(obj + 0x10), rdf(obj + 0x30)));
            wrf(this + ENDPOINT + 4, add(rdf(obj + 0x14), rdf(obj + 0x34)));
            wrf(this + ENDPOINT + 8, add(rdf(obj + 0x18), rdf(obj + 0x38)));
            wr32(this + ENDPOINT + 12, obuf[6]);
        } else {
            let arr = obuf[1];
            let base = arr.wrapping_add((n.wrapping_mul(ENTRY_STRIDE).wrapping_sub(ENTRY_STRIDE)) as u32);
            wr32(this + ENDPOINT, rd32(base));
            wrf(this + ENDPOINT + 4, rdf(base + 4));
            wrf(this + ENDPOINT + 8, rdf(base + 8));
            wr32(this + ENDPOINT + 12, rd32(base + 12));
        }
        let _: u32 = lf_checker_rt::callee_cdecl!(RELEASE, u32, rd32(this + HANDLE));
        wr32(this + HANDLE, 0);
        1
    }
});
