// original: 0x0097AFE0 audio_mix_level
// 0x0097AFE0 audio_mix_level (proposed): thiscall/0, f32 result in ST0.
//
// Blends a clamped rate from the inner object's two level words with a
// helper-supplied sample: the helper fills four frame slots (object pointer,
// sample value, two spares); the sample is scaled by 0.25 or 0.5 according to
// the object's tag word at +0xC, clamped to [0, 1], and averaged with the
// rate. Returns 0.0 when the inner faded flag is clear.
export!(thiscall, rw_97afe0(this: u32) -> f32 {
    unsafe {
        const RATE: f32 = 2.0; // rdata const
        const UNITY: f32 = 1.0;
        const HALF: f32 = 0.5; // rdata const
        const QUARTER: f32 = 0.25; // rdata const
        let inner = *((this as *const u32).add(0x120 / 4));
        if *((inner as *const u8).add(0x219)) == 0 {
            return 0.0;
        }
        // fabs of both level words, then max: fa wins only on strict >,
        // else fb (matches comiss/ja, including NaN).
        let fa = f32::from_bits(*((inner as *const u32).add(0xce8 / 4)) & 0x7fff_ffff);
        let fb = f32::from_bits(*((inner as *const u32).add(0xcec / 4)) & 0x7fff_ffff);
        let mut m = fb;
        if fa > fb {
            m = fa;
        }
        m = m * RATE;
        // Clamp to [0, 1] (NaN passes through, matching ja/jbe).
        if m < 0.0 {
            m = 0.0;
        } else if m > UNITY {
            m = UNITY;
        }
        let bus = *((inner as *const u32).add(0x78 / 4));
        let mut obj: u32 = 0;
        let mut sample: u32 = 0;
        let mut spare0: u32 = 0;
        let mut spare1: u32 = 0;
        let _: u32 = callee_thiscall!(
            1, u32, bus, 0, 4,
            &mut obj as *mut u32 as u32,
            &mut sample as *mut u32 as u32,
            &mut spare0 as *mut u32 as u32,
            &mut spare1 as *mut u32 as u32
        );
        let v = f32::from_bits(sample);
        let mut x = 0.0f32;
        if obj != 0 {
            let tag = *((obj as *const u32).add(3));
            if matches!(tag, 0x2c | 0x2d | 0x2e | 0x2f | 0x39 | 0x3a) {
                x = v * QUARTER;
            } else if matches!(tag, 0x33 | 0x34 | 0x3b | 0x3c) {
                x = v * HALF;
            }
        }
        if x < 0.0 {
            x = 0.0;
        } else if x > UNITY {
            x = UNITY;
        }
        x * HALF + m * HALF
    }
});
