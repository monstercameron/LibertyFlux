// original: 0x005AFDE0 MS_PAUSED

/// Paused-state ticker: advance a small state machine and its timer, then run
/// one update pass over the pause-menu subsystems.
///
/// Calling convention: cdecl, no stack arguments, no observed return value
/// (the single caller ignores `eax`, so the contract compares no return
/// channel; on the early-exit path the original leaves `cookie ^ esp` in
/// `eax`, which is likewise unobserved).
///
/// State lives in two globals: `STATE` (0..8 are the menu states, anything
/// else takes the default arm) and `TIMER` (accumulates tick deltas until it
/// reaches a threshold). Entry gates: `GATE` must be 1, three pause-flag
/// bytes must be 0, and `MODE` must not be `0x39`; otherwise the function
/// does nothing.
///
/// Behaviour in order:
/// 1. Pick the timer threshold: `LONG_TICKS` (0x1388) when the state is 1, 4
///    or 7, else `SHORT_TICKS` (0xc8). If the timer is below it (an UNSIGNED
///    comparison, `jae`), add one tick delta from callee 1, store the timer
///    and skip the state advance.
/// 2. Otherwise reset the timer and advance the state: 8 wraps to 3, every
///    other value increments (wrapping). States 0, 2, 3, 5, 6 and 8
///    (after the advance) re-arm the menu object through callee 2.
/// 3. Latch the flag byte: `0xff`, or callee 3's low byte when the menu
///    object's flag byte is non-zero (the state is re-read after that call).
/// 4. Run one switch arm on the state (0/6, 1/7, 2/8, 3, 4, 5, default),
///    each fetching one item through callee 4 or 6 and submitting it through
///    callee 5. The default arm additionally clears the TLS scratch byte and
///    runs callee 7. Arms for states 2, 5 and 8 submit `0xff - flag`.
/// 5. Run the fixed tail: callee 8's answer selects callee 9's argument,
///    then callees 10-16 run with constant or stack-slot arguments (the two
///    float words passed to callee 16 are bit-copies of uninitialised stack,
///    modelled here as zero words under the contract's `stack_fill`).
/// 6. When the re-read state is 0, 1, 2, 6, 7 or 8, run the long path:
///    callee 19's answer selects the five-argument (callee 22) or
///    four-argument (callee 23) log call, with the row id picked from `SUB`
///    and the two column words each defaulting unless overridden; then
///    callees 7, 24, 17, 28 and 25. Any other state runs the short path
///    (callees 17 and 18) instead.
/// 7. Finish through callee 26 and the cookie check (callee 27). Callee
///    26's third argument is path-dependent: the long path passes its stack
///    slot, while the short path jumps over that slot's address computation
///    and reuses the leftover `eax` (the TLS block pointer).
///
/// The TLS object (slot 0) is only ever addressed, never read: callees take
/// `+0x78` and `+0x4c8` interior pointers. All stack-slot buffers passed to
/// callees hold uninitialised stack except the latched flag byte, so they
/// are zero words here (the contract pins `stack_fill` to 0 and snapshots
/// them). No floating-point arithmetic happens; `movss` traffic is plain
/// word copies.
use lf_checker_rt::{callee_cdecl, callee_thiscall, relocated, tls_slot};

pub const GATE: u32 = 0x01030B7C;
pub const PAUSE0: u32 = 0x01160C35;
pub const PAUSE1: u32 = 0x01160C36;
pub const PAUSE2: u32 = 0x01160C3D;
pub const MODE: u32 = 0x01160C40;
pub const MODE_EXIT: u32 = 0x39;
pub const STATE: u32 = 0x018E51D8;
pub const TIMER: u32 = 0x018E51DC;
pub const MENU_OBJ: u32 = 0x01161608;
pub const SUB: u32 = 0x01160CC8;
pub const SINK_OBJ: u32 = 0x0116BFF0;
pub const COL_B_DFLT: u32 = 0x01295848;
pub const COL_A_DFLT: u32 = 0x0129584C;
pub const COL_B_OVER: u32 = 0x01295854;
pub const COL_A_OVER: u32 = 0x01295858;
pub const SCORE: u32 = 0x01295834;
pub const NO_OVERRIDE: u32 = 0xFFFF_FFFF;

pub const SHORT_TICKS: u32 = 0xC8;
pub const LONG_TICKS: u32 = 0x1388;
pub const TLS_SCRATCH: u32 = 0x4C8;
pub const TLS_BLOCK: u32 = 0x78;
pub const FETCH_LEN: u32 = 0x40;
pub const SINK_FMT: u32 = 0x00F8932C;
pub const LOG_FMT_FULL_A: u32 = 0x00F8936C;
pub const LOG_FMT_FULL_B: u32 = 0x00F89358;
pub const LOG_FMT_SHORT_A: u32 = 0x00F89348;
pub const LOG_FMT_SHORT_B: u32 = 0x00F89338;
pub const ONE_FLOAT_BITS: u32 = 0x3F80_0000;

#[inline(always)]
unsafe fn rd32(a: u32) -> u32 {
    unsafe { (a as *const u32).read_unaligned() }
}
#[inline(always)]
unsafe fn rd8(a: u32) -> u8 {
    unsafe { (a as *const u8).read() }
}

unsafe fn column(default_va: u32, over_va: u32) -> u32 {
    unsafe {
        let over = rd32(relocated(over_va));
        if over != NO_OVERRIDE {
            over
        } else {
            rd32(relocated(default_va))
        }
    }
}

unsafe fn body() {
    unsafe {
        // Every exit path runs the cookie check (callee 27): the
        // original shares one epilogue, so even a gate failure logs it.
        if rd32(relocated(GATE)) != 1 {
            callee_cdecl!(27, u32,);
            return;
        }
        if rd8(relocated(PAUSE0)) != 0 {
            callee_cdecl!(27, u32,);
            return;
        }
        if rd8(relocated(PAUSE1)) != 0 {
            callee_cdecl!(27, u32,);
            return;
        }
        if rd8(relocated(PAUSE2)) != 0 {
            callee_cdecl!(27, u32,);
            return;
        }
        if rd32(relocated(MODE)) == MODE_EXIT {
            callee_cdecl!(27, u32,);
            return;
        }
        let menu = relocated(MENU_OBJ);
        let sink = relocated(SINK_OBJ);
        let mut state = rd32(relocated(STATE));
        let threshold = match state {
            1 | 4 | 7 => LONG_TICKS,
            _ => SHORT_TICKS,
        };
        let timer = rd32(relocated(TIMER));
        // Unsigned: the original branches with `jae`.
        if timer < threshold {
            let next = timer.wrapping_add(callee_cdecl!(1, u32,));
            (relocated(TIMER) as *mut u32).write_unaligned(next);
        } else {
            (relocated(TIMER) as *mut u32).write_unaligned(0);
            if state == 8 {
                (relocated(STATE) as *mut u32).write_unaligned(3);
            } else {
                state = state.wrapping_add(1);
                (relocated(STATE) as *mut u32).write_unaligned(state);
                // The chain ends `(an instruction of the original)`: an equal state 5
                // falls through into the re-arm block like the rest.
                match state {
                    0 | 2 | 3 | 5 | 6 | 8 => {}
                    _ => {
                        run_flag_and_switch(menu, state);
                        run_tail(sink);
                        return;
                    }
                }
            }
            callee_thiscall!(2, u32, menu, 0xC8, 0, 0xFF);
            state = rd32(relocated(STATE));
            run_flag_and_switch(menu, state);
            run_tail(sink);
            return;
        }
        run_flag_and_switch(menu, state);
        run_tail(sink);
    }
}

unsafe fn fetch(slot: &mut [u32; 2], callee: u32, flag: u32) {
    unsafe {
        let ptr = slot.as_mut_ptr() as u32;
        let item = match callee {
            4 => callee_cdecl!(4, u32, ptr, FETCH_LEN, flag),
            _ => callee_cdecl!(6, u32, ptr, FETCH_LEN),
        };
        let word = ((item) as *const u32).read_unaligned();
        callee_cdecl!(5, u32, word);
    }
}

unsafe fn run_flag_and_switch(menu: u32, mut state: u32) {
    unsafe {
        let mut flag: u8 = 0xFF;
        if rd8(menu) != 0 {
            let answer = callee_thiscall!(3, u32, menu);
            state = rd32(relocated(STATE));
            flag = (answer & 0xFF) as u8;
        }
        let mut flag_slot = [0u32; 2];
        flag_slot[0] = flag as u32;
        let flag_word = flag_slot[0];
        let mut other = [0u32; 2];
        let inverse = 0xFFu8.wrapping_sub(flag) as u32;
        match state {
            0 | 6 => fetch(&mut flag_slot, 4, flag_word),
            1 | 7 => fetch(&mut other, 6, 0),
            2 | 8 => {
                other = [0, 0];
                let ptr = other.as_mut_ptr() as u32;
                let item = callee_cdecl!(4, u32, ptr, FETCH_LEN, inverse);
                callee_cdecl!(5, u32, (item as *const u32).read_unaligned());
            }
            3 => fetch(&mut other, 4, flag_word),
            4 => fetch(&mut other, 6, 0),
            5 => {
                other = [0, 0];
                let ptr = other.as_mut_ptr() as u32;
                let item = callee_cdecl!(4, u32, ptr, FETCH_LEN, inverse);
                callee_cdecl!(5, u32, (item as *const u32).read_unaligned());
            }
            _ => {
                other = [0, 0];
                let ptr = other.as_mut_ptr() as u32;
                let item = callee_cdecl!(6, u32, ptr, FETCH_LEN);
                callee_cdecl!(5, u32, (item as *const u32).read_unaligned());
                let obj = tls_slot(0);
                ((obj + TLS_SCRATCH) as *mut u8).write(0);
                callee_cdecl!(7, u32, obj + TLS_SCRATCH, obj + TLS_BLOCK);
            }
        }
    }
}

unsafe fn run_tail(sink: u32) {
    unsafe {
        let picked = callee_thiscall!(8, u32, sink);
        callee_cdecl!(9, u32, if (picked & 0xFF) != 0 { 2 } else { 0 });
        callee_cdecl!(10, u32, 0);
        callee_cdecl!(11, u32, 1);
        callee_cdecl!(12, u32, 0, ONE_FLOAT_BITS);
        callee_cdecl!(13, u32, 1);
        let mut slot_a = [0u32; 2];
        let mut slot_b = [0u32; 2];
        callee_cdecl!(14, u32, slot_a.as_mut_ptr() as u32, 0x17);
        callee_cdecl!(14, u32, slot_b.as_mut_ptr() as u32, 0x18);
        let mut slot_c = [0u32; 2];
        callee_cdecl!(
            15,
            u32,
            2,
            slot_a.as_mut_ptr() as u32,
            slot_c.as_mut_ptr() as u32,
            0
        );
        callee_cdecl!(16, u32, 0, 0);
        let mut shared = [0u32; 2];
        let state = rd32(relocated(STATE));
        // Callee 26's third argument differs by path: the long path
        // passes its stack slot, while the short path jumps over the
        // slot's address computation and passes the leftover `eax`,
        // which still holds the TLS block pointer set up for callee 18.
        let slot: u32 = match state {
            0 | 1 | 2 | 6 | 7 | 8 => {
                run_long(sink, &mut shared);
                shared.as_mut_ptr() as u32
            }
            _ => {
                let row = callee_thiscall!(17, u32, sink, relocated(SINK_FMT));
                let obj = tls_slot(0);
                callee_cdecl!(18, u32, obj + TLS_BLOCK, row, NO_OVERRIDE);
                obj + TLS_BLOCK
            }
        };
        callee_cdecl!(26, u32, 0, 0, slot, NO_OVERRIDE, NO_OVERRIDE);
        callee_cdecl!(27, u32,);
    }
}

unsafe fn run_long(sink: u32, shared: &mut [u32; 2]) {
    unsafe {
        if (callee_cdecl!(19, u32,) & 0xFF) == 0 {
            let mut row_id = 0u32;
            let first = callee_cdecl!(20, u32,);
            if first != 0 {
                let second = callee_cdecl!(20, u32,);
                let this = ((second + 0x228) as *const u32).read_unaligned();
                row_id = callee_thiscall!(21, u32, this);
            }
            let sub = rd32(relocated(SUB));
            let fmt = match sub {
                1 | 4 => relocated(LOG_FMT_FULL_B),
                _ => relocated(LOG_FMT_FULL_A),
            };
            let a = column(COL_A_DFLT, COL_A_OVER);
            let b = column(COL_B_DFLT, COL_B_OVER);
            let obj = tls_slot(0);
            callee_cdecl!(22, u32, obj + TLS_SCRATCH, fmt, b, a, row_id);
        } else {
            let sub = rd32(relocated(SUB));
            let fmt = match sub {
                1 | 4 => relocated(LOG_FMT_SHORT_B),
                _ => relocated(LOG_FMT_SHORT_A),
            };
            let a = column(COL_A_DFLT, COL_A_OVER);
            let b = column(COL_B_DFLT, COL_B_OVER);
            let obj = tls_slot(0);
            callee_cdecl!(23, u32, obj + TLS_SCRATCH, fmt, b, a);
        }
        let obj = tls_slot(0);
        callee_cdecl!(7, u32, obj + TLS_SCRATCH, obj + TLS_BLOCK);
        let score = callee_cdecl!(24, u32, rd32(relocated(SCORE)));
        let row = callee_thiscall!(17, u32, sink, score);
        callee_cdecl!(28, u32, shared.as_mut_ptr() as u32, row, NO_OVERRIDE);
        callee_cdecl!(25, u32, shared.as_mut_ptr() as u32, obj + TLS_BLOCK);
    }
}

lf_checker_rt::export!(cdecl, rw_005AFDE0() -> u32 {
    unsafe { body() };
    0
});
