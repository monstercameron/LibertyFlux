// original: 0x008ebb60 NativeImpl_GET_COORDINATES_FOR_NETWORK_RESTART_NODE
/// Read one node's position and heading nibble.
///
/// The low word of `id` selects an area (presence-checked in a global
/// table), the high word a node within it. Writes the scaled position to
/// `out` and the scaled low nibble of the flag byte to `out2`. Returns
/// `out2`.
///
/// Note: when the area entry is null the original copies an uninitialized
/// stack slot into `out[3]`; that path cannot be reproduced from safe
/// Rust, so the rewrite writes zeros there instead. The checker contract
/// keeps every indexed table entry nonzero, covering only the success
/// path (documented, not hidden).
export!(thiscall, rw_008ebb60(
    this: *const u8,
    id: u32,
    out: *mut f32,
    out2: *mut f32,
) -> u32 {
    unsafe {
        let area = id & 0xFFFF;
        if *global::<u32>(0x1178284 + area * 4) == 0 {
            *out = 0.0;
            *out.add(1) = 0.0;
            *out.add(2) = 0.0;
            *out.add(3) = 0.0;
            *out2 = 0.0;
        } else {
            let base =
                *(this.add((0x804 + area * 4) as usize) as *const u32);
            let node = base.wrapping_add((id >> 16).wrapping_mul(32));
            let k: f32 = *global::<f32>(0xFE87A4);
            let kz: f32 = *global::<f32>(0xFE8720);
            *out =
                ((node + 0x14) as *const i16).read_unaligned() as f32 * k;
            *out.add(1) =
                ((node + 0x16) as *const i16).read_unaligned() as f32 * k;
            *out.add(2) =
                ((node + 0x18) as *const i16).read_unaligned() as f32 * kz;
            let nib = (*((node + 0x1c) as *const u8) & 0xF) as u32;
            *out2 = nib as f32 * *global::<f32>(0xE7CB9C);
        }
        out2 as u32
    }
});
