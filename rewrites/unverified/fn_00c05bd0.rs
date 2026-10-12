// original: 0x00C05BD0 stream_gain_to_byte
/// Writes a tagged stream item, computes a level byte from a float input and
/// a double helper, then stores two flag bits and returns the level byte.
lf_checker_rt::export!(thiscall, rw_00c05bd0(this: u32, a0: u32, a1: u32, a2: u32, a3: u32, a4: u32) -> u32 {
    unsafe {
        #[inline(always)]
        unsafe fn rd32(address: u32) -> u32 {
            unsafe { (address as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rd8(address: u32) -> u8 {
            unsafe { (address as *const u8).read() }
        }
        #[inline(always)]
        unsafe fn wr32(address: u32, value: u32) {
            unsafe { (address as *mut u32).write_unaligned(value) }
        }
        #[inline(always)]
        unsafe fn wr8(address: u32, value: u8) {
            unsafe { (address as *mut u8).write(value) }
        }
        #[inline(always)]
        fn fsub(left: f32, right: f32) -> f32 {
            core::hint::black_box(left) - core::hint::black_box(right)
        }
        #[inline(always)]
        fn fmul(left: f32, right: f32) -> f32 {
            core::hint::black_box(left) * core::hint::black_box(right)
        }
        #[inline(always)]
        unsafe fn cvtt_to_low_byte(value: f32) -> u8 {
            unsafe {
                use core::arch::x86::{_mm_cvtt_ss2si, _mm_set_ss};
                _mm_cvtt_ss2si(_mm_set_ss(value)) as u8
            }
        }

        const HELPER_ID: u32 = 1;
        const TAG: u8 = 0x75;
        const SHARED_DWORD_VA: u32 = 0x011735A4;
        const ONE_VA: u32 = 0x00FE88E8;
        const BACK_VA: u32 = 0x00FE8DF8;
        const GAIN_VA: u32 = 0x00FE8740;
        const BASE_VA: u32 = 0x00FE8A78;
        const FULL_VA: u32 = 0x00FE8C08;

        wr8(this, TAG);
        wr32(this + 4, rd32(lf_checker_rt::relocated(SHARED_DWORD_VA)));
        wr32(this + 8, rd32(a0));
        wr32(this + 12, rd32(a0 + 4));
        wr32(this + 16, rd32(a0 + 8));
        wr32(this + 20, a1);

        let input = f32::from_bits(a2);
        let one = f32::from_bits(rd32(lf_checker_rt::relocated(ONE_VA)));
        let level = if !(input >= one) {
            0xFF
        } else {
            let back = f32::from_bits(rd32(lf_checker_rt::relocated(BACK_VA)));
            let distance = fsub(input, back);
            if !(distance >= 0.0) {
                0
            } else {
                let gain = f32::from_bits(rd32(lf_checker_rt::relocated(GAIN_VA)));
                let scaled_input = fmul(input, gain) as f64;
                let base_bits = (lf_checker_rt::relocated(BASE_VA) as *const u64).read_unaligned();
                let answer_bits = lf_checker_rt::callee_cdecl!(
                    HELPER_ID,
                    u64,
                    base_bits as u32,
                    (base_bits >> 32) as u32,
                    scaled_input.to_bits() as u32,
                    (scaled_input.to_bits() >> 32) as u32
                );
                let answer = f64::from_bits(answer_bits);
                let full = f32::from_bits(rd32(lf_checker_rt::relocated(FULL_VA)));
                let scaled_answer = fmul(answer as f32, full);
                cvtt_to_low_byte(scaled_answer)
            }
        };

        wr8(this + 1, level);
        wr8(this + 2, 0);
        wr8(this + 3, 0);
        if (a3 as u8) != 0 {
            wr8(this + 2, 1);
        }
        if (a4 as u8) != 0 {
            wr8(this + 2, rd8(this + 2) | 2);
        }
        level as u32
    }
});
