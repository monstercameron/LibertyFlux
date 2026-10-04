// original: 0x008d2160 vec_distance_to_anchor
/// Distance from an anchor point to a target point. The target is the
/// object's linked vector plus 0x30 when the link at +0x20 is set, otherwise
/// the inline vector at +0x10; the anchor is the vector at +0x200. Returns
/// the Euclidean distance as a float.
export!(thiscall, rw_008d2160(this_: u32) -> f32 {
    unsafe {
        let link = *((this_ + 0x20) as *const u32);
        let target = if link != 0 { link + 0x30 } else { this_ + 0x10 };
        let dx = fsub(*(target as *const f32), *((this_ + 0x200) as *const f32));
        let dy = fsub(*((target + 4) as *const f32), *((this_ + 0x204) as *const f32));
        let dz = fsub(*((target + 8) as *const f32), *((this_ + 0x208) as *const f32));
        // Sum order matches the original: (dy*dy + dx*dx) + dz*dz.
        fsqrt(fadd(fadd(fmul(dy, dy), fmul(dx, dx)), fmul(dz, dz)))
    }
});
