// original: 0x00a892d0 signal_gate_check
/// Decide a 0/1 gate from two float stages and object flags.
///
/// Takes the indirect reading (vtable slot 0x13, intercepted); when it is
/// below zero (NaN included) the second stage is skipped. Otherwise runs
/// the shaping helper (intercepted, cdecl/5) and returns 0 when its output
/// exceeds the reading. The fallback returns 0 only for flag classes 2-4
/// with the marker set; 1 otherwise. Only the low byte is compared.
export!(cdecl, rw_00a892d0(arg0: u32, arg1: u32, arg2bits: u32) -> u32 {
    unsafe {
        let obj = *((arg0.wrapping_add(4)) as *const u32);
        let inner = *((obj.wrapping_add(0xc)) as *const u32);
        let vtable = *(inner as *const u32);
        let target = *((vtable.wrapping_add(0x4c)) as *const u32);
        let f: extern "thiscall" fn(u32) -> f32 =
            core::mem::transmute(target as usize);
        let r1 = f(inner);
        let t = f32::from_bits(*global::<u32>(0xfe8628));
        // jb: below or NaN skips the second stage.
        if !(r1.is_nan() || t.is_nan() || r1 < t) {
            let a2 = f32::from_bits(arg2bits);
            let r2: f32 = callee_cdecl!(2, f32, arg2bits, 0x3dcccccd, 0x42480000,
                                        0x3cf5c28f, 0x3f800000);
            let _ = a2;
            // jbe: at-or-below or NaN falls through; above returns 0.
            if r2 > r1 {
                return 0;
            }
        }
        let v = ((*((arg1.wrapping_add(0x28)) as *const u32) >> 6) & 0xf);
        if v > 1
            && v < 5
            && *((arg1.wrapping_add(0x1f8)) as *const u32) != 0
            && *((arg1.wrapping_add(0x1f4)) as *const u8) & 8 != 0
        {
            0
        } else {
            1
        }
    }
});
