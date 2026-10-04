// original: 0x00cf73c0 climb_task_matrix_apply (proposed)

/// Applies the climb task's matrix stage: exits with the mode value unless
/// the object at `+0x64` selects matrix mode 2-4 (bits 6-9 of its word at
/// `+0x28`); a null selector returns the caller's leftover eax and is not
/// tested. When the selector's array at `+0x20` is missing it is built
/// through the two builder callees (the second one's fill is scripted), then
/// the 3x3-plus-offset transform combines the matrix rows with the vector at
/// `+0x40`/`+0x44`/`+0x48` in the original's exact operation order and stores
/// the result back there. Returns the array pointer on the compute path.
///
/// Original: 0x00cf73c0 (thiscall: ecx holds the object, no stack words).
lf_checker_rt::export!(thiscall, rw_00cf73c0(this: u32) -> u32 {
    unsafe {
        const BUILD_A: u32 = 1;
        const BUILD_B: u32 = 2;
        #[inline(always)]
        fn mul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }
        #[inline(always)]
        fn add(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) + core::hint::black_box(b)
        }
        #[inline(always)]
        fn lan(a: u32) -> f32 {
            f32::from_bits((a as *const u32).read_unaligned())
        }
        let sel = ((this + 0x64) as *const u32).read_unaligned();
        let mode = (((sel + 0x28) as *const u32).read_unaligned() >> 6) & 0xf;
        if mode < 2 || mode > 4 {
            return mode;
        }
        let mut arr = ((sel + 0x20) as *const u32).read_unaligned();
        if arr == 0 {
            lf_checker_rt::callee_thiscall!(BUILD_A, u32, sel);
            lf_checker_rt::callee_thiscall!(BUILD_B, u32, sel.wrapping_add(0x10), 0);
            arr = ((sel + 0x20) as *const u32).read_unaligned();
        }
        let vx = lan(this + 0x40);
        let vy = lan(this + 0x44);
        let vz = lan(this + 0x48);
        let m0 = lan(arr);
        let m1 = lan(arr + 4);
        let m2 = lan(arr + 0x14);
        let m3 = lan(arr + 0x24);
        let mut r0 = add(mul(m1, vy), mul(m0, vx));
        let mut r1 = mul(m2, vy);
        let mut r2 = mul(m3, vy);
        let t = mul(lan(arr + 8), vz);
        r0 = add(r0, t);
        let t = mul(lan(arr + 0x10), vx);
        r1 = add(r1, t);
        let t = mul(vx, lan(arr + 0x20));
        r2 = add(r2, t);
        let t = mul(lan(arr + 0x18), vz);
        r1 = add(r1, t);
        let t = mul(lan(arr + 0x28), vz);
        ((this + 0x40) as *mut u32).write_unaligned(r0.to_bits());
        ((this + 0x44) as *mut u32).write_unaligned(r1.to_bits());
        r2 = add(r2, t);
        ((this + 0x48) as *mut u32).write_unaligned(r2.to_bits());
        arr
    }
});
