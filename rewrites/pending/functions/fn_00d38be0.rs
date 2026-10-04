// original: 0x00d38be0 mode_gate
/// Mode gate on the inner object: modes 1 and 3 reject; mode 4 accepts only
/// sub-values 0 and 2; every other mode accepts.
export!(thiscall, rw_00d38be0(this: u32, _a1: u32, a2: u32) -> u32 {
    let inner = unsafe { *((this + 0x18) as *const u32) };
    let mode = unsafe { *((inner + 0x1304) as *const u32) };
    match mode {
        1 | 3 => 0,
        4 => {
            if a2 == 0 || a2 == 2 {
                1
            } else {
                0
            }
        }
        _ => 1,
    }
});
