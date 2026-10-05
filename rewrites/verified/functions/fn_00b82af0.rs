// original: 0x00B82AF0 slot_search_float
/// Find the first slot matching `(tgt, word)` whose scaled tick fits.
///
/// Scans the 140 `STRIDE`-byte slots for a kind byte equal to `tgt`
/// with the word at `+0x24` equal to `word` (a zero kind additionally
/// needs the byte at `+0x20` clear or `strict` set). Each candidate's
/// tick (callee 1, low 16 bits, as float, scaled by two constants) must
/// convert below the limit at `+0x26`; the winner's index is stored to
/// `out` and 1 returned, else 0.
///
/// Original: 0x00B82AF0 (thiscall, four stack arguments).
lf_checker_rt::export!(thiscall, rw_00B82AF0(this: u32, out: u32, tgt: u32, word: u32, strict: u32) -> u32 {
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
        #[inline(always)]
        fn cvtt(x: f32) -> i32 {
            if x.is_nan() {
                return i32::MIN;
            }
            if x >= 2147483648.0 || x < -2147483648.0 {
                return i32::MIN;
            }
            x as i32
        }
        const SLOTS: u32 = 0x8C;
        const STRIDE: u32 = 0x2C;
        const C1: u32 = 0x00FE8680;
        const C2: u32 = 0x00FE8BB0;
        let want = (tgt & 0xFF) as u8;
        let sb = (strict & 0xFF) as u8;
        let mut i = 0u32;
        while i < SLOTS {
            let slot = this.wrapping_add(i.wrapping_mul(STRIDE));
            let mut candidate = false;
            if rd8(slot + 0x18) == want {
                let w = unsafe { (slot.wrapping_add(0x24) as *const u16).read_unaligned() as i16 as i32 as u32 };
                if w == word {
                    if want != 0 || rd8(slot + 0x20) == 0 || sb != 0 {
                        candidate = true;
                    }
                }
            }
            if candidate {
                let tick = lf_checker_rt::callee_cdecl!(1, u32,) & 0xFFFF;
                let lim = unsafe { (slot.wrapping_add(0x26) as *const u16).read_unaligned() as i32 };
                let c1 = (lf_checker_rt::global::<u32>(C1)).read_unaligned();
                let c2 = (lf_checker_rt::global::<u32>(C2)).read_unaligned();
                let scaled = mul(mul(tick as f32, f32::from_bits(c1)), f32::from_bits(c2));
                if cvtt(scaled) < lim {
                    wr8(out, i as u8);
                    return 1;
                }
            }
            i += 1;
        }
        0
    }
});
