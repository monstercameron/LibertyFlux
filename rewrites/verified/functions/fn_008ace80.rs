// original: 0x008ace80 audio_smoothstep_blend
/// Audio smoothstep blend: clamp to 0..1, shape it, mix `a0` towards `a1`.
///
/// Maps `a4` into a 0..1 factor over the `[a2, a3]` range (an unordered
/// comparison falls through to the ratio, as with the original's `jb`), feeds
/// the factor's bit pattern to the shaped-response helper (thiscall/1
/// returning `f32` on ST0, stubbed), then returns `(a1 - a0) * r + a0`.
export!(thiscall, rw_008ace80(
    this_: *mut u8,
    a0: f32,
    a1: f32,
    a2: f32,
    a3: f32,
    a4: f32,
) -> f32 {
    unsafe {
        let t = if a2 >= a4 {
            0.0
        } else if a4 >= a3 {
            1.0
        } else {
            (a4 - a2) / (a3 - a2)
        };
        let shape: extern "thiscall" fn(*mut u8, u32) -> f32 =
            core::mem::transmute(callee_addr(1) as usize);
        let r = shape(this_, t.to_bits());
        (a1 - a0) * r + a0
    }
});
