// original: 0x008ABF90 rage::audBiquadFilterEffect::vf4
/// Address of the coefficient row selected by the filter's step index.
export!(thiscall, rw_008ABF90(obj: *mut u8) -> u32 {
    unsafe {
        let step = *(obj.add(0x30) as *const u32);
        let row = step.wrapping_add(5);
        let tripled = row.wrapping_add(row.wrapping_mul(2));
        (obj as u32).wrapping_add(tripled.wrapping_mul(8))
    }
});
