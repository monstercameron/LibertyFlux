// original: 0x00a633a0 ped_task_stat_update (proposed)

/// Update a clamped 0..100 task stat from a scaled input value.
///
/// `this+0xd8` is a handle passed to the registry lookup, `this+0x2c8` the
/// signed stat byte. The function resolves a shared registry object through
/// the writable global `G_REGISTRY` (allocating it with callee 0, cdecl,
/// arg `ALLOC_TAG`, then constructing with callee 1, thiscall, when the
/// global is null), looks the handle up twice with callee 2 (thiscall, one
/// arg), and on a partial match consults callee 3 (thiscall on `this`).
/// Each answer's flag word at `+0x8f4` selects the input scale: no flag
/// bits on the first lookup, or both bits on the second, gives 1.0; a
/// partial second match with exactly bit `0x8000` on the third answer
/// gives 0.75; anything else gives 0.0.
///
/// The stat update is `clamp(sx8(truncate(scale * arg)) + sx8(old), 0,
/// 100)`, where only the low byte of the `cvttss2si` truncation is kept
/// (an out-of-range or NaN input truncates to `0x80000000`, contributing
/// 0). The return value is the sign-extended old byte when the sum clamps
/// at 100, else the sum itself or 0.
///
/// Float operation order is the original's SSE order. Original: 0x00a633a0
/// (thiscall, one stack word: the input float bits).
lf_checker_rt::export!(thiscall, rw_00a633a0(this: u32, arg: u32) -> u32 {
    unsafe {
        const HANDLE: u32 = 0xd8;
        const STAT: u32 = 0x2c8;
        const FLAG_WORD: u32 = 0x8f4;
        const FLAG_MASK: u32 = 0x18000;
        const FLAG_FULL: u32 = 0x18000;
        const FLAG_LOW: u32 = 0x8000;
        const ALLOC_TAG: u32 = 0x20020;
        const G_REGISTRY: u32 = 0x0167e3b4;
        const G_ONE: u32 = 0x00fe88e8;
        const G_THREE_Q: u32 = 0x00fe888c;
        const CALLEE_ALLOC: u32 = 0;
        const CALLEE_BUILD: u32 = 1;
        const CALLEE_LOOKUP: u32 = 2;
        const CALLEE_CONSULT: u32 = 3;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
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
            unsafe { f32::from_bits(rd32(a)) }
        }
        #[inline(always)]
        fn mul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }

        let g = lf_checker_rt::global::<u32>(G_REGISTRY);
        let mut reg = g.read_unaligned();
        if reg == 0 {
            let p: u32 = lf_checker_rt::callee_cdecl!(CALLEE_ALLOC, u32, ALLOC_TAG);
            reg = if p == 0 {
                0
            } else {
                lf_checker_rt::callee_thiscall!(CALLEE_BUILD, u32, p)
            };
            g.write_unaligned(reg);
        }
        let hit1: u32 = lf_checker_rt::callee_thiscall!(CALLEE_LOOKUP, u32, reg, rd32(this + HANDLE));
        let scale = if rd32(hit1 + FLAG_WORD) & FLAG_MASK == 0 {
            rdf(lf_checker_rt::relocated(G_ONE))
        } else {
            let mut reg = g.read_unaligned();
            if reg == 0 {
                let p: u32 = lf_checker_rt::callee_cdecl!(CALLEE_ALLOC, u32, ALLOC_TAG);
                reg = if p == 0 {
                    0
                } else {
                    lf_checker_rt::callee_thiscall!(CALLEE_BUILD, u32, p)
                };
                g.write_unaligned(reg);
            }
            let hit2: u32 =
                lf_checker_rt::callee_thiscall!(CALLEE_LOOKUP, u32, reg, rd32(this + HANDLE));
            if rd32(hit2 + FLAG_WORD) & FLAG_MASK == FLAG_FULL {
                rdf(lf_checker_rt::relocated(G_ONE))
            } else {
                let hit3: u32 = lf_checker_rt::callee_thiscall!(CALLEE_CONSULT, u32, this);
                if rd32(hit3 + FLAG_WORD) & FLAG_MASK == FLAG_LOW {
                    rdf(lf_checker_rt::relocated(G_THREE_Q))
                } else {
                    0.0
                }
            }
        };
        let t = mul(scale, f32::from_bits(arg));
        // Low byte of cvttss2si: NaN and out-of-range give 0x80000000.
        let lo: u8 = if t.is_nan() || t >= 2147483648.0 || t < -2147483648.0 {
            0
        } else {
            (t as i32) as u8
        };
        let old = rd8(this + STAT) as i8 as i32;
        let sum = (lo as i8 as i32) + old;
        if sum > 0 {
            if sum < 0x64 {
                wr8(this + STAT, sum as u8);
                sum as u32
            } else {
                wr8(this + STAT, 0x64);
                old as u32
            }
        } else {
            wr8(this + STAT, 0);
            0
        }
    }
});
