// original: 0x00acf190 audio_voice_tick
// 0x00ACF190: audio voice timer update (proposed name: audio_voice_tick).
// Counts down the voice timer at this+0x160 by arg1*200 (clamped to the
// 0..1000 window), then gates on voice energy (|this+0x80| * this+8 > 18)
// and a random roll against the shared RNG at 0x11101A0 (chance 1%).
// When every gate passes it fires three engine calls: a start request on
// this, a six-argument dispatch on the global audio engine object, and a
// tail request on arg0+0x210. Returns nothing meaningful (whatever the
// last callee left in eax on the taken path, entry garbage otherwise).
export!(thiscall, rw_00acf190(this: u32, arg0: u32, arg1: f32) -> u32 {
    unsafe {
        // Gate 1+2: timer must be inside (0, 1000). `comiss`+`jbe` jumps
        // when the ordered `>` is false, which `!(a > b)` reproduces
        // exactly, NaN included.
        let timer = *((this + 0x160) as *const f32);
        if !(timer > 0.0) {
            return 0;
        }
        let cap = *global::<f32>(0x00FE8C58);
        if !(cap > timer) {
            return 0;
        }
        // Count the timer down and store it before the next gate.
        let rate = *global::<f32>(0x00FE8BF4);
        let stepped = timer - arg1 * rate;
        *((this + 0x160) as *mut f32) = stepped;
        if !(rate > stepped) {
            return 0;
        }
        // Gate 3: voice energy above threshold, else park the timer.
        let amp_bits = *((this + 0x80) as *const u32);
        let abs_mask = *global::<u32>(0x00FE8F80);
        let energy = f32::from_bits(amp_bits & abs_mask) * *((this + 8) as *const f32);
        let floor = *global::<f32>(0x00FE8B34);
        if !(energy > floor) {
            *((this + 0x160) as *mut u32) = 0x43480007;
            return 0;
        }
        // Shared 64-bit RNG step: state += state.lo * C (full 64-bit).
        let m1 = *global::<u32>(0x011101A0);
        let m2 = *global::<u32>(0x011101A4);
        let stepped64 = (m1 as u64).wrapping_mul(0x5CDCFAA7).wrapping_add(m2 as u64);
        *global::<u32>(0x011101A0) = stepped64 as u32;
        *global::<u32>(0x011101A4) = (stepped64 >> 32) as u32;
        // Gate 4: low 23 bits scaled to [0,1) must fall below the chance.
        let scale = *global::<f32>(0x00FE864C);
        let roll = ((stepped64 as u32) & 0x7FFFFF) as f32 * scale;
        let chance = *global::<f32>(0x00FE870C);
        if !(chance > roll) {
            *((this + 0x160) as *mut u32) = 0x43480007;
            return 0;
        }
        // All gates passed: fire the three engine calls.
        *((this + 0x160) as *mut u32) = 0;
        let mut scratch1 = [0u32; 2];
        let started: f64 = callee_thiscall!(1, f64, this, arg0, scratch1.as_mut_ptr() as u32);
        let _ = started;
        let near = *((this + 0x16C) as *const f32) >= *global::<f32>(0x00FE8B08);
        let far = *((this + 0x8C) as *const f32) >= *global::<f32>(0x00FE8B90);
        let head = *(this as *const u32);
        let mut scratch2 = [0u32; 2];
        let engine = relocated(0x016D9F58);
        callee_thiscall!(2, u32, engine, this, arg0, scratch2.as_mut_ptr() as u32, head, far as u32, near as u32);
        let tail = arg0.wrapping_add(0x210);
        callee_thiscall!(3, u32, tail, head, 0)
    }
});
