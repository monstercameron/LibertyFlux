// original: 0x00E41F70 emit_blend_slots
/// Fetch blended floats, fill the slot table, and emit it twice.
///
/// `this` is the owning object: enable byte at +0x44c, mode byte at +0x44d,
/// blend bytes at +0x3cc/+0x395, count bytes at +0x311/+0x313.
///
/// Behaviour: return at once when the enable byte is clear. Otherwise derive
/// two flags from the blend and count bytes (signed comparisons), fetch four
/// float pairs through the table-fetch callee (each call writes two words at
/// its slot address and returns the value address), resolve one count byte
/// through the counter callee when the shared mode byte is set, clamp it
/// against the fetched floats, and lay out two overlapping slot ranges from
/// sums and differences of the fetched floats with two fixed constants. The
/// first range (when the span flag is set) goes to the nine-slot emitter, the
/// second (when the blend flag is set) to the five-slot emitter; both paths
/// run the shared post callee and the frame-cookie check. Returns the cookie
/// check's answer (the only caller ignores it).
///
/// Merge note: the helpers below are part of this rewrite (the merge target
/// can wrap this file in its own module). The `fadd`/`fsub`/`i2f`/`cvtt`
/// helpers must stay `#[inline(never)]` opaque calls in this exact order:
/// inlining them lets LLVM's SLP vectorizer re-pair the reduction order,
/// which keeps values but changes NaN payloads (r-b66's finding).
#[inline(always)]
fn r32(a: u32) -> u32 {
    unsafe { *(a as *const u32) }
}

#[inline(always)]
fn r8(a: u32) -> u8 {
    unsafe { *(a as *const u8) }
}

#[inline(always)]
fn rf(a: u32) -> f32 {
    f32::from_bits(r32(a))
}

#[inline(never)]
fn fadd(a: f32, b: f32) -> f32 {
    a + b
}

#[inline(never)]
fn fsub(a: f32, b: f32) -> f32 {
    a - b
}

/// Signed integer to float, matching cvtdq2ps on the low lane.
#[inline(never)]
fn i2f(x: u32) -> f32 {
    (x as i32) as f32
}

/// Truncating float to int, matching cvttss2si exactly (out-of-range values,
/// infinities and NaNs all yield 0x80000000; Rust's `as` saturates instead).
#[inline(never)]
fn cvtt(x: f32) -> i32 {
    if x.is_nan() || x >= 2147483648.0f32 || x < -2147483648.0f32 {
        i32::MIN
    } else {
        x as i32
    }
}

const C1: u32 = 0x3f7a_e148;
const C2: u32 = 0x3ca3_d70a;

/// Shuffle four floats into the eight emission slots. `x2a` is the difference
/// term before the second addition, `x2b` after it; `x0` is the side sum and
/// `x1` the side term. Both emission blocks share this exact shuffle.
#[inline(always)]
fn fill_slots(f: &mut [u8; 112], x2a: f32, x2b: f32, x1: f32, x0: f32) {
    let w = |f: &mut [u8; 112], off: usize, v: u32| {
        f[off..off + 4].copy_from_slice(&v.to_le_bytes());
    };
    w(f, 0x2c, x2a.to_bits());
    w(f, 0x30, x0.to_bits());
    w(f, 0x34, x2a.to_bits());
    w(f, 0x38, x1.to_bits());
    w(f, 0x3c, x2b.to_bits());
    w(f, 0x40, x0.to_bits());
    w(f, 0x44, x2b.to_bits());
    w(f, 0x48, x1.to_bits());
}

#[inline(always)]
fn slot(f: &[u8; 112], off: usize) -> u32 {
    u32::from_le_bytes([f[off], f[off + 1], f[off + 2], f[off + 3]])
}

#[inline(always)]
fn slotf(f: &[u8; 112], off: usize) -> f32 {
    f32::from_bits(slot(f, off))
}

#[inline(always)]
fn wslot(f: &mut [u8; 112], off: usize, v: u32) {
    f[off..off + 4].copy_from_slice(&v.to_le_bytes());
}

#[inline(always)]
fn body(this: u32) {
    if r8(this.wrapping_add(0x44c)) == 0 {
        return;
    }
    let dl = r8(this.wrapping_add(0x3cc));
    let dh = r8(this.wrapping_add(0x395));
    let lo = r8(this.wrapping_add(0x311)) as u32;
    let hi = r8(this.wrapping_add(0x313)) as u32;
    // Span flag: blend byte above 1 (signed). Blend flag: shifted blend
    // byte below the count span (signed, wrapping span).
    let flag = ((dl as i8) > 1) as u32;
    let bl: u8 = if dh != 0 {
        let span = hi.wrapping_sub(lo).wrapping_add(1);
        (((dl as i8 as i32).wrapping_add(2) < span as i32) as u8)
    } else {
        let span = hi.wrapping_sub(lo);
        (((dl as i8 as i32).wrapping_add(2) < span as i32) as u8)
    };
    // Frame scratch; callee out-param writes land at these same offsets.
    let mut f = [0u8; 112];
    let fp = |f: &[u8; 112], off: usize| -> u32 { &f[off] as *const u8 as u32 };
    callee_cdecl!(1, u32, fp(&f, 0x14), 0x63u32);
    callee_cdecl!(1, u32, fp(&f, 0x1c), 0x64u32);
    callee_cdecl!(2, u32, 3u32, fp(&f, 0x08), fp(&f, 0x10), 0u32);
    // Call 4's lea runs with 32 bytes of uncleared call args below it, so
    // its slot is +0x24 (same slot call 5 refreshes), not +0x12.
    let p4: u32 = callee_cdecl!(1, u32, fp(&f, 0x24), 0x57u32);
    let cvt1 = cvtt(rf(p4));
    // Counter byte, resolved only when the shared mode byte is set.
    let f24: u8 = if r8(relocated(0x011616c8)) != 0 {
        let a: u32 = callee_thiscall!(3, u32, relocated(0x011616c8));
        let esi = (a & 0xff) as u32;
        let p5: u32 = callee_cdecl!(1, u32, fp(&f, 0x24), 0x57u32);
        let x2 = rf(p5);
        let x1 = i2f(esi);
        // Ordered minimum with the original's NaN fallthrough: unordered
        // pairs keep x1, and a positive x1 against +0.0 keeps +0.0.
        let m = if 0.0f32 > x1 {
            0.0f32
        } else if x1 > x2 {
            x2
        } else {
            x1
        };
        cvtt(m) as u8
    } else if r8(this.wrapping_add(0x44d)) == 1 {
        0
    } else {
        cvt1 as u8
    };
    let mut x3 = slotf(&f, 0x1c);
    let mut x2 = slotf(&f, 0x14);
    if flag != 0 {
        // First emission range.
        if bl != 0 {
            x2 = fsub(x2, x3);
            wslot(&mut f, 0x14, x2.to_bits());
        }
        let x1 = slotf(&f, 0x18);
        let x0 = fadd(slotf(&f, 0x20), x1);
        let x2b = fadd(x2, x3);
        fill_slots(&mut f, x2, x2b, x1, x0);
        wslot(&mut f, 0x4c, C1);
        wslot(&mut f, 0x50, C2);
        wslot(&mut f, 0x54, C1);
        wslot(&mut f, 0x58, C1);
        wslot(&mut f, 0x5c, C2);
        wslot(&mut f, 0x60, C2);
        wslot(&mut f, 0x64, C2);
        wslot(&mut f, 0x68, C1);
        callee_thiscall!(4, u32, this);
        let color = ((f24 as u32) << 24) | 0x00ff_ffff;
        wslot(&mut f, 0x0c, color);
        // Slot order matches the original's push-shifted lea targets: each
        // lea compensates the pushes before it, so the addresses are 4 lower
        // per preceding push than the raw lea displacement.
        callee_cdecl!(
            5, u32, fp(&f, 0x2c), fp(&f, 0x34), fp(&f, 0x3c), fp(&f, 0x44), fp(&f, 0x4c),
            fp(&f, 0x54), fp(&f, 0x5c), fp(&f, 0x64), fp(&f, 0x0c)
        );
        callee_cdecl!(6, u32,);
        // Range-2 prelude: reload and re-add (block 1's register terms are
        // dead). This runs only on the block-1 fallthrough; the flag-clear
        // path jumps past it and keeps the initial loads above.
        x2 = slotf(&f, 0x14);
        x3 = slotf(&f, 0x1c);
        x2 = fadd(x2, x3);
        wslot(&mut f, 0x14, x2.to_bits());
    }
    if bl == 0 {
        return;
    }
    // Second emission range.
    let x1 = slotf(&f, 0x18);
    let x0 = fadd(slotf(&f, 0x20), x1);
    let x2b = fadd(x2, x3);
    fill_slots(&mut f, x2, x2b, x1, x0);
    callee_thiscall!(4, u32, this);
    let color = ((f24 as u32) << 24) | 0x00ff_ffff;
    wslot(&mut f, 0x0c, color);
    callee_cdecl!(
        7, u32, fp(&f, 0x2c), fp(&f, 0x34), fp(&f, 0x3c), fp(&f, 0x44), fp(&f, 0x0c)
    );
    callee_cdecl!(6, u32,);
}

export!(thiscall, rw_rb85_41f70(this: u32) -> u32 {
    body(this);
    callee_cdecl!(8, u32,)
});
