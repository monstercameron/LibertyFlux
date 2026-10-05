// original: 0x00570A60 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_R
/// Ranked-leaderboard row scan for one episodic-race board (family id 0x14a).
///
/// `this` is the board-info object: dword at `+0` points at its vtable, whose
/// slot at `+0x2c` answers the selected row for the trial and whose slot at
/// `+0x30` answers the entry index for iteration `i`. `cursor` is the running
/// write offset advanced by 0 or 8 per row; `bound` is `cursor + ctx_obj`.
/// `out_row` receives an 8-byte row copy on the selected iteration (never
/// zeroed: the copy overwrites all 8 bytes). `out_mask` is zeroed as 8 bytes
/// up front and then receives a one-hot mask of the last emitted row.
/// `out_flag` is zeroed as one byte up front and then a success byte.
/// `ctx_obj` is passed as the object pointer to every helper call.
/// `bound_in` is read once as the cursor-bound summand; the original then
/// reuses its stack slot as a spill for the selected row.
///
/// Up to 19 rows are scanned while the ok flag holds: the helper answers
/// per-row skip, a kind code mapped through a five-entry table (kinds
/// 1, 2, 3 and 5 advance by 8, anything else by 0), and either a single-row
/// fetch (copied only when its size is 8 or less) on the selected iteration
/// or an emit that records the row bit. The size gate is a signed
/// comparison (`(an instruction of the original); jg`), so a negative size still copies. The low
/// byte of the return is the ok flag (the original sets only `al`, leaving
/// the upper bytes from the last call, so the contract compares `al`).
///
/// Original: 0x00570A60 (thiscall, six stack words, callee pops 0x18).
lf_checker_rt::export!(thiscall, rw_00570a60(this: u32, cursor: u32, out_row: u32, out_mask: u32, out_flag: u32, ctx_obj: u32, bound_in: u32) -> u32 {
    unsafe {
        const FAMILY_ID: u32 = 0x14a;
        const ITERATIONS: u32 = 19;
        const VT_SELECT: u32 = 0x2c;
        const VT_INDEX: u32 = 0x30;
        const FULL_WIDTH: u32 = 8;
        const C_LOOKUP: u32 = 3;
        const C_PROBE: u32 = 4;
        const C_KIND: u32 = 5;
        const C_FETCH: u32 = 6;
        const C_SIZE: u32 = 7;
        const C_EMIT: u32 = 8;

        #[inline(always)]
        unsafe fn vcall(this: u32, slot: u32, arg: Option<u32>) -> u32 {
            unsafe {
                let vtable = (this as *const u32).read_unaligned();
                let target = ((vtable + slot) as *const u32).read_unaligned();
                if let Some(a) = arg {
                    let f: extern "thiscall" fn(u32, u32) -> u32 =
                        core::mem::transmute(target as usize);
                    f(this, a)
                } else {
                    let f: extern "thiscall" fn(u32) -> u32 =
                        core::mem::transmute(target as usize);
                    f(this)
                }
            }
        }

        (out_mask as *mut u64).write_unaligned(0);
        (out_flag as *mut u8).write(0);
        let bound = cursor.wrapping_add(bound_in);

        let selected = vcall(this, VT_SELECT, None);
        let mut probe_out = [0u32; 3];
        let lookup = lf_checker_rt::callee_fastcall!(
            C_LOOKUP, u32, FAMILY_ID, probe_out.as_mut_ptr() as u32);
        if (lookup as u8) == 0 {
            return 0;
        }
        let table = probe_out[2];

        let mut ok: u8 = 1;
        let mut cursor_saved = cursor;
        let mut cursor_now = cursor;
        let mut width = 0u32;
        let mut i = 0u32;
        while i < ITERATIONS {
            if ok == 0 {
                break;
            }
            let idx = vcall(this, VT_INDEX, Some(i));
            let skip = lf_checker_rt::callee_thiscall!(C_PROBE, u32, ctx_obj, idx);
            if (skip as u8) != 0 {
                i += 1;
                continue;
            }
            let kind_ptr = ((table + idx.wrapping_mul(4)) as *const u32).read_unaligned();
            let kind = lf_checker_rt::callee_thiscall!(C_KIND, u32, kind_ptr);
            width = match kind { 1 | 2 | 3 | 5 => FULL_WIDTH, _ => 0 };
            if selected != i {
                cursor_now = cursor_now.wrapping_add(width);
                if cursor_now > bound {
                    ok = 0;
                } else {
                    let done = lf_checker_rt::callee_thiscall!(
                        C_EMIT, u32, ctx_obj, idx, cursor_saved, width);
                    if (done as u8) == 0 {
                        ok = 0;
                    } else {
                        (out_mask as *mut u32).write_unaligned(1u32 << i);
                        ((out_mask + 4) as *mut u32).write_unaligned(0);
                        ok = 1;
                    }
                }
                cursor_saved = cursor_now;
            } else {
                let obj = lf_checker_rt::callee_thiscall!(C_FETCH, u32, ctx_obj, idx);
                ok = 0;
                if obj != 0 {
                    let size = lf_checker_rt::callee_thiscall!(C_SIZE, u32, obj);
                    // Signed: the original uses `jg` after `(an instruction of the original)`, so a
                    // negative size (e.g. -1) still copies.
                    if (size as i32) <= FULL_WIDTH as i32 {
                        (out_row as *mut u64).write_unaligned(
                            ((obj + 4) as *const u64).read_unaligned());
                        ok = 1;
                    }
                }
                (out_flag as *mut u8).write(ok);
            }
            i += 1;
        }
        ok as u32
    }
});
