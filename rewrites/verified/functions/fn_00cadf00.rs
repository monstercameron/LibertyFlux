// original: 0x00cadf00 motion_output_update (proposed)
/// Fill a motion output record from a queried pose, gated by speed and tag.
///
/// `a` points at the source (handle at `+0xD68`, velocity at `+0xD70`/`+0xD74`/
/// `+0xD78`); `out` takes 16 bytes, `out2` a type word. Queries a four-dword
/// pose for the handle into scratch (intercepted callee 1, thiscall/1 with the
/// scratch address as its stack argument; its contents are scripted and
/// compared through the copy). When `flag` is zero returns 0 at once.
/// Otherwise stores 2 into `out2`, then compares the squared speed
/// (`x*x + y*y + z*z` in that association, pinned) against the limit `0.01`
/// from `0x00FE8710`: below the limit the low byte of `byteflag` decides
/// (zero returns 0, nonzero continues); at or above, or NaN, continues.
/// Then reads the tag at the handle's `+0x00` masked with 7: 2 or 3 copies
/// the queried pose into `out`, anything else zeroes `out[0..8]`. Returns 1.
/// Original is cdecl(`a`, `out`, `flag`, `out2`, `byteflag`).
lf_checker_rt::export!(cdecl, rw_00cadf00(a: u32, out: u32, flag: u32, out2: u32, byteflag: u32) -> u32 {
    unsafe {
        const QUERY: u32 = 1;
        const HANDLE: u32 = 0xD68;
        const VEL: u32 = 0xD70;
        const LIMIT: u32 = 0x00FE_8710;
        #[inline(always)]
        fn add(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) + core::hint::black_box(b)
        }
        #[inline(always)]
        fn mul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }
        #[inline(always)]
        unsafe fn dw(base: u32, off: u32) -> u32 {
            unsafe { ((base.wrapping_add(off)) as *const u32).read_unaligned() }
        }
        let mut buf = [0u32; 4];
        let _: u32 = lf_checker_rt::callee_thiscall!(QUERY, u32,
            dw(a, HANDLE), buf.as_mut_ptr() as u32);
        if flag == 0 {
            return 0;
        }
        (out2 as *mut u32).write_unaligned(2);
        let x = f32::from_bits(dw(a, VEL));
        let y = f32::from_bits(dw(a, VEL.wrapping_add(4)));
        let z = f32::from_bits(dw(a, VEL.wrapping_add(8)));
        let len2 = add(add(mul(x, x), mul(y, y)), mul(z, z));
        let limit = f32::from_bits(
            (lf_checker_rt::global::<u32>(LIMIT) as *const u32).read());
        if len2 < limit {
            if byteflag & 0xFF == 0 {
                return 0;
            }
        }
        let tag = dw(dw(a, HANDLE), 0) & 7;
        if tag == 2 || tag == 3 {
            for k in 0..4u32 {
                ((out.wrapping_add(k * 4)) as *mut u32).write_unaligned(buf[k as usize]);
            }
        } else {
            for k in 0..3u32 {
                ((out.wrapping_add(k * 4)) as *mut u32).write_unaligned(0);
            }
        }
        1
    }
});
