// original: 0x00c058a0 stream_amplitude_to_db
/// Map a linear amplitude to a scaled log-domain value, flooring tiny input.
///
/// Returns -100.0 when `a0 - 0.0032 < 0.0` (a NaN input also takes this path,
/// since the comparison is unordered). Otherwise splits the bits into a
/// 9-bit exponent `e` and a mantissa `m` in [1, 2) and evaluates
/// `(((m * K4 + ((f32)((f64)e + D0) - 127.0) - 1.6749035)) - m * m * 0.34484765)
/// * 0.30103 * 40.0` with the original's SSE operation order (each multiply,
/// add and subtract pinned through `black_box`). The double table `D` is read
/// at index `e >> 31`, which is always 0 for a 9-bit value. Cdecl: one stack
/// word holding float bits, result in ST0. The stack check is off: the
/// original spills intermediate values into its incoming argument slot
/// (see `narrowed`).
lf_checker_rt::export!(cdecl, rw_00c058a0(a0: u32) -> f32 {
    unsafe {
#[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn rd16(a: u32) -> u32 {
            unsafe { (a as *const u16).read_unaligned() as u32 }
        }
        #[inline(always)]
        unsafe fn wr16(a: u32, v: u16) {
            unsafe { (a as *mut u16).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn rd8(a: u32) -> u8 {
            unsafe { (a as *const u8).read() }
        }
        #[inline(always)]
        unsafe fn wr8(a: u32, v: u8) {
            unsafe { (a as *mut u8).write(v) }
        }
        #[inline(always)]
        fn fadd(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) + core::hint::black_box(b)
        }
        #[inline(always)]
        fn fsub(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) - core::hint::black_box(b)
        }
        #[inline(always)]
        fn fmul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }

        #[inline(always)]
        unsafe fn rd64(a: u32) -> u64 {
            unsafe { (a as *const u64).read_unaligned() }
        }
        const K_FLOOR_SUB: u32 = 0x00EBDB34;
        const K_FLOOR_CMP: u32 = 0x00FE8628;
        const K_EXP_BIAS: u32 = 0x00FE8BC4;
        const K_LIN: u32 = 0x00E78584;
        const K_OFF: u32 = 0x00E78580;
        const K_SQR: u32 = 0x00E7857C;
        const K_LOG10_2: u32 = 0x00E78578;
        const K_SCALE: u32 = 0x00FE8B5C;
        const TBL: u32 = 0x00FE8F50;
        const FLOOR: f32 = f32::from_bits(0xC2C8_0000);
        let x = f32::from_bits(a0);
        let k1 = f32::from_bits(rd32(lf_checker_rt::relocated(K_FLOOR_SUB)));
        let d = fsub(x, k1);
        let e9 = a0 >> 23;
        let m = f32::from_bits((a0 & 0x007F_FFFF) | 0x3F80_0000);
        let slot = (e9 >> 31) as u32;
        let d0 = f64::from_bits(rd64(lf_checker_rt::relocated(TBL) + slot * 8));
        let base = (core::hint::black_box(e9 as f64) + core::hint::black_box(d0)) as f32;
        let k2 = f32::from_bits(rd32(lf_checker_rt::relocated(K_FLOOR_CMP)));
        if !(d >= k2) {
            return FLOOR;
        }
        let k3 = f32::from_bits(rd32(lf_checker_rt::relocated(K_EXP_BIAS)));
        let k4 = f32::from_bits(rd32(lf_checker_rt::relocated(K_LIN)));
        let k5 = f32::from_bits(rd32(lf_checker_rt::relocated(K_OFF)));
        let k6 = f32::from_bits(rd32(lf_checker_rt::relocated(K_SQR)));
        let k7 = f32::from_bits(rd32(lf_checker_rt::relocated(K_LOG10_2)));
        let k8 = f32::from_bits(rd32(lf_checker_rt::relocated(K_SCALE)));
        let mut x0 = fsub(base, k3);
        let mut x2 = fmul(m, k4);
        x0 = fsub(x0, k5);
        x2 = fadd(x2, x0);
        let m2 = fmul(m, m);
        x2 = fsub(x2, fmul(m2, k6));
        x2 = fmul(x2, k7);
        x2 = fmul(x2, k8);
        x2
    }
});
