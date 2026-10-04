// original: 0x008efed0 scaled_axis_value
/// Scaled axis value for selector 0/1: when the normalized axis magnitude
/// clears a dead-zone threshold, the selected global counter divided by 128;
/// otherwise 0. Any other selector yields 0.
export!(cdecl, rw_008efed0(ptr: u32, sel: u32) -> f32 {
    let g0 = unsafe { *global::<u32>(0x18B7A68) };
    let g1 = unsafe { *global::<u32>(0x18B7A6C) };
    if sel == 0 {
        if g0 == 0 {
            return 0.0;
        }
    } else if sel == 1 {
        if g1 == 0 {
            return 0.0;
        }
    } else {
        return 0.0;
    }
    let b6 = unsafe { *((ptr.wrapping_add(6)) as *const u8) };
    let b4 = unsafe { *((ptr.wrapping_add(4)) as *const u8) };
    let lo = unsafe { *global::<f32>(0xFE8D94) };
    let c1 = unsafe { *global::<f32>(0xFE8BC8) };
    let c2 = unsafe { *global::<f32>(0xE837F4) };
    let hi = unsafe { *global::<f32>(0xFE88E8) };
    let v = ((b6 ^ b4) as f32 - c1) * c2;
    let vc = if lo > v { lo } else if v > hi { hi } else { v };
    let thresh = unsafe { *global::<f32>(0xFE86E8) };
    if vc.abs() <= thresh {
        return 0.0;
    }
    let scale = unsafe { *global::<f32>(0xFE86FC) };
    let n = if sel == 0 { g0 as i32 } else { g1 as i32 } as f32;
    n * scale
});
