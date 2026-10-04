// original: 0x00945ae0 NativeImpl_RETUNE_RADIO_UP_3
/// Radio-state predicate: true when either probe succeeds or the stored
/// state byte is already set.
///
/// Calls two parameterless probes in order; the first nonzero low byte wins
/// and its full answer is returned. Otherwise compares a global state byte
/// against zero: equal returns the second answer (low byte clear), different
/// sets the low byte. Only the low byte carries meaning; the upper bytes
/// are the winning probe's answer bits and are reproduced exactly.
lf_checker_rt::export!(cdecl, rw_00945ae0() -> u32 {
    unsafe {
        let a = lf_checker_rt::callee_cdecl!(1, u32,);
        if (a & 0xff) != 0 {
            return a;
        }
        let b = lf_checker_rt::callee_cdecl!(2, u32,);
        if (b & 0xff) != 0 {
            return b;
        }
        let g = *(lf_checker_rt::global::<u8>(0x01037606) as *const u8);
        if g != 0 {
            (b & 0xffff_ff00) | 1
        } else {
            b
        }
    }
});
