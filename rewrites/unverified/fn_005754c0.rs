// original: 0x005754C0 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_Race_144, player_schema::LeaderboardInfo, 10>::vf14

/// Ranked-episodic-race leaderboard column query 0x168: resolve up to 19
/// ranked rows for one leaderboard, through scripted engine services.
///
/// `this` is the leaderboard-info object (vtable: slot `0x2c` answers the
/// selected row index, slot `0x30` answers a row token for a 0-based
/// position). `cursor`/`extent` bound the scan (`limit = extent + cursor`,
/// wrapping); `row_out` receives one 8-byte row value, `mask_out` a two-word
/// bit mask, `flag_out` one status byte, `ctx` is an opaque context passed to
/// the services. The column id 0x168 is this instantiation's constant.
///
/// Behaviour: zero the mask and flag; ask slot `0x2c` for the selected index
/// `v`; query the column (0x168) for a descriptor whose third word is a
/// table pointer (bail out returning 0 when it refuses). Then for positions
/// `i` in 0..19 while still live: take row token `p` from slot `0x30`; skip
/// the position when the pre-check service accepts `(ctx, p)`; otherwise map
/// the table word through the classifier into a stride (`8` for classes
/// 1, 2, 3, 5, else `0`). When `v == i`, look the row up: a found row whose
/// size is at most 8 copies its 8 payload bytes to `row_out` and stays live,
/// else the flag clears but the scan continues. Otherwise advance the cursor
/// by the stride past the limit dies, else the extend service records
/// position `i` as bit `i` of the mask. Returns 1 when the scan finished all
/// 19 positions live, else 0.
///
/// Edge cases: classifier answers outside 1..=5 (and class 4) mean stride 0;
/// a row token always indexes the table's 8 words; only the low byte of the
/// return is significant. The original also keeps its cursor and `v` in its
/// incoming argument slots (popped on return, so unobservable afterwards);
/// the rewrite holds them in locals.
///
/// Original: 0x005754C0 (thiscall, six stack words).
lf_checker_rt::export!(thiscall, rw_005754C0(this: u32, cursor: u32, row_out: u32, mask_out: u32, flag_out: u32, ctx: u32, extent: u32) -> u32 {
    unsafe {
        const COL_ID: u32 = 0x168;
        const ROW_SLOTS: u32 = 19;
        const VT_SELECTED: u32 = 0x2c;
        const VT_ROW_TOKEN: u32 = 0x30;
        const STRIDE: u32 = 8;
        const SIZE_CAP: u32 = 8;
        const CAL_QUERY: u32 = 2;
        const CAL_PRECHECK: u32 = 4;
        const CAL_CLASSIFY: u32 = 5;
        const CAL_LOOKUP: u32 = 6;
        const CAL_ROW_SIZE: u32 = 7;
        const CAL_EXTEND: u32 = 8;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn vcall0(this: u32, slot: u32) -> u32 {
            unsafe {
                let vt = rd32(this);
                let f: extern "thiscall" fn(u32) -> u32 =
                    core::mem::transmute(rd32(vt.wrapping_add(slot)) as usize);
                f(this)
            }
        }
        #[inline(always)]
        unsafe fn vcall1(this: u32, slot: u32, arg: u32) -> u32 {
            unsafe {
                let vt = rd32(this);
                let f: extern "thiscall" fn(u32, u32) -> u32 =
                    core::mem::transmute(rd32(vt.wrapping_add(slot)) as usize);
                f(this, arg)
            }
        }

        wr32(mask_out, 0);
        wr32(mask_out.wrapping_add(4), 0);
        (flag_out as *mut u8).write(0);
        let limit = extent.wrapping_add(cursor);
        let mut scan = cursor;
        let selected = vcall0(this, VT_SELECTED);
        let mut desc = [0u32; 3];
        let ok = lf_checker_rt::callee_fastcall!(CAL_QUERY, u32, COL_ID, desc.as_mut_ptr() as u32) & 0xFF;
        if ok == 0 {
            return 0;
        }
        let table = desc[2];
        let mut live: u8 = 1;
        let mut i: u32 = 0;
        while i < ROW_SLOTS {
            if live == 0 {
                break;
            }
            let p = vcall1(this, VT_ROW_TOKEN, i);
            let skip = lf_checker_rt::callee_thiscall!(CAL_PRECHECK, u32, ctx, p) & 0xFF;
            if skip == 0 {
                let word = rd32(table.wrapping_add(p.wrapping_mul(4)));
                let class = lf_checker_rt::callee_thiscall!(CAL_CLASSIFY, u32, word);
                let stride = match class {
                    1 | 2 | 3 | 5 => STRIDE,
                    _ => 0,
                };
                if selected == i {
                    live = 0;
                    let row = lf_checker_rt::callee_thiscall!(CAL_LOOKUP, u32, ctx, p);
                    if row != 0 {
                        let size = lf_checker_rt::callee_thiscall!(CAL_ROW_SIZE, u32, row);
                        if size <= SIZE_CAP {
                            let v = (row.wrapping_add(4) as *const u64).read_unaligned();
                            (row_out as *mut u64).write_unaligned(v);
                            live = 1;
                        }
                    }
                    (flag_out as *mut u8).write(live);
                } else {
                    scan = scan.wrapping_add(stride);
                    if scan > limit {
                        live = 0;
                    } else {
                        let done =
                            lf_checker_rt::callee_thiscall!(CAL_EXTEND, u32, ctx, p, scan, stride) & 0xFF;
                        if done == 0 {
                            live = 0;
                        } else {
                            // Bit `i` of the two-word mask (the original's
                            // bts/cmovae sequence; `i` < 19 always).
                            let bit = 1u32 << (i & 31);
                            let (lo, hi) = if i < 0x20 {
                                (bit, 0)
                            } else if i < 0x40 {
                                (0, bit)
                            } else {
                                (0, 0)
                            };
                            wr32(mask_out, lo);
                            wr32(mask_out.wrapping_add(4), hi);
                            live = 1;
                        }
                    }
                }
            }
            i += 1;
        }
        live as u32
    }
});
