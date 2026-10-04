// original: 0x00cb8310 cover_search_init
/// Initialise a cover search through two probe rounds (4 calls).
///
/// Runs the reset callee on `this` (thiscall, two stack arguments), then
/// the seven-argument first probe with (`a0`, `a1`, 1, 6, 0, `0x8E`, 0)
/// and stores its handle at `[this + 0xAC]`. When the handle is non-null,
/// builds the two 3-word probes `(x, y, z - 2.0)` and `(x, y, z + 2.0)`
/// from the point at `a1` (the `2.0` is the constant from file address
/// `0x00FE8A24`, applied in the original's operand order) and runs the
/// second probe with them plus (1, 6, 0, `0x8E`, 0), storing its handle
/// at `[this + 0xB0]`; a null second handle releases the first and clears
/// `[this + 0xAC]`. Always finishes by copying the four words at `a1`
/// into `[this + 0x90 .. +0x9C]` and returning the fourth. All callees
/// are intercepted by the checker; the two probe addresses are skipped
/// and their words are snapped.
lf_checker_rt::export!(thiscall, rw_00cb8310(this: u32, a0: u32, a1: u32) -> u32 {
    unsafe {
        /// Handle slots for the two probe rounds.
        const H1_OFF: u32 = 0xAC;
        const H2_OFF: u32 = 0xB0;
        /// Destination of the copied point.
        const DST_OFF: u32 = 0x90;
        /// Probe half-height (constant from file address 0x00FE8A24).
        const HALF_H: f32 = 2.0;
        const RESET: u32 = 1;
        const PROBE1: u32 = 2;
        const PROBE2: u32 = 3;
        const RELEASE: u32 = 4;
        #[inline(always)]
        unsafe fn rd(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rdf(a: u32) -> f32 {
            unsafe { f32::from_bits(rd(a)) }
        }
        #[inline(always)]
        unsafe fn wr(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        #[inline(always)]
        fn add(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) + core::hint::black_box(b)
        }
        #[inline(always)]
        fn sub(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) - core::hint::black_box(b)
        }
        let _: u32 = lf_checker_rt::callee_thiscall!(RESET, u32, this);
        let h1: u32 = lf_checker_rt::callee_cdecl!(
            PROBE1, u32, a0, a1, 1, 6, 0, 0x8E, 0);
        wr(this + H1_OFF, h1);
        if h1 != 0 {
            let x = rdf(a1);
            let y = rdf(a1 + 4);
            let z = rdf(a1 + 8);
            let zp = add(z, HALF_H);
            let zm = sub(z, HALF_H);
            let p1 = [x.to_bits(), y.to_bits(), zm.to_bits()];
            let p2 = [x.to_bits(), y.to_bits(), zp.to_bits()];
            let h2: u32 = lf_checker_rt::callee_cdecl!(
                PROBE2, u32, p2.as_ptr() as u32, p1.as_ptr() as u32,
                1, 6, 0, 0x8E, 0);
            wr(this + H2_OFF, h2);
            if h2 == 0 {
                let ha = rd(this + H1_OFF);
                let _: u32 = lf_checker_rt::callee_cdecl!(RELEASE, u32, ha);
                wr(this + H1_OFF, 0);
            }
        }
        let w0 = rd(a1);
        let w3 = rd(a1 + 12);
        wr(this + DST_OFF, w0);
        wr(this + DST_OFF + 4, rd(a1 + 4));
        wr(this + DST_OFF + 8, rd(a1 + 8));
        wr(this + DST_OFF + 12, w3);
        w3
    }
});
