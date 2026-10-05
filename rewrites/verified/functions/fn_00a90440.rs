// original: 0x00a90440 stream_detach_row_cleanup

/// Detaches an object, then clears its row-table cell when present.
///
/// Phase 1 is rw_00a90390 without the prepare call (an idle kind byte
/// returns the incoming EAX, fixed to 0 by the contract). Phase 2 always
/// runs: unless `obj+0x48` is -1 or the kind byte is 0x3F, it reads the
/// signed column at `obj+0x42` and the signed row byte, zeroes the live
/// cell at `rows[row * 160][column]` (`rows` at `this+0xE4`), applies the
/// same flag/state stores and returns the row pointer; otherwise the
/// phase-1 value is returned. One call.
/// Original: 0x00A90440 (thiscall, ECX + one stack word), 204 bytes.
lf_checker_rt::export!(thiscall, rw_00a90440(this: u32, obj: u32) -> u32 {
    unsafe {
        const ARR_OFF: u32 = 0x80;
        const ROWS_OFF: u32 = 0xE4;
        const KIND_OFF: u32 = 0x40;
        const KIND_ACTIVE: u8 = 0x3F;
        const COL_OFF: u32 = 0x42;
        const KEY_OFF: u32 = 0x44;
        const KEY_NONE: u16 = 0xFFFF;
        const LINK_OFF: u32 = 0x48;
        const FLAGS_OFF: u32 = 0x24;
        const FLAGS_KEEP: u32 = 0xF7FF_FFFF;
        const STATE_OFF: u32 = 0x41;
        const STATE_DONE: u8 = 9;
        const SLOT_BASE: u32 = 0x40;
        const STRIDE: u32 = 96;
        const SCAN_N: u32 = 4;
        const ROW_STRIDE: u32 = 160;
        const LOOKUP: u32 = 2;
        // Phase 1.
        let mut ph1: u32 = 0;
        if (obj.wrapping_add(KIND_OFF) as *const u8).read() == KIND_ACTIVE {
            let key = (obj.wrapping_add(KEY_OFF) as *const u16).read_unaligned();
            if key == KEY_NONE {
                ph1 = KEY_NONE as u32;
            } else {
                let sval = (key as i16) as i32 as u32;
                let mut scratch: u32 = 0;
                let mut answer: u32 = KIND_ACTIVE as u32;
                let mut spare: u32 = KIND_ACTIVE as u32;
                let ans: u32 = lf_checker_rt::callee_thiscall!(
                    LOOKUP, u32, this, sval,
                    &mut scratch as *mut u32 as u32,
                    &mut answer as *mut u32 as u32,
                    &mut spare as *mut u32 as u32
                );
                let edx = answer;
                if edx == KIND_ACTIVE as u32 {
                    ph1 = ans;
                } else {
                    let arr = (this.wrapping_add(ARR_OFF) as *const u32).read_unaligned();
                    let base = arr
                        .wrapping_add(edx.wrapping_mul(STRIDE))
                        .wrapping_add(SLOT_BASE);
                    let mut i = 0u32;
                    loop {
                        let slot = base.wrapping_add(i.wrapping_mul(4));
                        if (slot as *const u32).read_unaligned() == obj {
                            ph1 = i.wrapping_add(edx.wrapping_mul(3).wrapping_mul(8));
                            (slot as *mut u32).write_unaligned(0);
                            let flags =
                                (obj.wrapping_add(FLAGS_OFF) as *const u32).read_unaligned();
                            (obj.wrapping_add(FLAGS_OFF) as *mut u32)
                                .write_unaligned(flags & FLAGS_KEEP);
                            (obj.wrapping_add(STATE_OFF) as *mut u8).write(STATE_DONE);
                            break;
                        }
                        i = i.wrapping_add(1);
                        if i >= SCAN_N {
                            ph1 = base.wrapping_add(SCAN_N.wrapping_mul(4));
                            break;
                        }
                    }
                }
            }
        }
        // Phase 2.
        if (obj.wrapping_add(LINK_OFF) as *const u32).read_unaligned() == 0xFFFF_FFFF {
            return ph1;
        }
        let kind = (obj.wrapping_add(KIND_OFF) as *const u8).read();
        if kind == KIND_ACTIVE {
            // `(an instruction of the original)` keeps the upper bytes of the phase-1 value.
            return (ph1 & 0xFFFF_FF00) | kind as u32;
        }
        let col = (obj.wrapping_add(COL_OFF) as *const i16).read_unaligned() as i32;
        let row_idx = (kind as i8) as i32;
        let rows = (this.wrapping_add(ROWS_OFF) as *const u32).read_unaligned();
        let row = (rows.wrapping_add((row_idx.wrapping_mul(ROW_STRIDE as i32)) as u32)
            as *const u32)
            .read_unaligned();
        let cell = row.wrapping_add((col.wrapping_mul(4)) as u32);
        if (cell as *const u32).read_unaligned() != 0 {
            (cell as *mut u32).write_unaligned(0);
            let flags = (obj.wrapping_add(FLAGS_OFF) as *const u32).read_unaligned();
            (obj.wrapping_add(FLAGS_OFF) as *mut u32).write_unaligned(flags & FLAGS_KEEP);
            (obj.wrapping_add(STATE_OFF) as *mut u8).write(STATE_DONE);
        }
        row
    }
});
