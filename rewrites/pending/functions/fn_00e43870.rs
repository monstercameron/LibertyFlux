// original: 0x00E43870 emit_indexed_quad
use lf_checker_rt::{callee_cdecl, callee_thiscall, export, global, relocated};

/// x86 `cvttss2si` semantics: truncate toward zero; NaN and out-of-range
/// values (including both infinities and exactly +2^31) yield 0x80000000.
/// (Rust `as` saturates instead, so the edges are handled explicitly.)
fn cvttss2si(x: f32) -> i32 {
    if x.is_nan() || x >= 2147483648.0 || x < -2147483648.0 {
        0x80000000u32 as i32
    } else {
        x as i32
    }
}

/// Publish one indexed color quad when the owner is enabled (original 0x00E43870).
///
/// Reads a brightness level through the shared lookup (or takes the level
/// from the flag-refresh call's answer when the global flag is set), clamps
/// it against a second lookup sample, then publishes the index slot together
/// with the caller-supplied corner values and the packed color. Does nothing
/// when the owner's enable flag is clear or the index is negative. The return
/// value is incidental (callers ignore it; the checker compares the rest).
export!(thiscall, rw_00e43870(this: u32, index: i32, a1: f32, a2: f32, a3: f32, a4: f32) -> u32 {
    const ENABLE_OFF: usize = 0x44c;
    const LOOKUP_ID: u32 = 0x84;
    const FLAG: u32 = 0x01161698;
    const WHITE_RGB: u32 = 0x00ff_ffff;
    unsafe {
        if (this as *const u8).add(ENABLE_OFF).read() == 0 || index < 0 {
            return 0;
        }
        // Scratch for the lookup out-param; the original never reads it back.
        let mut scratch = [0u32; 1];
        let out = scratch.as_mut_ptr() as u32;
        let p1 = callee_cdecl!(1, u32, out, LOOKUP_ID);
        let mut level = cvttss2si((p1 as *const f32).read()) as u8;
        if global::<u8>(FLAG).read() != 0 {
            // The original reads AL only after this call, so the callee's
            // answer becomes the level whenever the flag is set.
            level = callee_thiscall!(2, u32, relocated(FLAG)) as u8;
        }
        let p2 = callee_cdecl!(1, u32, out, LOOKUP_ID);
        let fi = level as f32;
        let f2 = (p2 as *const f32).read();
        // Clamp: `ja` after comparing 0.0 with fi can never fire (fi is an
        // exact 0.0..=255.0); the `jbe` below lets fi win ties and unordered
        // (NaN sample) comparisons, otherwise the sample wins.
        let v = if 0.0f32 > fi {
            0.0
        } else if fi <= f2 || f2.is_nan() {
            fi
        } else {
            f2
        };
        let n = cvttss2si(v);
        let sum2 = a3 + a1;
        let color = ((n as u8 as u32) << 24) | WHITE_RGB;
        callee_thiscall!(
            3,
            u32,
            this.wrapping_add((index as u32).wrapping_mul(4)).wrapping_add(8)
        );
        let mut c0 = a1;
        let mut c1 = a1;
        let mut c2 = sum2;
        let mut c3 = sum2;
        let mut c4 = color;
        callee_cdecl!(
            4, u32,
            &mut c0 as *mut f32 as u32,
            &mut c1 as *mut f32 as u32,
            &mut c2 as *mut f32 as u32,
            &mut c3 as *mut f32 as u32,
            &mut c4 as *mut u32 as u32,
        );
        callee_cdecl!(5, u32,)
    }
});
