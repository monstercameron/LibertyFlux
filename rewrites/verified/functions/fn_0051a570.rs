// original: 0x0051A570 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Race14NoHolds, player_schema::LeaderboardInfo, 10>::vf14

/// Collect one leaderboard's rows for a stat query (Race14NoHolds instantiation).
///
/// `this` is the leaderboard-info object; its virtual slot `+VF_PICK` chooses
/// one distinguished row index and slot `+VF_ROW` maps each row number to a
/// key. `cursor`/`size` bound a byte range (`limit = size + cursor`, wrapping,
/// compared unsigned). `id_out` receives 8 bytes copied from the distinguished
/// row's item (`+ITEM_DATA`), `mask_out` a two-word bit mask with one bit per
/// written row, `flag_out` a single status byte. `board` is passed through as
/// the object of the row/key/item/write helpers. `BOARD_ID` is this
/// instantiation's constant tag.
///
/// Algorithm: zero the outputs, fetch the distinguished index and the key
/// table (three-word info block, table pointer at `+INFO_TABLE`), then scan
/// rows 0..`ROWS`: skip rows the skip-helper rejects; classify the row's cell
/// (`1|2|3|5` advance the cursor by `ELEM`, anything else by 0); on the
/// distinguished row copy the item bytes and set the flag; otherwise advance
/// the cursor (past the limit fails the row) and record the row's bit. Any
/// failed row clears the status and ends the scan; the return is 1 only if
/// every visited row succeeded.
///
/// Edge cases: a null item pointer, an item length above `ELEM`, a rejected
/// write and a cursor past the limit all end the scan with status 0. The
/// row counter never reaches 7, so the mask's high word is always 0 and the
/// original's above-32 bit-index handling is dead code. Only the low byte of
/// the return is meaningful; the upper bytes are callee leftovers.
///
/// Original: 0x0051A570 (thiscall, ECX = this, six stack words, callee pops
/// 0x18; one fastcall helper taking ECX = tag and EDX = info block).
lf_checker_rt::export!(thiscall, rw_0051A570(this: u32, cursor: u32, id_out: u32, mask_out: u32, flag_out: u32, board: u32, size: u32) -> u32 {
    unsafe {
        const BOARD_ID: u32 = 0x6b;
        const VF_PICK: u32 = 0x2c;
        const VF_ROW: u32 = 0x30;
        const INFO_TABLE: u32 = 8;
        const ITEM_DATA: u32 = 4;
        const ROWS: u32 = 7;
        const ELEM: u32 = 8;
        const C_PICK: u32 = 1;
        const C_ROW: u32 = 2;
        const C_INFO: u32 = 3;
        const C_CLASS: u32 = 4;
        const C_SKIP: u32 = 5;
        const C_ITEM: u32 = 6;
        const C_LEN: u32 = 7;
        const C_WRITE: u32 = 8;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn wr8(a: u32, v: u8) {
            unsafe { (a as *mut u8).write(v) }
        }

        wr32(mask_out, 0);
        wr32(mask_out.wrapping_add(4), 0);
        wr8(flag_out, 0);
        let limit = size.wrapping_add(cursor);
        let mut pos = cursor;
        let vt = rd32(this);
        let pick: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(rd32(vt.wrapping_add(VF_PICK)) as usize);
        let picked = pick(this);
        let _ = C_PICK;
        let mut info = [0u32; 3];
        info[(INFO_TABLE / 4) as usize] = 0;
        let ok: u32 =
            lf_checker_rt::callee_fastcall!(C_INFO, u32, BOARD_ID, info.as_mut_ptr() as u32);
        if (ok as u8) == 0 {
            return 0;
        }
        let rowof: extern "thiscall" fn(u32, u32) -> u32 =
            core::mem::transmute(rd32(vt.wrapping_add(VF_ROW)) as usize);
        let _ = C_ROW;
        let mut bl: u8 = 1;
        let mut row: u32 = 0;
        while row < ROWS {
            if bl == 0 {
                break;
            }
            let key = rowof(this, row);
            let skip: u32 = lf_checker_rt::callee_thiscall!(C_SKIP, u32, board, key);
            if (skip as u8) == 0 {
                let table = info[(INFO_TABLE / 4) as usize];
                let cell = rd32(table.wrapping_add(key.wrapping_mul(4)));
                let class: u32 = lf_checker_rt::callee_thiscall!(C_CLASS, u32, cell);
                let adv: u32 = match class {
                    1 | 2 | 3 | 5 => ELEM,
                    _ => 0,
                };
                if picked == row {
                    let item: u32 = lf_checker_rt::callee_thiscall!(C_ITEM, u32, board, key);
                    bl = 0;
                    if item != 0 {
                        let len: u32 = lf_checker_rt::callee_thiscall!(C_LEN, u32, item);
                        if len <= ELEM {
                            let v = (item.wrapping_add(ITEM_DATA) as *const u64).read_unaligned();
                            (id_out as *mut u64).write_unaligned(v);
                            bl = 1;
                        }
                    }
                    wr8(flag_out, bl);
                } else {
                    let old = pos;
                    pos = pos.wrapping_add(adv);
                    if pos > limit {
                        bl = 0;
                    } else {
                        let good: u32 =
                            lf_checker_rt::callee_thiscall!(C_WRITE, u32, board, key, old, adv);
                        if (good as u8) == 0 {
                            bl = 0;
                        } else {
                            wr32(mask_out, 1u32 << row);
                            wr32(mask_out.wrapping_add(4), 0);
                            bl = 1;
                        }
                    }
                }
            }
            row += 1;
        }
        bl as u32
    }
});
