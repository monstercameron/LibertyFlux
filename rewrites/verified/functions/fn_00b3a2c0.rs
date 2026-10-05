// original: 0x00B3A2C0 gate_then_check

/// Gate on callee 1, then forward to callee 2 with a float bit-pattern.
///
/// Callee 1 tests `a`; when its low byte is zero the function returns its
/// full answer unchanged. Otherwise callee 2 runs as `(a, f, c, d)` where `f`
/// is the float in the third argument slot. (The original pushes a scratch
/// register and overwrites the slot; only the float bits are behaviour.)
/// Cdecl, four stack words, returns in eax.
///
/// Original: 0x00B3A2C0.

lf_checker_rt::export!(cdecl, rw_00B3A2C0(a: u32, c: u32, f: u32, d: u32) -> u32 {
    unsafe {
        const GATE: u32 = 1;
        const CHECK: u32 = 2;
        let v: u32 = lf_checker_rt::callee_cdecl!(GATE, u32, a);
        if (v & 0xFF) == 0 {
            return v;
        }
        lf_checker_rt::callee_cdecl!(CHECK, u32, a, f, c, d)
    }
});
