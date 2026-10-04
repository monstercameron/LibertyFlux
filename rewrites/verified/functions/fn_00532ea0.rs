// original: 0x00532EA0 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Race44Standard, player_schema::LeaderboardInfo, 10>::vf14

/// Fetch up to five leaderboard rows into the caller's buffers.
///
/// `this` is the leaderboard-info object; slot 11 of its table answers the
/// wanted row (`wanted`), slot 12 maps a loop counter to a row index. `rows`
/// points at a `row_cap`-byte buffer that receives one 8-byte row per wanted
/// miss, `row_copy` takes an 8-byte copy on a wanted hit, `hit_mask` is two
/// flag words, `wrote_flag` is one flag byte, `store` is the row store passed
/// to the row helpers, and `row_cap` sizes the buffer (`rows + row_cap` is
/// the end, wrapping).
///
/// Both flag outputs start cleared. The info helper (fastcall of the table
/// id 0x8b and a scratch struct) must answer true or the function returns
/// that answer at once. Otherwise it loops `i` in 0..5 while the status byte
/// stays true: helper 4 accepts the row (status kept, next row), or helper 5
/// classifies it (answers 1, 2, 3 or 5 mean a full 8-byte row, anything else
/// an empty one). On a wanted hit (`wanted == i`) helper 6 fetches the entry
/// and, when it is non-null with size at most 8, its 8 payload bytes move to
/// `row_copy` and the status is set; the flag byte records the status either
/// way. On a miss the cursor advances past the row, must stay within the end,
/// and helper 8 stores the row; success sets bit `i` of `hit_mask` (low word
/// for rows below 32, high word at and past it) and the status, failure
/// clears the status. The caller slot holding `rows` is refreshed only on
/// the miss path. The return is the status byte in the low 8 bits over
/// whatever the last executed path left in the register: the info answer on
/// the early path, the flag pointer on a wanted hit, the mask pointer on a
/// store, the classify answer minus one (the dispatch decrements first) on a
/// bounds failure, or the last accept/store answer otherwise.
///
/// Original: 0x00532EA0 (thiscall, six stack words).
lf_checker_rt::export!(thiscall, rw_00532EA0(this: u32, rows: u32, row_copy: u32, hit_mask: u32, wrote_flag: u32, store: u32, row_cap: u32) -> u32 {
    unsafe {
        const TABLE_ID: u32 = 0x8b;
        const ROWS: u32 = 5;
        const FULL_ROW: u32 = 8;
        const VT_WANTED: u32 = 0x2c;
        const VT_INDEX: u32 = 0x30;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }

        wr32(hit_mask, 0);
        wr32(hit_mask.wrapping_add(4), 0);
        (wrote_flag as *mut u8).write(0);
        let end = row_cap.wrapping_add(rows);
        let mut cursor = rows;
        let mut rows_slot = rows;

        let wanted_of: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(rd32(rd32(this).wrapping_add(VT_WANTED)) as usize);
        let wanted = wanted_of(this);

        let mut info = [0u32; 4];
        let mut eax: u32 =
            lf_checker_rt::callee_fastcall!(2, u32, TABLE_ID, info.as_mut_ptr() as u32);
        if eax & 0xff == 0 {
            return eax;
        }
        let index_of: extern "thiscall" fn(u32, u32) -> u32 =
            core::mem::transmute(rd32(rd32(this).wrapping_add(VT_INDEX)) as usize);

        let mut status: u8 = 1;
        let mut i: u32 = 0;
        loop {
            if status == 0 {
                break;
            }
            let index = index_of(this, i);
            let accept: u32 = lf_checker_rt::callee_thiscall!(4, u32, store, index);
            eax = accept;
            if accept & 0xff == 0 {
                let mut stride: u32 = 0;
                let elem = rd32(info[2].wrapping_add(index.wrapping_mul(4)));
                let class: u32 = lf_checker_rt::callee_thiscall!(5, u32, elem);
                // The original compares against -1, then decrements before
                // dispatching, so EAX holds class - 1 past this point.
                if class == 0xffff_ffff {
                    eax = class;
                } else {
                    eax = class.wrapping_sub(1);
                    if eax <= 4 && eax != 3 {
                        stride = FULL_ROW;
                    }
                }
                if wanted == i {
                    status = 0;
                    let entry: u32 = lf_checker_rt::callee_thiscall!(6, u32, store, index);
                    eax = entry;
                    if entry != 0 {
                        let size: u32 = lf_checker_rt::callee_thiscall!(7, u32, entry);
                        eax = size;
                        if (size as i32) <= FULL_ROW as i32 {
                            let payload = (entry.wrapping_add(4) as *const u64).read_unaligned();
                            (row_copy as *mut u64).write_unaligned(payload);
                            status = 1;
                        }
                    }
                    eax = wrote_flag;
                    (wrote_flag as *mut u8).write(status);
                } else {
                    cursor = cursor.wrapping_add(stride);
                    if cursor > end {
                        status = 0;
                    } else {
                        let stored: u32 =
                            lf_checker_rt::callee_thiscall!(8, u32, store, index, rows_slot, stride);
                        eax = stored;
                        if stored & 0xff == 0 {
                            status = 0;
                        } else {
                            let mut lo = 1u32.wrapping_shl(i);
                            let mut hi = 0u32;
                            if i >= 32 {
                                hi = lo;
                            }
                            lo ^= hi;
                            if i >= 64 {
                                hi = lo;
                            }
                            eax = hit_mask;
                            wr32(hit_mask, lo);
                            wr32(hit_mask.wrapping_add(4), hi);
                            status = 1;
                        }
                    }
                    rows_slot = cursor;
                }
            }
            i = i.wrapping_add(1);
            if i >= ROWS {
                break;
            }
        }
        (eax & 0xffff_ff00) | status as u32
    }
});
