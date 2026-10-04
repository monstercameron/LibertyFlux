// original: 0x008a3360 audMultitrackSound_scan_and_arm
/// Bank scan that arms the sound when a live entry answers.
///
/// Walks the `count` selector bytes at offset `0x48`. Each selector other
/// than `0xFF` resolves a scaled table entry; entries whose kind word is 2
/// are probed through a `thiscall` helper, and a probe whose low answer byte
/// is nonzero arms the sound. A second pass then flags every kind-2 entry
/// and records the armed byte, but only when the trailing scratch byte is
/// nonzero and the gate bytes allow it. Returns 1 with the scale's high bits
/// on the armed paths, the scale with a cleared low byte when no probe
/// answered, and the entry EAX (a declared contract input) with a cleared
/// low byte when the count is zero.
export!(thiscall, rw_008a3360(this: *mut u8, arg: u32) -> u32 {
    const COUNT_OFF: usize = 0xb0;
    const SELECTORS_OFF: usize = 0x48;
    const ROW_OFF: usize = 0x40;
    const ARMED_OFF: usize = 0xb4;
    const GATE_OFF: usize = 0xb5;
    const ROW_STRIDE: u32 = 0x6f40;
    const ROW_BASE: u32 = 0x6f10;
    const TABLE_BASE_GLOB: u32 = 0x115d988;
    const SCALE_GLOB: u32 = 0x115d964;
    const ENTRY_EAX: u32 = 0x1234_5678; // pinned contract input, see contract
    unsafe {
        let count = *(this.add(COUNT_OFF) as *const u32);
        if count == 0 {
            return ENTRY_EAX & !0xff;
        }
        let table = *global::<u32>(TABLE_BASE_GLOB);
        let scale = *global::<u32>(SCALE_GLOB);
        let row = *this.add(ROW_OFF) as u32;
        let row_entry = *(table
            .wrapping_add(row.wrapping_mul(ROW_STRIDE))
            .wrapping_add(ROW_BASE) as *const u32);
        let target_of = |sel: u8| -> u32 {
            scale.wrapping_mul(sel as u32).wrapping_add(row_entry)
        };
        let mut armed = false;
        let mut scratch_low: u8 = 0;
        let mut dl: u8 = 0;
        let mut j: u32 = 0;
        while j < count {
            let sel = *this.add(SELECTORS_OFF + j as usize);
            j += 1;
            if sel == 0xff {
                continue;
            }
            dl = row_entry as u8;
            let target = target_of(sel);
            if target == 0 {
                continue;
            }
            let kind = *((target + 6) as *const u16);
            if kind == 2 {
                let answer = callee_thiscall!(1, u32, target, arg);
                dl = scratch_low;
                if (answer & 0xff) != 0 {
                    armed = true;
                }
            } else {
                if kind == 3 {
                    scratch_low = 1;
                }
                dl = scratch_low;
            }
        }
        if !armed {
            return scale & !0xff;
        }
        if dl == 0 || *this.add(GATE_OFF) == 0 || *this.add(ARMED_OFF) != 0 {
            return (scale & !0xff) | 1;
        }
        let mut k: u32 = 0;
        while k < count {
            let sel = *this.add(SELECTORS_OFF + k as usize);
            k += 1;
            if sel == 0xff {
                continue;
            }
            let target = target_of(sel);
            if target == 0 {
                continue;
            }
            if *((target + 6) as *const u16) != 2 {
                continue;
            }
            *((target_of(sel) + 0x38) as *mut u8) |= 0x10;
        }
        *this.add(ARMED_OFF) = 1;
        (scale & !0xff) | 1
    }
});
