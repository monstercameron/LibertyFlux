// original: 0x009F3E90 CPlayerPed::vf54

/// Rebuild a player ped's movement blend and hand it to its motion sink.
///
/// `this` is the player ped. Callee 1 (no arguments) decides which of two
/// paths runs:
///
/// - Slow path: hand the matrix row at `+MATRIX` plus `ROW_TAIL` straight
///   to the sink object at `+SINK` through its slot `+SINK_SLOT`, with a
///   zero tag alongside.
/// - Fast path: resolve the active driver through the virtual slot
///   `+VT_DRIVER` (called once to test, and on success a second time to
///   obtain the driver, whose own slot `+VT_FINISH` yields the tag source;
///   when the first call answers null the fallback object at `+FALLBACK`
///   is used instead). The tag word (`+TAG`) goes to callee 4 with a zero
///   tag, and its answer seeds callee 5, which returns the coefficient
///   block (`+COEF_A`, `+COEF_B`).
///
///   Three output floats are then accumulated from the matrix at `+MATRIX`
///   in the original's exact operation order: each is
///   `((row_a * coef_a + row_b * 0) + row_c * coef_b) + row_d`, where the
///   `* 0` terms still run (they poison the sum when their row word is a
///   NaN). The fourth word is NOT computed: the original reads it from an
///   uninitialized stack scratch slot, so under the checker's defined
///   stack fill of zero it is `0.0`, which is what this rewrite stores.
///   The four words are passed by pointer, with a zero tag, to the same
///   sink slot as the slow path.
///
/// Returns nothing; the original is void (thiscall, no stack arguments).
lf_checker_rt::export!(thiscall, rw_009F3E90(this: u32) -> u32 {
    unsafe {
        const MATRIX: u32 = 0x20;
        const SINK: u32 = 0x80;
        const SINK_SLOT: u32 = 0x04;
        const ROW_TAIL: u32 = 0x30;
        const VT_DRIVER: u32 = 0xa0;
        const VT_FINISH: u32 = 0xe0;
        const FALLBACK: u32 = 0x100;
        const TAG: u32 = 0x04;
        const COEF_A: u32 = 0x34;
        const COEF_B: u32 = 0x38;
        const CAL_GATE: u32 = 1;
        const CAL_TAG: u32 = 4;
        const CAL_COEF: u32 = 5;

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
        #[inline(always)]
        unsafe fn sink_call(sink_field: u32, arg0: u32, arg1: u32) {
            unsafe {
                let obj = rd32(sink_field);
                let slot = rd32(obj.wrapping_add(SINK_SLOT));
                let f: extern "thiscall" fn(u32, u32, u32) -> u32 =
                    core::mem::transmute(slot as usize);
                f(sink_field, arg0, arg1);
            }
        }

        let gate = lf_checker_rt::callee_thiscall!(CAL_GATE, u32, this);
        if (gate as u8) == 0 {
            let m = rd32(this.wrapping_add(MATRIX));
            sink_call(this.wrapping_add(SINK), m.wrapping_add(ROW_TAIL), 0);
            return 0;
        }

        let driver_slot = rd32(rd32(this).wrapping_add(VT_DRIVER));
        let driver: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(driver_slot as usize);
        let first = driver(this);
        let tagged = if first == 0 {
            rd32(this.wrapping_add(FALLBACK))
        } else {
            let q = driver(this);
            let fin_slot = rd32(rd32(q).wrapping_add(VT_FINISH));
            let finish: extern "thiscall" fn(u32) -> u32 =
                core::mem::transmute(fin_slot as usize);
            finish(q)
        };
        let tag = lf_checker_rt::callee_cdecl!(CAL_TAG, u32, rd32(tagged.wrapping_add(TAG)), 0);
        let coefs = lf_checker_rt::callee_thiscall!(CAL_COEF, u32, this, tag);
        let ca = rdf(coefs.wrapping_add(COEF_A));
        let cb = rdf(coefs.wrapping_add(COEF_B));
        let m = rd32(this.wrapping_add(MATRIX));
        let z = 0.0f32;
        let o0 = add(
            add(add(mul(rdf(m.wrapping_add(0x10)), ca), mul(rdf(m), z)), mul(rdf(m.wrapping_add(0x20)), cb)),
            rdf(m.wrapping_add(0x30)),
        );
        let o1 = add(
            add(add(mul(rdf(m.wrapping_add(0x14)), ca), mul(rdf(m.wrapping_add(4)), z)), mul(rdf(m.wrapping_add(0x24)), cb)),
            rdf(m.wrapping_add(0x34)),
        );
        let o2 = add(
            add(add(mul(rdf(m.wrapping_add(0x18)), ca), mul(rdf(m.wrapping_add(8)), z)), mul(rdf(m.wrapping_add(0x28)), cb)),
            rdf(m.wrapping_add(0x38)),
        );
        // Fourth word: the original loads it from uninitialized stack
        // scratch; it is 0.0 under the harness's zero stack fill.
        let mut out = [o0, o1, o2, 0.0f32];
        sink_call(this.wrapping_add(SINK), out.as_mut_ptr() as u32, 0);
        0
    }
});
