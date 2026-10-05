// original: 0x00B859A0 scaled_pick
/// Pick the first table entry at or above the scaled tick.
///
/// Seeds `dst` with `0xC8` and `dpos` with -1, then scans the float
/// table at `+0x18` (length at `+0x30`) for the first entry ordered
/// greater-or-equal to the tick (callee 1, as float, scaled by a
/// constant). The matching source word from `+0` and its index are
/// stored to `dst`/`dpos`. No match (or an aliased `dst`/`dpos`, which
/// fails the seed check): the seeds stand.
///
/// Original: 0x00B859A0 (thiscall, three stack arguments, no return value).
lf_checker_rt::export!(thiscall, rw_00B859A0(this: u32, _u: u32, dst: u32, dpos: u32) -> u32 {
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
        unsafe fn rd8(a: u32) -> u8 {
            unsafe { (a as *const u8).read() }
        }
        #[inline(always)]
        unsafe fn wr8(a: u32, v: u8) {
            unsafe { (a as *mut u8).write(v) }
        }
        #[inline(always)]
        unsafe fn rdf(a: u32) -> f32 {
            unsafe { f32::from_bits((a as *const u32).read_unaligned()) }
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
        fn mul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }
        const SCALE: u32 = 0x00FE8684;
        wr32(dst, 0xC8);
        wr32(dpos, 0xFFFF_FFFF);
        if rd32(dst) != 0xC8 {
            return 0;
        }
        let tick = lf_checker_rt::callee_cdecl!(1, u32,) as i32;
        let c = (lf_checker_rt::global::<u32>(SCALE)).read_unaligned();
        let f = mul(tick as f32, f32::from_bits(c));
        let count = rd32(this + 0x30) as i32;
        if count <= 0 {
            return 0;
        }
        let mut i = 0i32;
        while i < count {
            if rdf(this.wrapping_add(0x18).wrapping_add((i as u32) * 4)) >= f {
                wr32(dst, rd32(this.wrapping_add((i as u32) * 4)));
                wr32(dpos, i as u32);
                return 0;
            }
            i += 1;
        }
        0
    }
});
