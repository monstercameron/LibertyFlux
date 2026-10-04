// original: 0x0052A750 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Race13Standard, player_schema::LeaderboardInfo, 10>::vf14

/// Read one row-set of the Race13Standard ranked leaderboard into the caller's buffers.
///
/// `this` is the leaderboard-info object. Virtual slot `+VT_SLOT_SELECTED`
/// answers which of the five rows is selected, slot `+VT_SLOT_ROW` maps a row
/// index to a row handle. `store` is the backing store passed to the row
/// helpers. `value_out` receives the selected row's 8-byte value, `mask_out`
/// a one-hot 8-byte mask of the last committed row, `flag_out` a byte telling
/// whether the selected row was found. `base`/`span` bound the cursor: the
/// cursor starts at `base` and must stay within `base + span` (unsigned).
///
/// Algorithm: clear `mask_out` (8 bytes) and `flag_out` (1 byte), fetch the
/// selected index and the row-cell
/// table for leaderboard `LEADERBOARD_ID`, then visit rows 0..4. A row the
/// store skips keeps the previous state. Otherwise the row's cell kind sets
/// the row size (8 bytes for kinds 1, 2, 3 and 5, else 0). The selected row
/// is looked up and, when found with a value of at most 8 bytes, copied to
/// `value_out`; any other row advances the cursor first (the advance sticks
/// even past the bound), then, when inside the bound and committed by the
/// store, writes its one-hot mask, with the store call seeing the previous
/// cursor. The first failing row
/// clears the status and ends the walk; the return value is that status.
///
/// Edge cases: a failed table lookup returns 0 at once; a null lookup or an
/// over-long value leaves the status cleared; kinds outside 1/2/3/5 advance
/// the cursor by 0; the bound comparison is unsigned.
///
/// Original: 0x0052A750 (thiscall, six stack words, byte result).
lf_checker_rt::export!(thiscall, rw_0052A750(this: u32, base: u32, value_out: u32, mask_out: u32, flag_out: u32, store: u32, span: u32) -> u8 {
    unsafe {
        const LEADERBOARD_ID: u32 = 0x5B;
        const VT_SLOT_SELECTED: u32 = 0x2c;
        const VT_SLOT_ROW: u32 = 0x30;
        const ROWS: u32 = 5;
        const ROW_BYTES: u32 = 8;
        const MAX_VALUE_LEN: u32 = 8;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }

        (mask_out as *mut u64).write_unaligned(0);
        (flag_out as *mut u8).write(0);
        let limit = span.wrapping_add(base);
        let vtab = rd32(this);
        let selected_of: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(rd32(vtab + VT_SLOT_SELECTED) as usize);
        let selected = selected_of(this);
        let mut info = [0u32; 3];
        let ok: u8 =
            lf_checker_rt::callee_fastcall!(1, u8, LEADERBOARD_ID, info.as_mut_ptr() as u32);
        if ok == 0 {
            return 0;
        }
        let cells = rd32(info.as_ptr() as u32 + 8);
        let row_of: extern "thiscall" fn(u32, u32) -> u32 =
            core::mem::transmute(rd32(vtab + VT_SLOT_ROW) as usize);
        let mut alive: u8 = 1;
        let mut cursor = base;
        let mut row: u32 = 0;
        while row < ROWS {
            if alive == 0 {
                break;
            }
            let handle = row_of(this, row);
            let skip: u8 = lf_checker_rt::callee_thiscall!(4, u8, store, handle);
            if skip == 0 {
                let cell = rd32(cells.wrapping_add(handle.wrapping_mul(4)));
                let kind: u32 = lf_checker_rt::callee_thiscall!(5, u32, cell);
                let size: u32 = match kind {
                    1 | 2 | 3 | 5 => ROW_BYTES,
                    _ => 0,
                };
                if selected == row {
                    alive = 0;
                    let found: u32 = lf_checker_rt::callee_thiscall!(6, u32, store, handle);
                    if found != 0 {
                        let len: u32 = lf_checker_rt::callee_thiscall!(7, u32, found);
                        if len <= MAX_VALUE_LEN {
                            (value_out as *mut u64).write_unaligned(((found + 4) as *const u64).read_unaligned());
                            alive = 1;
                        }
                    }
                    (flag_out as *mut u8).write(alive);
                } else {
                    let prev = cursor;
                    cursor = cursor.wrapping_add(size);
                    if cursor > limit {
                        alive = 0;
                    } else {
                        let done: u8 =
                            lf_checker_rt::callee_thiscall!(8, u8, store, handle, prev, size);
                        if done == 0 {
                            alive = 0;
                        } else {
                            (mask_out as *mut u32).write_unaligned(1u32 << row);
                            ((mask_out + 4) as *mut u32).write_unaligned(0);
                            alive = 1;
                        }
                    }
                }
            }
            row += 1;
        }
        alive
    }
});
