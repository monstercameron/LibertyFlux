// original: 0x00538A50 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_CompMafiaWork, player_schema::LeaderboardInfo, 10>::vf14

/// Leaderboard column fill for one ranked board (board id 0x8, 7 columns).
///
/// `this` is the leaderboard-info object (its virtual slots at `+0x2c` and
/// `+0x30` answer the session handle and the per-column value). `start` and
/// `span` bound a cursor range (`end = start + span`, wrapping); `pair_out`
/// takes one 8-byte pair, `mask_out` a 64-bit column mask, `flag_out` a flag
/// byte; `ctx` is an opaque context passed to every helper call.
///
/// Behaviour: zero the mask and flag, fetch the handle (slot `0x2c`), then
/// probe the board with `(id, scratch)`; a zero answer returns 0 at once.
/// Otherwise loop over the 7 columns while the running flag is set: fetch
/// the column value (slot `0x30`), ask the skip helper; when it says go,
/// classify the column through the kind helper (answers 1, 2, 3 or 5 mean a
/// wide 8-unit stride, anything else a zero stride). The column matching the
/// handle copies its pair (8 bytes at object `+4`) when the object exists
/// and its size is at most 8, and records the flag; every other column
/// advances the cursor by the stride, and when still in range asks the place
/// helper and on success writes bit `i` into the mask (plain stores, not
/// or-ed). A failed place, an overrun cursor or a zero flag ends the column
/// (a zero flag ends the loop). Returns the running flag byte.
///
/// Original: 0x00538A50 (thiscall, six stack words; return channel is AL, the
/// upper bytes of EAX are whatever the last call left).
lf_checker_rt::export!(thiscall, rw_00538A50(this: u32, start: u32, pair_out: u32, mask_out: u32, flag_out: u32, ctx: u32, span: u32) -> u32 {
    unsafe {
        const BOARD_ID: u32 = 0x8;
        const ITERATIONS: u32 = 7;
        const VT_HANDLE: u32 = 0x2c;
        const VT_VALUE: u32 = 0x30;
        const STRIDE_WIDE: u32 = 8;
        const PAIR_MAX: u32 = 8;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }

        wr32(mask_out, 0);
        wr32(mask_out + 4, 0);
        (flag_out as *mut u8).write(0);

        let end = start.wrapping_add(span);
        let vtable = rd32(this);
        let handle_of: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(rd32(vtable + VT_HANDLE) as usize);
        let handle = handle_of(this);
        let mut probe = [0u32; 3];
        let probe_ok: u32 =
            lf_checker_rt::callee_fastcall!(2, u32, BOARD_ID, probe.as_mut_ptr() as u32);
        if probe_ok as u8 == 0 {
            return 0;
        }
        let value_of: extern "thiscall" fn(u32, u32) -> u32 =
            core::mem::transmute(rd32(vtable + VT_VALUE) as usize);
        let mut ok: u8 = 1;
        let mut cursor = start;
        let mut i: u32 = 0;
        while i < ITERATIONS {
            if ok == 0 {
                break;
            }
            let v = value_of(this, i);
            let skip: u32 = lf_checker_rt::callee_thiscall!(4, u32, ctx, v);
            if skip as u8 == 0 {
                let cells = probe[2];
                let cell = rd32(cells.wrapping_add(v.wrapping_mul(4)));
                let kind: u32 = lf_checker_rt::callee_thiscall!(5, u32, cell);
                let stride: u32 = match kind {
                    1 | 2 | 3 | 5 => STRIDE_WIDE,
                    _ => 0,
                };
                if handle == i {
                    let obj: u32 = lf_checker_rt::callee_thiscall!(6, u32, ctx, v);
                    ok = 0;
                    if obj != 0 {
                        let size: u32 = lf_checker_rt::callee_thiscall!(7, u32, obj);
                        if size <= PAIR_MAX {
                            let pair = ((obj + 4) as *const u64).read_unaligned();
                            (pair_out as *mut u64).write_unaligned(pair);
                            ok = 1;
                        }
                    }
                    (flag_out as *mut u8).write(ok);
                } else {
                    let advanced = cursor.wrapping_add(stride);
                    if advanced > end {
                        ok = 0;
                    } else {
                        let placed: u32 =
                            lf_checker_rt::callee_thiscall!(8, u32, ctx, v, cursor, stride);
                        if placed as u8 == 0 {
                            ok = 0;
                        } else {
                            let bit: u64 = if i < 64 { 1u64 << i } else { 0 };
                            wr32(mask_out, bit as u32);
                            wr32(mask_out + 4, (bit >> 32) as u32);
                            ok = 1;
                        }
                    }
                    cursor = advanced;
                }
            }
            i += 1;
        }
        ok as u32
    }
});
