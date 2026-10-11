// original: 0x00d2a870 task_pick_weighted (proposed)
/// Pick one of three weighted entries: draw `r` from the random source,
/// scale `(r & 0xffff)` by the two global factors and truncate; when the
/// flag byte of `arg` is set and the low byte is below 0x64, return 7.
/// Otherwise map the global level to a cap (<=2: 0, <=4: 1, 5: 2, else 3)
/// and return the first entry (`[this+4+i*4]`) whose cumulative weight
/// (`[this+0x10+i]`, bytes) exceeds the draw, provided its index is within
/// the cap; 0 when none qualifies.
///
/// Thiscall, one stack word. Float order matches the original.
lf_checker_rt::export!(thiscall, rw_00d2a870(this: u32, arg: u32) -> u32 {
    unsafe {
        const RAND: u32 = 1;
        const F1_GLOB: u32 = 0x00fe8680;
        const F2_GLOB: u32 = 0x00fe8c10;
        const LEVEL_GLOB: u32 = 0x0103b6ec;
        const QUICK_RET: u32 = 7;
        const QUICK_LIM: u8 = 0x64;
        fn mul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }
        fn cvtt(x: f32) -> i32 {
            if x >= -2147483648.0 && x < 2147483648.0 {
                core::hint::black_box(x) as i32
            } else {
                i32::MIN
            }
        }
        let r: u32 = lf_checker_rt::callee_thiscall!(RAND, u32, this);
        let f1 = lf_checker_rt::global::<f32>(F1_GLOB).read_unaligned();
        let f2 = lf_checker_rt::global::<f32>(F2_GLOB).read_unaligned();
        let draw = mul(mul((r & 0xffff) as f32, f1), f2);
        let n = cvtt(draw);
        let bl = n as u8;
        if (arg & 0xff) as u8 != 0 && bl < QUICK_LIM {
            return QUICK_RET;
        }
        let level = lf_checker_rt::global::<u32>(LEVEL_GLOB).read_unaligned();
        let cap: u32 = if level > 2 { if level > 4 { if level > 5 { 3 } else { 2 } } else { 1 } } else { 0 };
        let mut acc: u8 = 0;
        for i in 0..3u32 {
            acc = acc.wrapping_add(((this + 0x10 + i) as *const u8).read());
            if bl < acc && i <= cap {
                return ((this + 4 + i * 4) as *const u32).read_unaligned();
            }
        }
        0
    }
});
