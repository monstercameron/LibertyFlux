// original: 0x00685fe0 dof_pair_combine (proposed)

/// Resolve two frames by key and combine them into an output object.
///
/// `r1 = lookup(key0, key2)` and `r2 = lookup(key1, key2)`. When the low
/// byte of `flag` is non-zero, each valid result contributes on its own: a
/// non-null `r1` whose flag byte at `+0x04` lacks bit `0x10` has its
/// quaternion (`+0x10`..`+0x1c`) copied to `out+0x30`..`out+0x3c`, and a
/// likewise valid `r2` has its quaternion scaled by the global at
/// `SCALE_ADDR` and expanded to a 3x3 rotation matrix on a 16-byte row
/// stride at `out+0x00`..`out+0x28` (diagonal from the global at
/// `ONE_ADDR`, same expansion as `rw_006863C0`). The result is 1 unless
/// neither contributed. When the flag byte is zero, both results must be
/// valid or the result is 0 with no writes; otherwise the `r1` quaternion
/// is copied and the `r2` quaternion address is handed to the set-quaternion
/// callee with `out` in `ecx`, and the result is 1. `this` is unused.
///
/// Original: 0x00685fe0 (thiscall, five stack words). Only `al` is the
/// result; `ah` keeps the lookup's leftover.
lf_checker_rt::export!(thiscall, rw_00685FE0(_this: u32, key0: u32, key1: u32, key2: u32, out: u32, flag: u32) -> u32 {
    unsafe {
        const LOOKUP1: u32 = 1;
        const LOOKUP2: u32 = 2;
        const SETQUAT: u32 = 3;
        const FLAGS_OFF: u32 = 0x04;
        const SKIP_FLAG: u8 = 0x10;
        const QX: u32 = 0x10;
        const QY: u32 = 0x14;
        const QZ: u32 = 0x18;
        const QW: u32 = 0x1c;
        const DST_QUAT: u32 = 0x30;
        const SCALE_ADDR: u32 = 0x00FE8944;
        const ONE_ADDR: u32 = 0x00FE88E8;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rdf(a: u32) -> f32 {
            unsafe { f32::from_bits(rd32(a)) }
        }
        #[inline(always)]
        unsafe fn wrf(a: u32, v: f32) {
            unsafe { (a as *mut u32).write_unaligned(v.to_bits()) }
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
        fn sub(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) - core::hint::black_box(b)
        }
        #[inline(always)]
        unsafe fn valid(frame: u32) -> bool {
            unsafe {
                frame != 0 && ((frame + FLAGS_OFF) as *const u8).read() & SKIP_FLAG == 0
            }
        }
        #[inline(always)]
        unsafe fn copy_quat(frame: u32, out: u32) {
            unsafe {
                for i in 0..4u32 {
                    wrf(out + DST_QUAT + i * 4, rdf(frame + QX + i * 4));
                }
            }
        }
        /// Same scaled quaternion-to-matrix expansion as `rw_006863C0`.
        #[inline(always)]
        unsafe fn expand_matrix(frame: u32, out: u32) {
            unsafe {
                let scale: f32 = rdf(lf_checker_rt::relocated(SCALE_ADDR));
                let one: f32 = rdf(lf_checker_rt::relocated(ONE_ADDR));
                let mut x = rdf(frame + QX);
                let mut y = rdf(frame + QY);
                let mut z = rdf(frame + QZ);
                let mut w = rdf(frame + QW);
                y = mul(y, scale);
                x = mul(x, scale);
                z = mul(z, scale);
                w = mul(w, scale);
                let yx = mul(y, x);
                let wz = mul(w, z);
                wrf(out + 0x10, sub(yx, wz));
                wrf(out + 0x04, add(yx, wz));
                let zx = mul(z, x);
                let wy = mul(w, y);
                let wx = mul(w, x);
                wrf(out + 0x08, sub(zx, wy));
                wrf(out + 0x20, add(wy, zx));
                let zy = mul(z, y);
                let z2 = mul(z, z);
                wrf(out + 0x18, add(wx, zy));
                wrf(out + 0x24, sub(zy, wx));
                let y2 = mul(y, y);
                let x2 = mul(x, x);
                wrf(out + 0x00, sub(one, add(z2, y2)));
                wrf(out + 0x14, sub(one, add(z2, x2)));
                wrf(out + 0x28, sub(one, add(y2, x2)));
            }
        }

        let r1: u32 = lf_checker_rt::callee_stdcall!(LOOKUP1, u32, key0, key2);
        let r2: u32 = lf_checker_rt::callee_stdcall!(LOOKUP2, u32, key1, key2);
        if (flag as u8) != 0 {
            let mut acc = 0u32;
            if unsafe { valid(r1) } {
                unsafe { copy_quat(r1, out) };
                acc = 1;
            }
            if unsafe { valid(r2) } {
                unsafe { expand_matrix(r2, out) };
                acc = 1;
            }
            acc
        } else {
            if !(unsafe { valid(r1) } && unsafe { valid(r2) }) {
                return 0;
            }
            unsafe { copy_quat(r1, out) };
            let _ans: u32 =
                lf_checker_rt::callee_thiscall!(SETQUAT, u32, out, r2.wrapping_add(QX));
            1
        }
    }
});
