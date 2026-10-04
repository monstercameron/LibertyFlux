// original: 0x00ad0180 update_gated_filter_140
/// Update the filtered value at 0x140 through a chain of flag gates.
///
/// Backs up 0x140 to 0x150, then takes the fast path only when flag bit 15
/// is set, the first two arguments clear their thresholds, and either flag
/// bit 12 is set or the first threshold also clears the child's limit: the
/// field is zeroed and flag bit 24 is set (or preserved). Otherwise a child
/// factor selected by the field at 0x2c is multiplied by the first argument
/// into 0x140 and bit 24 is cleared. Returns a per-path leftover value that
/// the original leaves in the return register.
export!(thiscall, rw_00ad0180(this: *mut u8, a: f32, b: f32, c: f32) -> u32 {
    unsafe {
        let flags = *(this.add(0x164) as *const u32);
        *(this.add(0x150) as *mut u32) = *(this.add(0x140) as *const u32);
        let child = *(this.add(0x14) as *const u32) as *const u8;
        let mut fast = false;
        if flags & 0x8000 != 0 {
            let t1 = *global::<f32>(0x00FE879C);
            if a > t1 && b > *global::<f32>(0x00FE88BC) {
                fast = flags & 0x1000 != 0 || t1 > *(child.add(0x3c) as *const f32);
            }
        }
        if fast {
            *(this.add(0x140) as *mut u32) = 0;
            if *global::<f32>(0x00FE8AB8) > c {
                *(this.add(0x164) as *mut u32) = flags | 0x1000000;
                return if flags & 0x1000 != 0 {
                    flags >> 12
                } else {
                    child as u32
                };
            }
            if flags & 0x1000000 != 0 {
                *(this.add(0x164) as *mut u32) = flags | 0x1000000;
            } else {
                *(this.add(0x164) as *mut u32) = flags & !0x1000000;
            }
            return flags >> 24;
        }
        let x = *(this.add(0x2c) as *const f32);
        // The original jumps on below-or-unordered, i.e. !(x >= limit).
        let sel = if !(x >= *global::<f32>(0x00FE8628)) {
            *(child.add(0x7c) as *const f32)
        } else {
            *(child.add(0x78) as *const f32)
        };
        *(this.add(0x140) as *mut f32) = sel * a;
        *(this.add(0x164) as *mut u32) = flags & !0x1000000;
        child as u32
    }
});
