// original: 0x00afc870 timed_ui_dispatch
//! Timed UI dispatcher: a countdown held in a global is decremented by a
//! scaled amount while it stays positive; once it reaches zero or below, the
//! function fires a two-argument hook, then either returns early, runs a
//! 100-iteration three-argument sequence, or runs a 3-call sequence
//! tail-chained into a follow-up routine, depending on two more globals.

use lf_checker_rt::{callee_cdecl, export, global};

export!(cdecl, rw_afc870(a0: u32, a1: u32, a2: u32) -> u32 {
    unsafe {
        let count = global::<u32>(0x0160014C);
        let seen = count.read() as i32;
        if seen > 0 {
            // Countdown path: countdown -= trunc(scale_a * scale_b).
            // The original converts with x87 fistp under a forced
            // round-toward-zero control word; out-of-range/NaN yields the
            // integer-indefinite pattern. Only the low 32 bits are kept.
            let fa = f32::from_bits(global::<u32>(0x011735BC).read());
            let fb = f32::from_bits(global::<u32>(0x00FE8C58).read());
            let prod = fa * fb;
            let bits: u64 = if prod > -9.223372e18_f32 && prod < 9.223372e18_f32 {
                (prod as i64) as u64
            } else {
                0x8000_0000_0000_0000u64
            };
            let lo = bits as u32;
            count.write(count.read().wrapping_sub(lo));
            lo
        } else {
            // Fired path: hook(hook_arg0, rate * a2 + base).
            let rate = f32::from_bits(global::<u32>(0x0103FF5C).read());
            let base = f32::from_bits(global::<u32>(0x00FE8B38).read());
            let arg = f32::from_bits(a2);
            let scaled = rate * arg + base;
            callee_cdecl!(1, u32, a0, scaled.to_bits());
            let level = global::<u32>(0x01600154).read();
            let limit = global::<u32>(0x0103FFA4).read();
            if (level as i32) >= (limit as i32) {
                return level;
            }
            let phase = global::<u32>(0x01600150);
            let n = phase.read();
            if n == 0 {
                // Three identical worker calls, then tail into the follow-up.
                let f = f32::from_bits(a2).to_bits();
                callee_cdecl!(2, u32, a0, a1, f);
                callee_cdecl!(2, u32, a0, a1, f);
                callee_cdecl!(2, u32, a0, a1, f);
                return callee_cdecl!(3, u32, a0, a1, a2);
            }
            let d = n.wrapping_sub(1);
            phase.write(d);
            if d != 0 {
                return d;
            }
            // Phase just expired: 100 identical worker calls; return the last answer.
            let f = f32::from_bits(a2).to_bits();
            let mut last = 0u32;
            let mut i = 100u32;
            while i != 0 {
                last = callee_cdecl!(2, u32, a0, a1, f);
                i -= 1;
            }
            last
        }
    }
});
