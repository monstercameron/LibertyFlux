// original: 0x00ad0450 update_gated_filter_144
/// Update the filtered value at 0x144 when flag bit 15 allows it.
///
/// Backs up 0x144 to 0x154 and clears 0x144. When the gate bit is set, a
/// child factor selected by the sign of the field at 0x2c (non-positive
/// selects the first, positive or NaN-aware the second) is multiplied by the
/// argument back into 0x144. Flag bit 2 is then set exactly when the argument
/// is nonzero. Returns the leftover register value with the compare flags
/// folded into the second byte, as the original leaves them.
export!(thiscall, rw_00ad0450(this: *mut u8, scale: f32) -> u32 {
    unsafe {
        let flags = *(this.add(0x164) as *const u32);
        *(this.add(0x154) as *mut u32) = *(this.add(0x144) as *const u32);
        *(this.add(0x144) as *mut u32) = 0;
        let base: u32;
        if flags & 0x8000 != 0 {
            let x = *(this.add(0x2c) as *const f32);
            let child = *(this.add(0x14) as *const u32) as *const u8;
            // Below-or-unordered selects the second factor.
            let sel = if !(x >= 0.0) {
                *(child.add(0x3c) as *const f32)
            } else {
                *(child.add(0x38) as *const f32)
            };
            *(this.add(0x144) as *mut f32) = sel * scale;
            base = child as u32;
        } else {
            base = flags >> 15;
        }
        let mut f = flags & !4;
        // The original's flag dance sets bit 2 for every argument that does
        // not compare equal to zero, including NaN.
        if scale != 0.0 {
            f |= 4;
        }
        *(this.add(0x164) as *mut u32) = f;
        // Second byte the original loads from the comparison flags.
        let ah: u32 = if scale > 0.0 {
            0x02
        } else if scale == 0.0 {
            0x42
        } else if scale < 0.0 {
            0x03
        } else {
            0x47
        };
        (base & 0xFFFF00FF) | (ah << 8)
    }
});
