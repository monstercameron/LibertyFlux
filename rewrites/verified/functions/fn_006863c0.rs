// original: 0x006863c0 lookup_quat_to_matrix (proposed)

/// Look up a quaternion frame by two keys and expand it to a 3x3 rotation
/// matrix.
///
/// `key0` and `key1` are forwarded to the lookup callee, which answers with
/// a frame pointer or null. A null answer, or a frame whose flag byte at
/// `+0x04` has bit `0x10` set, fails with 0 and writes nothing. Otherwise
/// the quaternion at `+0x10`/`+0x14`/`+0x18`/`+0x1c` (x, y, z, w) is scaled
/// by the global at `SCALE_ADDR` and written to `out` as nine floats on a
/// 16-byte row stride (`+0x00`/`+0x04`/`+0x08`, `+0x10`/`+0x14`/`+0x18`,
/// `+0x20`/`+0x24`/`+0x28`; the `+0x0c` and `+0x1c` slots are untouched):
/// the diagonal is the global at `ONE_ADDR` minus the pairwise sums of the
/// squared scaled components, the off-diagonals are the usual
/// sum/difference products. Returns 1 in `al` on success.
///
/// Original: 0x006863c0 (stdcall, three stack words). Only `al` is the
/// result; `ah` keeps the lookup's leftover.
lf_checker_rt::export!(stdcall, rw_006863C0(key0: u32, key1: u32, out: u32) -> u32 {
    unsafe {
        const LOOKUP: u32 = 1;
        const FLAGS_OFF: u32 = 0x04;
        const SKIP_FLAG: u8 = 0x10;
        const QX: u32 = 0x10;
        const QY: u32 = 0x14;
        const QZ: u32 = 0x18;
        const QW: u32 = 0x1c;
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

        let frame: u32 = lf_checker_rt::callee_stdcall!(LOOKUP, u32, key0, key1);
        if frame == 0 {
            return 0;
        }
        let flags: u8 = unsafe { ((frame + FLAGS_OFF) as *const u8).read() };
        if flags & SKIP_FLAG != 0 {
            return 0;
        }
        let scale: f32 = unsafe { rdf(lf_checker_rt::relocated(SCALE_ADDR)) };
        let one: f32 = unsafe { rdf(lf_checker_rt::relocated(ONE_ADDR)) };
        let mut x = unsafe { rdf(frame + QX) };
        let mut y = unsafe { rdf(frame + QY) };
        let mut z = unsafe { rdf(frame + QZ) };
        let mut w = unsafe { rdf(frame + QW) };
        y = mul(y, scale);
        x = mul(x, scale);
        z = mul(z, scale);
        w = mul(w, scale);
        let yx = mul(y, x);
        let wz = mul(w, z);
        let m10 = sub(yx, wz);
        let m04 = add(yx, wz);
        let zx = mul(z, x);
        unsafe {
            wrf(out + 0x10, m10);
            wrf(out + 0x04, m04);
        }
        let wy = mul(w, y);
        let wx = mul(w, x);
        let m08 = sub(zx, wy);
        let m20 = add(wy, zx);
        let x2 = mul(x, x);
        unsafe {
            wrf(out + 0x08, m08);
            wrf(out + 0x20, m20);
        }
        let zy = mul(z, y);
        let z2 = mul(z, z);
        let m18 = add(wx, zy);
        let m24 = sub(zy, wx);
        let y2 = mul(y, y);
        unsafe {
            wrf(out + 0x18, m18);
            wrf(out + 0x24, m24);
        }
        let m00 = sub(one, add(z2, y2));
        unsafe { wrf(out + 0x00, m00) };
        let m14 = sub(one, add(z2, x2));
        let m28 = sub(one, add(y2, x2));
        unsafe {
            wrf(out + 0x14, m14);
            wrf(out + 0x28, m28);
        }
        1
    }
});
