// original: 0x00E45AE0 set_leaderboard_coefficients
use lf_checker_rt::{callee_cdecl, callee_thiscall, export, relocated};

/// Truncate an f32 to i32 with exactly the x86 `cvttss2si` semantics: round
/// toward zero, and yield `i32::MIN` for NaN, infinities and out-of-range
/// values (Rust's `as` cast saturates instead, which differs on exactly those
/// inputs).
#[inline]
fn cvttss2si(v: f32) -> i32 {
    if v.is_nan() || v >= 2147483648.0 || v < -2147483648.0 {
        i32::MIN
    } else {
        v as i32
    }
}

#[inline]
unsafe fn rd32(base: u32, off: u32) -> u32 {
    unsafe { ((base + off) as *const u32).read() }
}

#[inline]
unsafe fn wr8(base: u32, off: u32, v: u8) {
    unsafe { ((base + off) as *mut u8).write(v) }
}

#[inline]
unsafe fn wr32(base: u32, off: u32, v: u32) {
    unsafe { ((base + off) as *mut u32).write(v) }
}

// original: 0x00E45AE0 <unnamed>
// Numeric setup for the leaderboard display object: pulls coefficient pairs
// from the config table (callee 1), asks the shared UI state object three
// yes/no questions (callee 2), runs pairs through the combine helper (callees
// 3 and 4, one id per pointer kind), and finishes with a small float mixdown
// whose bits are both stored and returned.
export!(thiscall, rw_e45ae0(this: u32) -> u32 {
    unsafe {
        // Two-word scratch slots. The original reuses a handful of frame
        // slots for many calls, so several logical temporaries alias the same
        // storage; the stub writes through whatever pointer each side passes,
        // so the sharing must match exactly or later reads diverge.
        let mut s36 = [0u32; 2];
        let mut s4 = [0u32; 2];
        let mut s20 = [0u32; 2];
        let mut s12 = [0u32; 2];
        let mut s28 = [0u32; 2];

        // First coefficient -> low byte of the triple at +0x310.
        let p = callee_cdecl!(1u32, u32, s36.as_mut_ptr() as u32, 0x54u32);
        let b0 = cvttss2si(f32::from_bits((p as *const u32).read())) as u8;
        wr8(this, 0x310, b0);

        // Second coefficient: first or second word depending on UI state.
        let flag1 = callee_thiscall!(2u32, u32, relocated(0x118D7F0)) as u8;
        let p = callee_cdecl!(1u32, u32, s36.as_mut_ptr() as u32, 0x55u32);
        let w = if flag1 != 0 {
            (p as *const u32).read()
        } else {
            (p as *const u32).add(1).read()
        };
        wr8(this, 0x311, cvttss2si(f32::from_bits(w)) as u8);
        wr8(this, 0x312, b0.wrapping_mul(3));

        // Pair -> +0x328, combined in place.
        let p = callee_cdecl!(1u32, u32, s36.as_mut_ptr() as u32, 0x56u32);
        wr32(this, 0x328, (p as *const u32).read());
        wr32(this, 0x32C, (p as *const u32).add(1).read());
        callee_cdecl!(3u32, u32, 2u32, this + 0x328, 0u32, 0u32);

        callee_cdecl!(1u32, u32, s4.as_mut_ptr() as u32, 0x58u32);
        let flag2 = callee_thiscall!(2u32, u32, relocated(0x118D7F0)) as u8;
        if flag2 == 0 {
            let p = callee_cdecl!(1u32, u32, s36.as_mut_ptr() as u32, 0x92u32);
            s4[0] = (p as *const u32).read();
            s4[1] = (p as *const u32).add(1).read();
        }
        callee_cdecl!(4u32, u32, 2u32, 0u32, s4.as_mut_ptr() as u32, 0u32);
        let f314 = s4[0];
        callee_cdecl!(1u32, u32, s20.as_mut_ptr() as u32, 0x59u32);
        wr32(this, 0x314, f314);

        callee_cdecl!(4u32, u32, 2u32, 0u32, s20.as_mut_ptr() as u32, 0u32);
        let f320 = s20[1];
        wr32(this, 0x320, f320);
        callee_cdecl!(1u32, u32, s12.as_mut_ptr() as u32, 0x5Au32);

        let flag3 = callee_thiscall!(2u32, u32, relocated(0x118D7F0)) as u8;
        if flag3 == 0 {
            let p = callee_cdecl!(1u32, u32, s36.as_mut_ptr() as u32, 0x93u32);
            s12[0] = (p as *const u32).read();
            s12[1] = (p as *const u32).add(1).read();
        }
        callee_cdecl!(4u32, u32, 2u32, 0u32, s12.as_mut_ptr() as u32, 0u32);
        let f318 = s12[0];
        wr32(this, 0x318, f318);
        wr32(this, 0x31C, f318);
        callee_cdecl!(1u32, u32, s28.as_mut_ptr() as u32, 0x5Bu32);

        callee_cdecl!(4u32, u32, 2u32, 0u32, s28.as_mut_ptr() as u32, 0u32);
        let f324 = s28[1];
        wr32(this, 0x324, f324);
        let p = callee_cdecl!(1u32, u32, s36.as_mut_ptr() as u32, 0x5Eu32);
        wr32(this, 0x340, (p as *const u32).read());
        wr32(this, 0x344, (p as *const u32).add(1).read());
        callee_cdecl!(5u32, u32, 2u32, 0u32, this + 0x340, 0u32);

        let p = callee_cdecl!(1u32, u32, s36.as_mut_ptr() as u32, 0x5Cu32);
        wr32(this, 0x338, (p as *const u32).read());
        wr32(this, 0x33C, (p as *const u32).add(1).read());

        let p = callee_cdecl!(1u32, u32, s36.as_mut_ptr() as u32, 0x85u32);
        wr32(this, 0x330, (p as *const u32).read());
        wr32(this, 0x334, (p as *const u32).add(1).read());
        callee_cdecl!(3u32, u32, 3u32, this + 0x330, 0u32, 0u32);

        // Tail: repack two earlier pairs and run the float mixdown. The
        // operation order is load-bearing (floats do not reassociate).
        let tail_a = rd32(this, 0x328);
        let tail_b = rd32(this, 0x32C);
        let tail_c = rd32(this, 0x330);
        wr32(this, 0x43C, tail_a);
        wr32(this, 0x440, tail_b);
        wr32(this, 0x444, tail_c);
        let mixed = (b0 as f32) * f32::from_bits(rd32(this, 0x324))
            + (f32::from_bits(rd32(this, 0x440)) + f32::from_bits(rd32(this, 0x320)));
        wr32(this, 0x448, mixed.to_bits());
        mixed.to_bits()
    }
});
