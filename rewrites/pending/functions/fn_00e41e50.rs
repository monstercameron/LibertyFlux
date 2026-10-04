// original: 0x00E41E50 stage_selected_floats
/// Resolve four flag-selected values into floats and stage a helper call.
///
/// `this` is the owning object: enable byte at +0x44c, count bytes at
/// +0x311/+0x313, base floats at +0x314/+0x318/+0x31c/+0x43c.
///
/// Behaviour: return at once when the enable byte is clear or when the
/// count span (`[+0x313] - [+0x311] + 1`, signed) is below 3. Otherwise
/// combine the base floats with the low count scaled by `[+0x31c]`, ask the
/// flag callee four times and use each answer to pick one dword from each of
/// two global pairs, convert the four picks to `f32`, scale two of them (one
/// by the combined base, one by a shared constant), stage the results with a
/// tag word in frame slots, and forward the two slot addresses to the staging
/// callee. Returns the staging callee's answer (the only caller ignores it).
///
/// Merge note: the helpers below are part of this rewrite (the merge target
/// can wrap this file in its own module). The `fadd`/`fmul`/`i2f` helpers
/// must stay `#[inline(never)]` opaque calls in this exact order: inlining
/// them lets LLVM's SLP vectorizer re-pair the reduction order, which keeps
/// values but changes NaN payloads (r-b66's finding, applied here).
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
fn fmul(a: f32, b: f32) -> f32 {
    a * b
}

/// Signed integer to float, matching cvtdq2ps on the low lane.
#[inline(never)]
fn i2f(x: u32) -> f32 {
    (x as i32) as f32
}

export!(thiscall, rw_rb85_41e50(this: u32) -> u32 {
    // Gates: enable byte, then a count span of at least 3.
    if r8(this.wrapping_add(0x44c)) == 0 {
        return 0;
    }
    let lo = r8(this.wrapping_add(0x311)) as i32;
    let hi = r8(this.wrapping_add(0x313)) as i32;
    if hi.wrapping_sub(lo).wrapping_add(1) < 3 {
        return 0;
    }
    // Combined base: (([43c] + [314]) + [318]) + float(lo) * [31c].
    let t = fadd(rf(this.wrapping_add(0x43c)), rf(this.wrapping_add(0x314)));
    let t = fadd(t, rf(this.wrapping_add(0x318)));
    let m = fmul(i2f(lo as u32), rf(this.wrapping_add(0x31c)));
    let base = fadd(t, m);
    // Four flag answers select one dword from each global pair.
    let g0 = relocated(0x0105c880);
    let g1 = relocated(0x0105c87c);
    let g2 = relocated(0x0105c884);
    let g3 = relocated(0x0105c888);
    let a0: u32 = callee_cdecl!(1, u32,);
    let pick0 = if a0 & 0xff != 0 { r32(g1) } else { r32(g0) };
    let a1: u32 = callee_cdecl!(1, u32,);
    let pick1 = if a1 & 0xff != 0 { r32(g3) } else { r32(g2) };
    let a2: u32 = callee_cdecl!(1, u32,);
    let pick2 = if a2 & 0xff != 0 { r32(g1) } else { r32(g0) };
    let a3: u32 = callee_cdecl!(1, u32,);
    let pick3 = if a3 & 0xff != 0 { r32(g3) } else { r32(g2) };
    // Staged frame: tag word, scaled picks, and converted picks in order.
    let k = rf(relocated(0x00fe8628));
    let mut frame = [0u32; 5];
    frame[0] = 0x96000000;
    frame[1] = fmul(i2f(pick3), base).to_bits();
    frame[2] = i2f(pick0).to_bits();
    frame[3] = i2f(pick1).to_bits();
    frame[4] = fmul(i2f(pick2), k).to_bits();
    // Argument order matches the original's push order: the scaled-pick slot
    // address is pushed last, so it is the callee's first stack argument.
    callee_cdecl!(2, u32, frame[1..].as_ptr() as u32, frame.as_ptr() as u32)
});
