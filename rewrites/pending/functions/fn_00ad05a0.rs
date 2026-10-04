// original: 0x00ad05a0 update_signed_clamped_13c
/// Store the argument into 0x13c with sign and clamp handling.
///
/// Backs up 0x13c to 0x14c. Flag bit 13 stores the argument as is, bit 14
/// stores its negation, and with neither the field is zeroed. Flag bit 10
/// then ends the update; otherwise bits 11 and 12 pick a clamp direction and
/// an out-of-range value is scaled by 0.75. Returns a per-path leftover.
export!(thiscall, rw_00ad05a0(this: *mut u8, v: f32) -> u32 {
    unsafe {
        let flags = *(this.add(0x164) as *const u32);
        *(this.add(0x14c) as *mut u32) = *(this.add(0x13c) as *const u32);
        if flags & 0x2000 != 0 {
            *(this.add(0x13c) as *mut f32) = v;
        } else if flags & 0x4000 != 0 {
            // The original negates with a sign-bit xor of the constant.
            *(this.add(0x13c) as *mut f32) = -v;
        } else {
            *(this.add(0x13c) as *mut u32) = 0;
            return flags >> 14;
        }
        if flags & 0x400 != 0 {
            return flags >> 10;
        }
        let b11 = flags & 0x800 != 0;
        let b12 = flags & 0x1000 != 0;
        let cur = *(this.add(0x13c) as *const f32);
        if b11 == b12 {
            if cur > *global::<f32>(0x00FE8628) {
                *(this.add(0x13c) as *mut f32) = cur * *global::<f32>(0x00FE888C);
            }
        } else if 0.0 > cur {
            *(this.add(0x13c) as *mut f32) = cur * *global::<f32>(0x00FE888C);
        }
        flags >> 11
    }
});
