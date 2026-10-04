// original: 0x00cf96f0 climb_task_rate_solve (proposed)

/// Solves a climb task's rate: null state at `+0x60` returns 0.0. Otherwise
/// each of the two sampler calls takes the state float at `+0x4c` (first) or
/// the cached rate at `+0x58` (second) plus a selector (the word at `+0x40`
/// when the word at `+0x44` equals 1, else 0), writing three words per
/// out-slot of which only the third is read; the first two are unobserved.
/// The cached rate becomes the state float, and the result is `(r1 - r2) *
/// 1.0039153`, negated when above 0.0 for tags 0x88/0x8a or when below 0.0
/// otherwise (NaN never negates). Returned in ST0.
///
/// Original: 0x00cf96f0 (thiscall: ecx holds the object, no stack words).
lf_checker_rt::export!(thiscall, rw_00cf96f0(this: u32) -> f32 {
    unsafe {
        const SCALE_ADDR: u32 = 0x01051528; // 1.0039153f
        const ZERO_ADDR: u32 = 0x00fe8628; // 0.0f
        const SAMPLE_A: u32 = 1;
        const SAMPLE_B: u32 = 2;
        let state = ((this + 0x60) as *const u32).read_unaligned();
        if state == 0 {
            return 0.0;
        }
        unsafe fn pick(p: u32) -> u32 {
            unsafe {
                let tag = ((p + 0x44) as *const u16).read_unaligned();
                if tag == 1 {
                    ((p + 0x40) as *const u32).read_unaligned()
                } else {
                    0
                }
            }
        }
        let f0 = f32::from_bits(((state + 0x4c) as *const u32).read_unaligned());
        let mut slot1 = [0u32; 3];
        lf_checker_rt::callee_cdecl!(
            SAMPLE_A, u32, &mut slot1 as *mut u32 as u32, f0.to_bits(), pick(state));
        let cached = f32::from_bits(((this + 0x58) as *const u32).read_unaligned());
        let state2 = ((this + 0x60) as *const u32).read_unaligned();
        let mut slot2 = [0u32; 3];
        lf_checker_rt::callee_cdecl!(
            SAMPLE_B, u32, &mut slot2 as *mut u32 as u32, cached.to_bits(), pick(state2));
        let r1 = f32::from_bits(slot1[2]);
        let r2 = f32::from_bits(slot2[2]);
        ((this + 0x58) as *mut u32).write_unaligned(f0.to_bits());
        let scale = f32::from_bits((lf_checker_rt::global::<u32>(SCALE_ADDR)).read_unaligned());
        let mut res = core::hint::black_box(r1) - core::hint::black_box(r2);
        res = core::hint::black_box(res) * core::hint::black_box(scale);
        let tag = ((state2 + 0xc) as *const u32).read_unaligned();
        let zero = f32::from_bits((lf_checker_rt::global::<u32>(ZERO_ADDR)).read_unaligned());
        let negate = if tag == 0x88 || tag == 0x8a {
            res > zero
        } else {
            res < zero
        };
        if negate {
            res = f32::from_bits(res.to_bits() ^ 0x8000_0000);
        }
        res
    }
});
