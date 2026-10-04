// original: 0x00aba0d0 GB_CAP_USE
/// Replace the low byte of `hi` with `lo`, keeping the upper 24 bits.
///
/// This trivial combine is deliberately `#[inline(never)]`: rustc 1.96.1
/// in release for i686 miscompiles the inlined form
/// `(hi & 0xFFFF_FF00) | (lo as u32)` into just `lo` whenever `lo` was
/// loaded from an address derived from `hi` (the mask, shift, byte-array,
/// store and volatile spellings all fold the same way; the out-of-line
/// call is the only form observed to keep the upper bits). The checker
/// proves the combined value on every trial, so any toolchain drift that
/// reintroduces the fold fails loudly. Do not inline this function.
#[inline(never)]
fn with_low_byte(hi: u32, lo: u8) -> u32 {
    (hi & 0xFFFF_FF00) | (lo as u32)
}

/// Resolve two capability slots and report success.
///
/// Runs the same block twice (format/slot pair): fetch a handle through
/// callee 1 into a local slot; when non-null, publish it to the output
/// global, register it with callee 2 alongside the shared global, and,
/// when the handle's word at +0x52 is positive, notify callee 3. Finishes
/// with a reset call (callee 4) and returns its answer with the low byte
/// forced to 1. Skipped blocks leave their output global untouched.
lf_checker_rt::export!(cdecl, rw_00aba0d0() -> u32 {
    let mut slot = [0u32; 1];
    let slot_ptr = slot.as_mut_ptr() as u32;
    // SAFETY: the slot is a local; the globals are declared image ranges
    // and the handle reads stay in checker-backed heap on every trial.
    for (fmt, out) in [(0x00EA_55BCu32, 0x0150_E1C0u32), (0x00EA_55C8u32, 0x0150_E1C4u32)] {
        unsafe { (slot_ptr as *mut u32).write_unaligned(0) };
        let h = lf_checker_rt::callee_cdecl!(1, u32, lf_checker_rt::relocated(fmt), slot_ptr);
        if h != 0 {
            let g = unsafe { lf_checker_rt::global::<u32>(0x012B_4138).read_unaligned() };
            unsafe { lf_checker_rt::global::<u32>(out).write_unaligned(h) };
            let s = unsafe { (slot_ptr as *const u32).read_unaligned() };
            let _ = lf_checker_rt::callee_cdecl!(2, u32, s, g, 2);
            let h2 = unsafe { lf_checker_rt::global::<u32>(out).read_unaligned() };
            let w = unsafe { ((h2.wrapping_add(0x52)) as *const i16).read_unaligned() } as i32;
            if w > -1 {
                let _ = lf_checker_rt::callee_cdecl!(3, u32, w as u32, 2);
            }
        }
    }
    let r = lf_checker_rt::callee_cdecl!(4, u32, 0);
    with_low_byte(r, 1)
});

