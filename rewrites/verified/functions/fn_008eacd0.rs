// original: 0x008eacd0 scaled_int_pair_writer
/// Write two scaled integers as floats, with an optional float bias.
///
/// Selects a scale factor by `flag`, then: when `a1` is zero both outputs
/// get `a2 * scale * K`; when `a2` is zero both get `a1 * scale * K`;
/// otherwise each gets its own integer scaled plus the shared `a3 * K`
/// bias. Returns `out2`.
export!(cdecl, rw_008eacd0(
    a1: i32,
    a2: i32,
    a3: f32,
    out1: *mut f32,
    out2: *mut f32,
    flag: u32,
) -> u32 {
    unsafe {
        let scale: f32 = if flag & 0xFF != 0 {
            *global::<f32>(0xFE8AB8)
        } else {
            *global::<f32>(0xE833D4)
        };
        let kk: f32 = *global::<f32>(0xFE8830);
        if a1 == 0 {
            let v = a2 as f32 * scale * kk;
            *out1 = v;
            *out2 = v;
        } else if a2 == 0 {
            let v = a1 as f32 * scale * kk;
            *out1 = v;
            *out2 = v;
        } else {
            let t = a3 * kk;
            *out1 = a2 as f32 * scale + t;
            *out2 = a1 as f32 * scale + t;
        }
        out2 as u32
    }
});
