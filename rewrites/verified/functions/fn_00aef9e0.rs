// original: 0x00AEF9E0 stream_level_blend (proposed)

/// Blend factor between two streaming levels, from global stream state.
///
/// `rec` points at a record holding the low and high level (`+0x2C`,
/// `+0x30`, compared signed). Counter `C` comes from a global with an
/// override slot (the override wins unless it holds -1). When the record's
/// levels agree the answer is 1.0. Otherwise, when `(C + 1) % 24` (signed
/// remainder) equals the low level, the answer is a scaled table value
/// `((G_BASE + D * 60) as i32) as f32 * K` where `D` is a second
/// counter-with-override and `K` a read-only constant; when `C` equals the
/// high level it is `ONE - that value`. Otherwise the answer is 0.0 or 1.0
/// from where `C` falls against the level range (inclusive 1.0 inside when
/// low < high, exclusive 0.0 inside when low > high). Single precision in
/// the original's operand order.
///
/// Original: 0x00AEF9E0 (cdecl, one stack word, float result on x87 stack).
lf_checker_rt::export!(cdecl, rw_00aef9e0(rec: u32) -> f32 {
    unsafe {
        const OFF_LO: u32 = 0x2C;
        const OFF_HI: u32 = 0x30;
        const G_C_SRC: u32 = 0x01295848;
        const G_C_OVR: u32 = 0x01295854;
        const G_D_SRC: u32 = 0x0129584C;
        const G_D_OVR: u32 = 0x01295858;
        const G_BASE: u32 = 0x01295850;
        const G_SCALE: u32 = 0x00EA8220;
        const G_ONE: u32 = 0x00FE88E8;
        const NO_OVERRIDE: u32 = 0xFFFF_FFFF;
        const PERIOD: i32 = 24;
        const STRIDE: u32 = 60;
        #[inline(always)]
        fn mul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }
        #[inline(always)]
        fn sub(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) - core::hint::black_box(b)
        }
        unsafe fn gru(a: u32) -> u32 {
            unsafe { (lf_checker_rt::global::<u32>(a)).read_unaligned() }
        }
        unsafe fn grf(a: u32) -> f32 {
            unsafe { f32::from_bits(gru(a)) }
        }
        unsafe fn counter(src: u32, ovr: u32) -> i32 {
            unsafe {
                let o = gru(ovr);
                if o != NO_OVERRIDE {
                    o as i32
                } else {
                    gru(src) as i32
                }
            }
        }
        let lo = ((rec.wrapping_add(OFF_LO)) as *const i32).read_unaligned();
        let hi = ((rec.wrapping_add(OFF_HI)) as *const i32).read_unaligned();
        if lo == hi {
            return 1.0;
        }
        let c = counter(G_C_SRC, G_C_OVR);
        if c.wrapping_add(1) % PERIOD == lo {
            let d = counter(G_D_SRC, G_D_OVR);
            let v = gru(G_BASE).wrapping_add((d as u32).wrapping_mul(STRIDE));
            return mul((v as i32) as f32, grf(G_SCALE));
        }
        if c == hi {
            let d = counter(G_D_SRC, G_D_OVR);
            let v = gru(G_BASE).wrapping_add((d as u32).wrapping_mul(STRIDE));
            return sub(grf(G_ONE), mul((v as i32) as f32, grf(G_SCALE)));
        }
        if lo > hi {
            if c < hi {
                1.0
            } else if c < lo {
                0.0
            } else {
                1.0
            }
        } else if c < lo {
            0.0
        } else if c <= hi {
            1.0
        } else {
            0.0
        }
    }
});
