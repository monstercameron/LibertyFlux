// original: 0x00D4E790 task_restamp_random_period (proposed)

// Re-stamps the task tick and randomises its period.
///
/// Passes the stack argument in ECX with (-1, 1) to intercepted callee 1,
/// stores the global tick at `+0x14`, and returns `this`. When the signed
/// word at `+0x1c` exceeds -1 a fresh value is drawn: intercepted callee 2
/// supplies 32 bits, the low 16 are scaled by two constant factors (read from
/// the image) with the original's multiply order, truncated to an integer,
/// and subtracted from 1000; the low 16 bits land back at `+0x1c`.
///
/// Original: 0x00D4E790 (thiscall, one stack word).
lf_checker_rt::export!(thiscall, rw_00d4e790(this: u32, arg0: u32) -> u32 {
    unsafe {
        const TICK_SLOT: u32 = 0x011735B4;
        const SCALE_A: u32 = 0x00FE8680;
        const SCALE_B: u32 = 0x00E9C7DC;
        const BASE_PERIOD: u32 = 1000;
        const C1: u32 = 1;
        const C2: u32 = 2;
        #[inline(always)]
        fn mul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }
        lf_checker_rt::callee_thiscall!(C1, u32, arg0, 0xFFFFFFFF, 1);
        let tick = lf_checker_rt::global::<u32>(TICK_SLOT).read();
        ((this + 0x14) as *mut u32).write_unaligned(tick);
        if ((this + 0x1c) as *const i16).read_unaligned() > -1 {
            let raw: u32 = lf_checker_rt::callee_cdecl!(C2, u32,);
            let v = (raw & 0xFFFF) as f32;
            let a = f32::from_bits(lf_checker_rt::global::<u32>(SCALE_A).read());
            let b = f32::from_bits(lf_checker_rt::global::<u32>(SCALE_B).read());
            let scaled = mul(mul(v, a), b);
            let i = scaled as i32 as u32;
            let c = BASE_PERIOD.wrapping_sub(i);
            ((this + 0x1c) as *mut u16).write_unaligned(c as u16);
        }
        this
    }
});
