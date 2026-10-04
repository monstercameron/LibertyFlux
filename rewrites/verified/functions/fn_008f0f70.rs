// original: 0x008f0f70 axis_gate_a
/// Axis gate A: true when the two keyed bytes disagree in sign with the key
/// (first differs, second matches), else the window check decides; the
/// enable flag skips the window check when clear.
export!(thiscall, rw_008f0f70(this: u32) -> u32 {
    if unsafe { *((this.wrapping_add(0x328D)) as *const u8) } != 0 {
        let key = unsafe { *((this.wrapping_add(0x2EFC)) as *const u8) };
        let hi = unsafe { *((this.wrapping_add(0x2EFE)) as *const u8) } ^ key;
        if hi > 0x7F {
            let lo = unsafe { *((this.wrapping_add(0x2EFF)) as *const u8) } ^ key;
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
