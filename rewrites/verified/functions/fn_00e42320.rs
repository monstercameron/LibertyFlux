// original: 0x00E42320 emit_staged_blocks
/// Run four staged flag blocks, then color and emit the fetched floats.
///
/// `this` is the owning object (mode byte at +0x44d; +0x452 is passed by
/// address to the final emitter).
///
/// Behaviour: resolve one counter byte through the counter callee when the
/// shared mode byte is set (else 0 or 0xff from the mode byte), storing it
/// for the later color high byte. Then run four staging blocks: each asks
/// the flag callee two or three times, picks dwords from two global pairs
/// by the answers, scales table floats by the picks, and forwards two slot
/// addresses to the staging callee (constant slots plus one fetched word).
/// Finally combine the counter byte with a masked table word into a color,
/// fetch four float pairs, run two reducers over them, and emit the results
/// through the fixed six-call chain, subtracting one fetch from another when
/// the next-to-last call answers above 1. Returns the last call's answer
/// (the only caller ignores it).
///
/// The per-block scaled floats (except one forwarded slot) are Verified
/// dead: the slots they are stored to are never read again (slot-liveness
/// analysis in the lane notes), so only the picks, the calls and the
/// observed slots are reproduced here.
///
/// Merge note: the helpers below are part of this rewrite (the merge target
/// can wrap this file in its own module). The `fmul`/`fsub`/`i2f` helpers
/// must stay `#[inline(never)]` opaque calls in this exact order: inlining
/// them lets LLVM's SLP vectorizer re-pair the reduction order, which keeps
/// values but changes NaN payloads (r-b66's finding).
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
fn fmul(a: f32, b: f32) -> f32 {
    a * b
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

#[inline(always)]
fn slot(f: &[u8; 84], off: usize) -> u32 {
    u32::from_le_bytes([f[off], f[off + 1], f[off + 2], f[off + 3]])
}

#[inline(always)]
fn slotf(f: &[u8; 84], off: usize) -> f32 {
    f32::from_bits(slot(f, off))
}

#[inline(always)]
fn wslot(f: &mut [u8; 84], off: usize, v: u32) {
    f[off..off + 4].copy_from_slice(&v.to_le_bytes());
}

export!(thiscall, rw_rb85_42320(this: u32) -> u32 {
    callee_cdecl!(1, u32,);
    callee_cdecl!(2, u32,);
    // Counter byte for the later color high byte.
    let f80: u8 = if r8(relocated(0x01161668)) != 0 {
        callee_thiscall!(3, u32, relocated(0x01161668)) as u8
    } else if r8(this.wrapping_add(0x44d)) == 1 {
        0
    } else {
        0xff
    };
    // Frame slots (offsets match the original's esp0-relative layout shifted
    // by +92; only slots the calls observe are materialized).
    let mut f = [0u8; 84];
    let fp = |f: &[u8; 84], off: usize| -> u32 { &f[off] as *const u8 as u32 };
    let g0 = relocated(0x0105c880);
    let g1 = relocated(0x0105c87c);
    let g2 = relocated(0x0105c884);
    let g3 = relocated(0x0105c888);
    // Block 1: fetch word, two picks, stage constants.
    let p41: u32 = callee_cdecl!(4, u32, fp(&f, 16), 0x41u32);
    wslot(&mut f, 8, r32(p41));
    wslot(&mut f, 16, 0xff000000);
    let a0: u32 = callee_cdecl!(5, u32,);
    let _pk0 = if a0 & 0xff != 0 { r32(g3) } else { r32(g2) };
    let a1: u32 = callee_cdecl!(5, u32,);
    let _pk1 = if a1 & 0xff != 0 { r32(g1) } else { r32(g0) };
    callee_cdecl!(6, u32, fp(&f, 52), 0u32);
    wslot(&mut f, 60, 0);
    callee_cdecl!(7, u32, fp(&f, 60), fp(&f, 16));
    // Block 2: three picks, two fetches, stage the fetched word.
    let a2: u32 = callee_cdecl!(5, u32,);
    let _pk2 = if a2 & 0xff != 0 { r32(g1) } else { r32(g0) };
    let a3: u32 = callee_cdecl!(5, u32,);
    let _pk3 = if a3 & 0xff != 0 { r32(g3) } else { r32(g2) };
    let a4: u32 = callee_cdecl!(5, u32,);
    let _pk4 = if a4 & 0xff != 0 { r32(g1) } else { r32(g0) };
    callee_cdecl!(6, u32, fp(&f, 52), 0u32);
    callee_cdecl!(6, u32, fp(&f, 60), 0u32);
    wslot(&mut f, 76, 0);
    callee_cdecl!(7, u32, fp(&f, 76), fp(&f, 8));
    // Block 3: three picks, one fetch, stage constants.
    wslot(&mut f, 16, 0xff000000);
    let a5: u32 = callee_cdecl!(5, u32,);
    let _pk5 = if a5 & 0xff != 0 { r32(g1) } else { r32(g0) };
    let a6: u32 = callee_cdecl!(5, u32,);
    let _pk6 = if a6 & 0xff != 0 { r32(g3) } else { r32(g2) };
    let a7: u32 = callee_cdecl!(5, u32,);
    let _pk7 = if a7 & 0xff != 0 { r32(g1) } else { r32(g0) };
    callee_cdecl!(6, u32, fp(&f, 60), 0x16u32);
    callee_cdecl!(7, u32, fp(&f, 76), fp(&f, 16));
    // Block 4: three picks, two fetches, stage the fetched word. The second
    // fetch's scaled pick is the slot the color fetch reads below.
    let a8: u32 = callee_cdecl!(5, u32,);
    let _pk8 = if a8 & 0xff != 0 { r32(g1) } else { r32(g0) };
    let a9: u32 = callee_cdecl!(5, u32,);
    let _pk9 = if a9 & 0xff != 0 { r32(g3) } else { r32(g2) };
    let a10: u32 = callee_cdecl!(5, u32,);
    let pk10 = if a10 & 0xff != 0 { r32(g1) } else { r32(g0) };
    let q0: u32 = callee_cdecl!(6, u32, fp(&f, 60), 0x16u32);
    callee_cdecl!(6, u32, fp(&f, 52), 0x16u32);
    wslot(&mut f, 16, fmul(i2f(pk10), rf(q0)).to_bits());
    wslot(&mut f, 76, 0);
    callee_cdecl!(7, u32, fp(&f, 76), fp(&f, 8));
    // Mode select and color assembly.
    let mode: u32 = if r8(relocated(0x0116c250)) == 0x6a || r8(relocated(0x0116c253)) != 0 {
        2
    } else {
        7
    };
    callee_cdecl!(8, u32, mode);
    callee_cdecl!(9, u32, 0u32);
    let esi = (f80 as u32) << 24;
    callee_cdecl!(10, u32, esi);
    let p42: u32 = callee_cdecl!(4, u32, fp(&f, 16), 0x3eu32);
    let color = (r32(p42) & 0x00ff_ffff) | esi;
    wslot(&mut f, 8, color);
    callee_cdecl!(11, u32, color);
    // Tail float fetches and reduces.
    callee_cdecl!(6, u32, fp(&f, 36), 0x86u32);
    callee_cdecl!(6, u32, fp(&f, 44), 0x87u32);
    callee_cdecl!(12, u32, 2u32, fp(&f, 36), 0u32, 0u32);
    callee_cdecl!(6, u32, fp(&f, 20), 0x72u32);
    callee_cdecl!(6, u32, fp(&f, 28), 0x73u32);
    callee_cdecl!(13, u32, 2u32, fp(&f, 20), fp(&f, 28), 0u32);
    // Fixed emission chain.
    callee_cdecl!(14, u32, slot(&f, 48));
    callee_cdecl!(15, u32, slot(&f, 28), slot(&f, 32));
    callee_cdecl!(16, u32, 1u32);
    callee_cdecl!(17, u32, slot(&f, 20), slot(&f, 36));
    let eret: u32 = callee_cdecl!(18, u32, slot(&f, 20), slot(&f, 36), this.wrapping_add(0x452), 0u32);
    let tail = if (eret as i32) > 1 {
        fsub(slotf(&f, 24), slotf(&f, 44)).to_bits()
    } else {
        slot(&f, 24)
    };
    // Note: ebp still carries +0x452 here (added before the previous call
    // and never restored), so this address matches that call's third word.
    callee_cdecl!(19, u32, slot(&f, 20), tail, this.wrapping_add(0x452), 0xffff_ffffu32, 0xffff_ffffu32)
});
