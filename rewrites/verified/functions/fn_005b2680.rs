// original: 0x005B2680 MO_RED
/// Refresh the red menu overlay when its entry list selects it.
///
/// Bails out unless three gate bytes are clear and (for modes `0x31`/`0x3E`)
/// a precondition call accepts and a sub-mode differs from `0x13`. After a
/// seven-argument gate call accepts, rows are counted from shared state
/// (with an extra step for row 7) and the mode's entry list is scanned:
/// a `.` record finishes the list and publishes the overlay (clearing the
/// output word and marking this object), a type-4 record compares the row
/// tag against the overlay name and sends the record's signed word onward,
/// and a type-5 record tail-forwards to the sibling refresh routine.
/// Returns nothing meaningful.
lf_checker_rt::export!(thiscall, rb126_fn3(this: u32) -> u32 {
    // Intercepted callees (ids match the contract's `callees` table).
    const C_PRE: u32 = 1; // precondition, cdecl/0, al result
    const C_GATE: u32 = 2; // gate, cdecl/7 (0xb, six zeros), al result
    const C_SEVEN: u32 = 3; // row-7 step, cdecl/1
    const C_FIN0: u32 = 4; // finish step, cdecl/1
    const C_FIN1: u32 = 5; // finish step, cdecl/1
    const C_FIN2: u32 = 6; // finish step, cdecl/1
    const C_SEND: u32 = 7; // send word, cdecl/3 (0, word, 0)
    const C_FIN3: u32 = 8; // finish step, cdecl/2
    const C_TAIL: u32 = 9; // sibling refresh, tail call (entry in ecx)

    // Globals (file VAs; resolved through the worker's image base).
    const G_A_VA: u32 = 0x01160C3A; // gate byte A
    const G_B_VA: u32 = 0x01160C39; // gate byte B
    const G_MODE_VA: u32 = 0x01160C40; // mode / row index
    const G_SUB_VA: u32 = 0x01160C24; // sub-mode, must differ from 0x13
    const G_C_VA: u32 = 0x01160C3D; // gate byte C
    const G_OUT_VA: u32 = 0x011609EC; // output word, cleared on publish
    const G_TABLE_VA: u32 = 0x019D3390; // row table, 24 bytes per row
    const S_RED_VA: u32 = 0x00F8A560; // "MO_RED"

    const ROW_LEN: u32 = 24;
    const REC_LEN: u32 = 22;
    const ROW_PTR: u32 = 16; // entries pointer within a row
    const ROW_COUNT: u32 = 20; // entry count (word) within a row
    const REC_WORD: u32 = 18; // signed word within a record

    unsafe {
        if *lf_checker_rt::global::<u8>(G_A_VA) != 0 {
            return 0;
        }
        if *lf_checker_rt::global::<u8>(G_B_VA) != 0 {
            return 0;
        }
        let mode = *lf_checker_rt::global::<u32>(G_MODE_VA);
        if mode == 0x31 || mode == 0x3E {
            if lf_checker_rt::callee_cdecl!(C_PRE, u32,) & 0xFF != 0 {
                return 0;
            }
            if *lf_checker_rt::global::<u32>(G_SUB_VA) == 0x13 {
                return 0;
            }
        }
        if *lf_checker_rt::global::<u8>(G_C_VA) != 0 {
            return 0;
        }
        if lf_checker_rt::callee_cdecl!(C_GATE, u32, 0xB, 0, 0, 0, 0, 0, 0) & 0xFF == 0 {
            return 0;
        }
        let mut index = *lf_checker_rt::global::<u32>(G_MODE_VA);
        if index == 7 {
            lf_checker_rt::callee_cdecl!(C_SEVEN, u32, 1);
            index = *lf_checker_rt::global::<u32>(G_MODE_VA);
        }
        let table = lf_checker_rt::relocated(G_TABLE_VA);
        let row = table.wrapping_add(index.wrapping_mul(ROW_LEN));
        let count = *((row.wrapping_add(ROW_COUNT)) as *const u16) as u32;
        let entries = *((row.wrapping_add(ROW_PTR)) as *const u32);
        let mut i = 0u32;
        while i < count {
            let rec = entries.wrapping_add(i.wrapping_mul(REC_LEN));
            let kind = *(rec as *const u8);
            if kind == 0x2E {
                break;
            }
            if kind == 4 {
                // Tag compare against the overlay name, byte by byte.
                let name = lf_checker_rt::relocated(S_RED_VA);
                let mut k = 0u32;
                let eq = loop {
                    let a = *((row.wrapping_add(k)) as *const u8);
                    let b = *((name.wrapping_add(k)) as *const u8);
                    if a != b {
                        break false;
                    }
                    if a == 0 {
                        break true;
                    }
                    let a1 = *((row.wrapping_add(k + 1)) as *const u8);
                    let b1 = *((name.wrapping_add(k + 1)) as *const u8);
                    if a1 != b1 {
                        break false;
                    }
                    if a1 == 0 {
                        break true;
                    }
                    k += 2;
                };
                if eq {
                    lf_checker_rt::callee_cdecl!(C_FIN1, u32, 3);
                    index = *lf_checker_rt::global::<u32>(G_MODE_VA);
                }
                let row2 = table.wrapping_add(index.wrapping_mul(ROW_LEN));
                let entries2 = *((row2.wrapping_add(ROW_PTR)) as *const u32);
                let word = *((entries2
                    .wrapping_add(i.wrapping_mul(REC_LEN))
                    .wrapping_add(REC_WORD)) as *const i16) as i32 as u32;
                lf_checker_rt::callee_cdecl!(C_SEND, u32, 0, word, 0);
                lf_checker_rt::callee_cdecl!(C_FIN2, u32, 0x42);
                lf_checker_rt::callee_cdecl!(C_FIN3, u32, 0, 0);
                return 0;
            }
            if kind == 5 {
                return lf_checker_rt::callee_thiscall!(C_TAIL, u32, rec);
            }
            i += 1;
        }
        lf_checker_rt::callee_cdecl!(C_FIN0, u32, 0);
        lf_checker_rt::callee_cdecl!(C_FIN1, u32, 1);
        lf_checker_rt::callee_cdecl!(C_FIN1, u32, 3);
        lf_checker_rt::callee_cdecl!(C_FIN1, u32, 5);
        *lf_checker_rt::global::<u32>(G_OUT_VA) = 0;
        *((this.wrapping_add(1)) as *mut u8) = 1;
        lf_checker_rt::callee_cdecl!(C_FIN2, u32, 0xA);
        0
    }
});
