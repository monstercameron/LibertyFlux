// original: 0x008f7fd0 input_backend_init (proposed)

/// Bring up the input backend through its six init callees.
///
/// Takes no arguments and reads no object. Five init callees run with
/// constant arguments accumulating on the stack (0; 0; the float 1.0;
/// 0 with 1.0; 0, 0 with 1.0), all cdecl. The global backend pointer is
/// then read: when null the function returns 0, otherwise the pointer is
/// handed to the sixth callee and its answer is returned.
///
/// Cdecl: no stack words.
lf_checker_rt::export!(cdecl, rw_008f7fd0() -> u32 {
    unsafe {
        const C_A: u32 = 1;
        const C_B: u32 = 2;
        const C_C: u32 = 3;
        const C_D: u32 = 4;
        const C_E: u32 = 5;
        const C_F: u32 = 6;
        const ONE: u32 = 0x3f800000;
        const G_BACK: u32 = 0x15b0e8c;
        let _: u32 = lf_checker_rt::callee_cdecl!(C_A, u32, 0);
        let _: u32 = lf_checker_rt::callee_cdecl!(C_B, u32, 0);
        let _: u32 = lf_checker_rt::callee_cdecl!(C_C, u32, ONE);
        let _: u32 = lf_checker_rt::callee_cdecl!(C_D, u32, 0, ONE);
        let _: u32 = lf_checker_rt::callee_cdecl!(C_E, u32, 0, 0, ONE);
        let g = (lf_checker_rt::global::<u32>(G_BACK)).read_unaligned();
        if g == 0 {
            0
        } else {
            lf_checker_rt::callee_cdecl!(C_F, u32, g)
        }
    }
});
