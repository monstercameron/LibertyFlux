// original: 0x00872210 crmt_entry_write
//! Ensure the request slot is initialized, then stamp one entry descriptor.
//!
//! When the slot count is not 1 the embedded state at `this+0x10` is rebuilt
//! through callee 1 (the state copy) from a default template holding the
//! shared tune floats, the count is reset to 1 and the fresh entry address
//! is kept at `this+8`. Either way the entry at `this+8` is then stamped
//! with the tag word, the key (`a0`), the weight (`a1`) and the four words
//! from `a2`. Returns the last `a2` word.
/// File VAs of the three shared tune floats consumed by the entry writer.
const ENTRY_TUNE_0: u32 = 0x01B4B2A0;
const ENTRY_TUNE_1: u32 = 0x01B4B2A4;
const ENTRY_TUNE_2: u32 = 0x01B4B2A8;

export!(thiscall, rw_00872210(this: *mut u8, a0: u32, a1: f32, a2: *const u8) -> u32 {
    unsafe {
        let g0 = *global::<f32>(ENTRY_TUNE_0);
        let g1 = *global::<f32>(ENTRY_TUNE_1);
        let g2 = *global::<f32>(ENTRY_TUNE_2);
        let entry: *mut u8;
        if *(this.add(0x0C) as *const u32) != 1 {
            *(this.add(0x0C) as *mut u32) = 0;
            // Default template: kind 1 followed by zeros, tune floats last.
            let mut tpl = [0u32; 11];
            tpl[0] = 1;
            tpl[8] = g0.to_bits();
            tpl[9] = g1.to_bits();
            tpl[10] = g2.to_bits();
            let fresh: u32 = callee_thiscall!(
                1,
                u32,
                this.add(0x10) as u32,
                tpl.as_mut_ptr() as u32
            );
            // Shift-down loop over the slot array: dead in practice because
            // the count was just zeroed, kept for fidelity.
            let mut c = *(this.add(0x0C) as *const u32);
            while c > 0 {
                let o = c.wrapping_mul(4) as usize;
                *(this.add(o.wrapping_add(8)) as *mut u32) =
                    *(this.add(o.wrapping_add(4)) as *const u32);
                c -= 1;
                if c == 0 {
                    break;
                }
            }
            *(this.add(0x0C) as *mut u32) =
                (*(this.add(0x0C) as *const u32)).wrapping_add(1);
            *(this.add(0x08) as *mut u32) = fresh;
            entry = fresh as *mut u8;
        } else {
            entry = *(this.add(0x08) as *const u32) as *mut u8;
        }
        // Note: the tag fields sit at unaligned offsets (+1/+5/+9).
        (entry.add(1) as *mut u32).write_unaligned(0x00010000);
        (entry.add(5) as *mut u32).write_unaligned(0);
        (entry.add(9) as *mut u16).write_unaligned(0);
        *entry.add(0x0B) = 0;
        *(entry.add(0x0C) as *mut u16) = a0 as u16;
        *(entry.add(0x10) as *mut f32) = a1;
        *(entry.add(0x20) as *mut u32) = *(a2.add(0x00) as *const u32);
        *(entry.add(0x24) as *mut f32) = *(a2.add(0x04) as *const f32);
        *(entry.add(0x28) as *mut f32) = *(a2.add(0x08) as *const f32);
        *(entry.add(0x2C) as *mut u32) = *(a2.add(0x0C) as *const u32);
        *(a2.add(0x0C) as *const u32)
    }
});
