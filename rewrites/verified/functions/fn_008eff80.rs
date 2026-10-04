// original: 0x008eff80 normalized_axis_value
/// Signed-normalized axis value in [-1, 1] from two keyed object bytes:
// ((b6 ^ b4) - 127.5) / 127.5, clamped at both ends.
export!(thiscall, rw_008eff80(this: u32) -> f32 {
    let b6 = unsafe { *((this.wrapping_add(6)) as *const u8) };
    let b4 = unsafe { *((this.wrapping_add(4)) as *const u8) };
    let lo = unsafe { *global::<f32>(0xFE8D94) };
    let c1 = unsafe { *global::<f32>(0xFE8BC8) };
    let c2 = unsafe { *global::<f32>(0xE837F4) };
    let hi = unsafe { *global::<f32>(0xFE88E8) };
    let v = ((b6 ^ b4) as f32 - c1) * c2;
    if lo > v {
        lo
    } else if v > hi {
        hi
    } else {
        v
    }
});
