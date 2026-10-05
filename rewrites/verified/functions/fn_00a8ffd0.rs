// original: 0x00a8ffd0 stream_masked_min_scan

/// Scans masked rows for the minimum float.
///
/// `gate`'s low byte must be non-zero (a zero low byte takes the null path
/// even for a live value) and the flag at `this+0x76` must be set, or the
/// scan is skipped and 0 is returned. `sel`'s low byte picks the
/// accumulator slot (`this+0x110` when 0, `this+0x114` otherwise), starting
/// at 40.0. Each live row of the table at `this+0xEC` (count = u16 at
/// `this+0xF0`, re-read each step) whose flag at `+0xB1` is clear is
/// bit-tested (`[row+0x70] & (1 << [key+0x900])`); survivors are reported
/// (callee 1, cdecl) as `(row, c, key)` and fold their float at `+0x74`
/// into the slot with `comiss` minimum semantics (a strict greater
/// comparison only, so NaN keeps the old value). The sticky result byte is
/// 1 once any row folds, else the incoming EBP's second byte (the contract
/// fixes incoming EBP and EAX to 0). Always zeroes the word at `this+0x77`
/// and returns the sticky byte in AL over the last loop EAX. One call.
/// Original: 0x00A8FFD0 (thiscall, ECX + four stack words), 186 bytes.
lf_checker_rt::export!(thiscall, rw_00a8ffd0(this: u32, gate: u32, sel: u32, c: u32, key: u32) -> u32 {
    unsafe {
        const TABLE_OFF: u32 = 0xEC;
        const COUNT_OFF: u32 = 0xF0;
        const FLAG_OFF: u32 = 0x76;
        const ACC_A_OFF: u32 = 0x110;
        const ACC_B_OFF: u32 = 0x114;
        const ACC_INIT: f32 = 40.0;
        const TAG_OFF: u32 = 0x77;
        const ROW_MASK_OFF: u32 = 0x70;
        const ROW_VAL_OFF: u32 = 0x74;
        const ROW_SKIP_OFF: u32 = 0xB1;
        const SHIFT_OFF: u32 = 0x900;
        const REPORT: u32 = 1;
        // Sticky result: the incoming-EBP byte, 0 under the contract.
        // `last_eax` tracks EAX: only AL is set at the end, so the upper
        // bytes of the last loop value (or incoming EAX, 0) survive.
        let mut dl: u8 = 0;
        let mut last_eax: u32 = 0;
        let acc = if (sel & 0xFF) == 0 {
            this.wrapping_add(ACC_A_OFF)
        } else {
            this.wrapping_add(ACC_B_OFF)
        };
        // Only the gate's low byte is tested (`cmp byte`).
        if (gate & 0xFF) != 0 {
            (acc as *mut f32).write_unaligned(ACC_INIT);
            if (this.wrapping_add(FLAG_OFF) as *const u8).read() != 0 {
                let mut count =
                    (this.wrapping_add(COUNT_OFF) as *const u16).read_unaligned() as u32;
                if 0u32 < count {
                    let table =
                        (this.wrapping_add(TABLE_OFF) as *const u32).read_unaligned();
                    let mut i = 0u32;
                    loop {
                        last_eax = table;
                        let row = (table.wrapping_add(i.wrapping_mul(4)) as *const u32)
                            .read_unaligned();
                        if row != 0
                            && (row.wrapping_add(ROW_SKIP_OFF) as *const u8).read() == 0
                        {
                            let k = ((key.wrapping_add(SHIFT_OFF)) as *const u32)
                                .read_unaligned();
                            let bit = 1u32.wrapping_shl(k);
                            last_eax = bit;
                            let mask = (row.wrapping_add(ROW_MASK_OFF) as *const u32)
                                .read_unaligned();
                            if (mask & bit) != 0 {
                                let ans: u32 =
                                    lf_checker_rt::callee_cdecl!(REPORT, u32, row, c, key);
                                last_eax = ans;
                                dl = 1;
                                let cand = (row.wrapping_add(ROW_VAL_OFF) as *const f32)
                                    .read_unaligned();
                                let cur =
                                    (acc as *const f32).read_unaligned();
                                // comiss + jbe: store only on strict greater.
                                if cur > cand {
                                    (acc as *mut f32).write_unaligned(cand);
                                }
                            } else {
                                // Reloads the sticky byte (unchanged value).
                                dl = dl;
                            }
                        }
                        count = (this.wrapping_add(COUNT_OFF) as *const u16)
                            .read_unaligned() as u32;
                        i = i.wrapping_add(1);
                        if i >= count {
                            break;
                        }
                    }
                }
                let _ = count;
            }
        }
        (this.wrapping_add(TAG_OFF) as *mut u16).write_unaligned(0);
        (last_eax & 0xFFFF_FF00) | dl as u32
    }
});
