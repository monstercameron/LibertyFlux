// original: 0x0056F020 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_Race_121, player_schema::LeaderboardInfo, 10>::vf14
// (this file holds the function alone; the crate lib.rs concatenates all eight)
/// Read one leaderboard row-set for a ranked episodic race.
///
/// `thiscall` with six stack words (the callee pops 0x18 bytes). `this` is the leaderboard-info
/// object; `a1` receives an 8-byte row, `a2` a two-word column mask, `a3` a
/// status byte; `a4` is the row-source object; `a0`/`a5` bound a running total.
/// The per-race template id (0x144) is passed to the session callee.
///
/// Behaviour: zero the outputs, ask the object (vtable slot 11) for the
/// target row, open a session through the fastcall callee (out-block of three
/// words, third preset to zero). If the session refuses, return its answer.
/// Otherwise visit rows 0..19: ask the object (slot 12) for the row index;
/// if the poll callee accepts it, continue. If not, classify the row through
/// the table at out-block word 2 and the class callee (classes 1, 2, 3 and 5
/// add a step of 8, anything else adds 0), then either (target row) fetch the
/// row object and copy its 8 payload bytes to `a1` when its size is 8 or
/// less, recording 1 in `a3`, or (other rows) extend the running total and,
/// while it stays within `a0 + a5`, confirm through the commit callee and set
/// bit `row` in the mask at `a2`. Any refusal clears the status and ends the
/// visit at the next row. Returns the status in the low byte; the upper bytes
/// are the last value the original happened to hold in `eax` (a callee answer,
/// one of the `a1`/`a2`/`a3` pointers, or the classifier answer minus one),
/// tracked exactly.
///
/// The jump table behind the classifier reads `[1,2,3,5] -> 8`, else 0.
/// Row indices stay below 19, so the mask's high word is always 0.
lf_checker_rt::export!(thiscall, rw_0056F020(this: u32, a0: u32, a1: u32, a2: u32, a3: u32, a4: u32, a5: u32) -> u32 {
    unsafe {
        const ROWS: u32 = 19;
        const STEP: u32 = 8;
        const MAX_INLINE: i32 = 8;
        const VT_TARGET: u32 = 0x2c;
        const VT_INDEX: u32 = 0x30;
        const ROW_PAYLOAD: u32 = 0x04;
        const SESSION_ID: u32 = 0x144;
        const C_FASTCALL: u32 = 3;
        const C_POLL: u32 = 4;
        const C_CLASS: u32 = 5;
        const C_FETCH: u32 = 6;
        const C_SIZE: u32 = 7;
        const C_COMMIT: u32 = 8;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }

        wr32(a2, 0);
        wr32(a2.wrapping_add(4), 0);
        (a3 as *mut u8).write(0);
        let limit = a5.wrapping_add(a0);
        let mut total = a0;
        type Vf11 = extern "thiscall" fn(u32) -> u32;
        type Vf12 = extern "thiscall" fn(u32, u32) -> u32;
        let target: u32 = {
            let f: Vf11 = core::mem::transmute(rd32(rd32(this).wrapping_add(VT_TARGET)) as usize);
            f(this)
        };
        let mut eax_now = target;
        let mut out = [0u32; 3];
        let session: u32 = lf_checker_rt::callee_fastcall!(C_FASTCALL, u32, SESSION_ID, out.as_mut_ptr() as u32);
        eax_now = session;
        if session & 0xff == 0 {
            return session;
        }
        let mut status: u8 = 1;
        let mut row: u32 = 0;
        while row < ROWS {
            if status == 0 {
                break;
            }
            let f12: Vf12 = core::mem::transmute(rd32(rd32(this).wrapping_add(VT_INDEX)) as usize);
            let index = f12(this, row);
            eax_now = index;
            let poll: u32 = lf_checker_rt::callee_thiscall!(C_POLL, u32, a4, index);
            eax_now = poll;
            if poll & 0xff == 0 {
                let table = out[2];
                let key = rd32(table.wrapping_add(index.wrapping_mul(4)));
                let class: u32 = lf_checker_rt::callee_thiscall!(C_CLASS, u32, key);
                // The classifier decrements before the range check (except on
                // the -1 early-out), so eax holds class-1 past this point.
                eax_now = if class == 0xffffffff { 0xffffffff } else { class.wrapping_sub(1) };
                let step = match class {
                    1 | 2 | 3 | 5 => STEP,
                    _ => 0,
                };
                if target == row {
                    let fetched: u32 = lf_checker_rt::callee_thiscall!(C_FETCH, u32, a4, index);
                    eax_now = fetched;
                    status = 0;
                    if fetched != 0 {
                        let size: u32 = lf_checker_rt::callee_thiscall!(C_SIZE, u32, fetched);
                        eax_now = size;
                        if (size as i32) <= MAX_INLINE {
                            wr32(a1, rd32(fetched.wrapping_add(ROW_PAYLOAD)));
                            wr32(a1.wrapping_add(4), rd32(fetched.wrapping_add(ROW_PAYLOAD + 4)));
                            status = 1;
                            eax_now = a1;
                        }
                    }
                    (a3 as *mut u8).write(status);
                    eax_now = a3;
                } else {
                    let grown = total.wrapping_add(step);
                    if grown > limit {
                        status = 0;
                    } else {
                        let ok: u32 = lf_checker_rt::callee_thiscall!(C_COMMIT, u32, a4, index, total, step);
                        eax_now = ok;
                        if ok & 0xff == 0 {
                            status = 0;
                        } else {
                            wr32(a2, 1u32 << row);
                            wr32(a2.wrapping_add(4), 0);
                            status = 1;
                            eax_now = a2;
                        }
                    }
                    total = grown;
                }
            }
            row += 1;
        }
        (eax_now & 0xffffff00) | status as u32
    }
});
