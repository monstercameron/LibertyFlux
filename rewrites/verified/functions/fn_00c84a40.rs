// original: 0x00c84a40 scenario_box_union (proposed)

/// Fold the three-float vectors `a` and `b` into the box kept on `this`.
///
/// Writes the component-wise minimum of `a[i]`/`b[i]` at `this[0..3]` and the
/// component-wise maximum at `this[0x10..0x13]` (floats, `i` in 0..3). Each
/// lane is one `comiss` plus a conditional move, so the comparisons are
/// ordered (`false` when either side is NaN) with the exact NaN and signed-zero
/// behaviour of the instructions: the minimum lane keeps `a[i]` only when
/// `b[i] > a[i]`, otherwise `b[i]`; every maximum lane keeps `a[i]` only when
/// `a[i] > b[i]`, otherwise `b[i]`. Plain Rust `>` on `f32` lowers to the same
/// ordered comparison. Returns `a` (EAX still holds the first stack argument).
///
/// Original: thiscall, two stack words (the callee pops 8 bytes).
lf_checker_rt::export!(thiscall, rw_00c84a40(this: u32, a: u32, b: u32) -> u32 {
    unsafe {
        const MIN_OFF: u32 = 0x00;
        const MAX_OFF: u32 = 0x10;
        for i in 0..3u32 {
            let x = f32::from_bits(((a + i * 4) as *const u32).read_unaligned());
            let y = f32::from_bits(((b + i * 4) as *const u32).read_unaligned());
            let lo = if core::hint::black_box(y) > core::hint::black_box(x) { x } else { y };
            let hi = if core::hint::black_box(x) > core::hint::black_box(y) { x } else { y };
            ((this + MIN_OFF + i * 4) as *mut u32).write_unaligned(lo.to_bits());
            ((this + MAX_OFF + i * 4) as *mut u32).write_unaligned(hi.to_bits());
        }
        a
    }
});
