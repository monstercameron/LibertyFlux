// original: 0x008f0fb0 axis_gate_b
/// Axis gate B: same gate protocol over its own keyed bytes and window.
export!(thiscall, rw_008f0fb0(this: u32) -> u32 {
    if unsafe { *((this.wrapping_add(0x328D)) as *const u8) } != 0 {
        let key = unsafe { *((this.wrapping_add(0x2F0C)) as *const u8) };
        let hi = unsafe { *((this.wrapping_add(0x2F0E)) as *const u8) } ^ key;
        if hi > 0x7F {
            let lo = unsafe { *((this.wrapping_add(0x2F0F)) as *const u8) } ^ key;
            if lo <= 0x7F {
                return 1;
            }
        }
    }
    if unsafe { *((this.wrapping_add(0x328C)) as *const u8) } == 0 {
        return 0;
    }
    if (callee_thiscall!(1, u32, this) & 0xFF) == 0 {
        0
    } else {
        1
    }
});
