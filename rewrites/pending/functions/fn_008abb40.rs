// original: 0x008ABB40 audio_spatial_blend
/// Refresh the sub-object, copy a four-word source vector in, scale its
/// spatial part by the probe routine's answer for the squared length, and
/// store the three trailing values.
export!(thiscall, rw_008ABB40(
    obj: *mut u8,
    target: u32,
    src: *const u32,
    fx: f32,
    fy: f32,
    fz: f32,
) -> () {
    unsafe {
        callee_thiscall!(1, u32, (obj as u32).wrapping_add(0x20), target);
        let x = *(src as *const f32);
        let y = *(src.add(1) as *const f32);
        let z = *(src.add(2) as *const f32);
        let w = *src.add(3);
        *(obj.add(0x10) as *mut f32) = x;
        *(obj.add(0x14) as *mut f32) = y;
        *(obj.add(0x18) as *mut f32) = z;
        *(obj.add(0x1C) as *mut u32) = w;
        let sq = x * x + y * y + z * z;
        let scale: f32 = callee_cdecl!(2, f32, sq.to_bits());
        *(obj.add(0x10) as *mut f32) = x * scale;
        *(obj.add(0x14) as *mut f32) = y * scale;
        *(obj.add(0x18) as *mut f32) = z * scale;
        *(obj as *mut f32) = fx;
        *(obj.add(4) as *mut f32) = fy;
        *(obj.add(8) as *mut f32) = fz;
    }
});
