// original: 0x00CF9C80 ladder_ray_submit (proposed)
//
// Submits a ladder ray query: packs the target matrix rows (skipping every
// w slot), the owner words and the half-scaled direction into one block and
// hands it to the ray validator.
//
// Arguments (thiscall, ecx=this plus two stack words): `this` (task object;
// word at +0x14), `a0` (owner object; matrix pointer at +0x20), `a1`
// (three-float direction).
//
// Behaviour: builds a 17-word block: the owner pointer, the word at
// this+0x14, matrix rows 0-2 with the w slots skipped (three pattern gaps
// the rewrite leaves zero; never snapped), and the fourth row plus
// direction*0.5. Callee 1 (cdecl/5, ecx ignored) answers the query with the
// block passed as two overlapping pointers plus constants (2, 0xa8, -1).
// Returns 1 when the callee answers nonzero, else 0. Float order is the
// original's, pinned with black_box.
lf_checker_rt::export!(thiscall, rw_00CF9C80(this: u32, a0: u32, a1: u32) -> u32 {
    unsafe {
        const THIS_W_OFF: u32 = 0x14;
        const MAT_PTR_OFF: u32 = 0x20;
        const HALF_ADDR: u32 = 0x00FE8830;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
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

        let half = f32::from_bits((lf_checker_rt::global::<u32>(HALF_ADDR) as *const u32).read_unaligned());
        let mat = rd32(a0 + MAT_PTR_OFF);
        let mut st = [0u32; 17];
        st[0] = a0;
        st[1] = rd32(this + THIS_W_OFF);
        st[2] = rd32(mat);
        st[3] = rd32(mat + 4);
        st[4] = rd32(mat + 8);
        st[6] = rd32(mat + 0x10);
        st[7] = rd32(mat + 0x14);
        st[8] = rd32(mat + 0x18);
        st[10] = rd32(mat + 0x20);
        st[11] = rd32(mat + 0x24);
        st[12] = rd32(mat + 0x28);
        let hx = mul(rdf(a1), half);
        let hy = mul(rdf(a1 + 4), half);
        let hz = mul(rdf(a1 + 8), half);
        st[14] = add(rdf(mat + 0x30), hx).to_bits();
        st[15] = add(rdf(mat + 0x34), hy).to_bits();
        st[16] = add(rdf(mat + 0x38), hz).to_bits();
        let p = st.as_mut_ptr() as u32;
        let ans = lf_checker_rt::callee_cdecl!(1, u32, p + 8, p, 2, 0xa8, 0xFFFF_FFFF);
        ((ans & 0xFF) != 0) as u32
    }
});
