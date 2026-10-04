// original: 0x00cbbb20 heading_filter_accept
/// Accept a filtered heading when it moved enough (1 call).
///
/// Passes the float `a0` through the float callee (thiscall, one stack
/// argument). When the threshold `0.1396...` (constant from file address
/// `0x00ED8EF8`) does NOT strictly exceed the absolute difference between
/// the stored heading at `[this + 0x18]` and the filtered value, sets bit
/// `0x10` in the flag word at `[this + 0x54]` and stores the filtered
/// value; otherwise leaves both alone. The subtraction order and the
/// sign-bit absolute value are the original's. No return value (the
/// original never writes eax). The original also writes the filtered
/// value over its own incoming argument slot, which the rewrite cannot
/// reproduce, so the stack check is off for this function.
lf_checker_rt::export!(thiscall, rw_00cbbb20(this: u32, a0: u32) -> u32 {
    unsafe {
        /// Stored heading and flag word offsets.
        const HDG_OFF: u32 = 0x18;
        const FLAG_OFF: u32 = 0x54;
        const ACCEPT_BIT: u32 = 0x10;
        /// Accept threshold (constant from file address 0x00ED8EF8).
        const THRESH: f32 = f32::from_bits(0x3E0EFA35);
        const SIGN: u32 = 0x8000_0000;
        /// Callee id of the float normaliser.
        const NORMALISE: u32 = 1;
        #[inline(always)]
        unsafe fn rdf(a: u32) -> f32 {
            unsafe { f32::from_bits((a as *const u32).read_unaligned()) }
        }
        #[inline(always)]
        fn sub(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) - core::hint::black_box(b)
        }
        let f: f32 = lf_checker_rt::callee_cdecl!(NORMALISE, f32, a0);
        let d = sub(rdf(this + HDG_OFF), f);
        let ad = f32::from_bits(d.to_bits() & !SIGN);
        if !(THRESH > ad) {
            let fl = ((this + FLAG_OFF) as *const u32).read_unaligned();
            ((this + FLAG_OFF) as *mut u32).write_unaligned(fl | ACCEPT_BIT);
            ((this + HDG_OFF) as *mut u32).write_unaligned(f.to_bits());
        }
        0
    }
});
