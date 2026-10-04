// original: 0x0097B4F0 audio_level_adjust
// 0x0097B4F0 audio_level_adjust (proposed): thiscall/2.
//
// Reads two flag bytes and two bitmask words from the inner object at
// this+0x120, stores a base level into *out_a selected by a helper call and
// the inner mode word, scales *out_b by the gain at this+0xC0, then applies
// step-up and (when both gate bits are set) a helper-driven correction.
// Returns out_b (the exit EAX of the original on every path).
export!(thiscall, rw_97b4f0(this: u32, out_a: *mut f32, out_b: *mut f32) -> u32 {
    unsafe {
        const BASE_MUTED: f32 = -8.0; // 0xC1000000
        const BASE_LOW: f32 = -3.0; // 0xC0400000
        const BASE_FLAT: f32 = -1.5; // 0xBFC00000
        const STEP_UP: f32 = 1.5; // rdata const
        const STEP_DOWN: f32 = 7.0; // rdata const
        const CORR_SCALE: f32 = 2.5; // rdata const
        const UNITY: f32 = 1.0;

        let inner = *((this as *const u32).add(0x120 / 4)) as u32;
        let inner_b = inner as *const u8;
        let alive = callee_thiscall!(1, u32, inner) as u8;
        if alive != 0 {
            *out_a = BASE_MUTED;
        } else {
            // The mode select is sub/dec/je: `mov` preserves flags, so the
            // second je tests dec's ZF, i.e. mode == 4 (not out_b == 0).
            let mode = *((inner as *const u32).add(0xb80 / 4));
            if mode == 3 {
                *out_a = BASE_FLAT;
            } else if mode == 4 {
                *out_a = 0.0;
            } else {
                *out_a = BASE_LOW;
            }
        }
        *out_b = UNITY;
        let idle = *inner_b.add(0x218) == 0;
        let faded = *inner_b.add(0x219) != 0;
        if idle && faded {
            *out_a = *out_a + STEP_DOWN;
        } else {
            *out_a = *out_a + STEP_UP;
        }
        let gain = *((this as *const f32).add(0xc0 / 4));
        *out_b = gain * *out_b;
        if *((this as *const u8).add(0xaf)) & 1 != 0 {
            *out_a = *out_a + STEP_UP;
        }
        let gate_a = *((inner as *const u32).add(0x24 / 4));
        let gate_b = *((inner as *const u32).add(0x118 / 4));
        if gate_a & 0x2000_0000 != 0 && gate_b & 0x0008_0000 != 0 {
            let measured: f32 = callee_thiscall!(2, f32, inner);
            let mut corr = measured * CORR_SCALE;
            if corr > UNITY {
                corr = UNITY;
            }
            let asked = UNITY - corr;
            let applied: f32 = callee_cdecl!(3, f32, asked.to_bits());
            *out_a = *out_a + applied;
        }
        out_b as u32
    }
});
