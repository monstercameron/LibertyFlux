// original: 0x00662BF0 rage::snJoinSessionTask::vf3

/// Advance the join-session task: a countdown, then an eight-case state
/// machine that repeats while its progress check keeps answering.
///
/// `this` is the task object, `delta` a tick count. A positive `+COUNTDOWN`
/// drops by `delta` (wrapping), an exact zero becomes -1, a non-positive
/// value is left alone.
///
/// Each pass switches on `+STATE` (above 7 falls straight to the tail):
///
/// * 0: callee 1 (thiscall, no stack words, `this = inner + 0xbb0`) then
///   callee 2 (thiscall, no stack words) run. Callee 3 (thiscall, one word:
///   `this+0x118`) takes a frame pointer with the four `+108` dwords stored
///   just below it; only the pointed-to zero word is compared. Callee 4
///   (cdecl, five words: `+94`, `+150`, `+154`, `+158`, `this+GATE`) runs; a
///   zero low byte makes the virtual call `this[0][+0x1c](this, 0, 0),
///   otherwise the virtual call `obj[0][+8](obj)` through `+OBJ` runs and
///   the state becomes 1.
/// * 1: `+GATE` of 1 makes the virtual call `obj[0]+0xc(obj, delta)`; 3 moves
///   to state 2; anything else writes 1 to `+364` and makes the (0, 0) call.
/// * 2: callee 5 (thiscall, no stack words) takes a zero-word frame pointer.
///   A non-zero guard global enters a critical section on its address, the
///   global is re-read, a flag byte being zero selects the first base
///   constant and non-zero the second, and a still non-zero guard leaves the
///   section again. Callee 6 (thiscall, nine words, `this = inner`) runs; its
///   frame-pointer word aims at the entry register spill, which Rust cannot
///   observe, so it goes uncompared. Callee 7 (thiscall, two words) runs,
///   then callee 8 (thiscall, three words: two zero-word pointers and the
///   size 0x498, plus a zero-word pointer in `ecx`) runs; the original keeps
///   `ecx` live across it, so it is declared register-preserving. A zero low
///   byte moves to state 6. Otherwise callee 9 (thiscall, seven words, `this
///   = inner`) runs with the preserved pointer, a fill zero word, the second
///   buffer pointer, -1, the relocated constant, `this+F38` and `this+880`;
///   a zero full answer moves to state 6, otherwise callee 10 (thiscall, two
///   words from `this+880`) answers a pointer whose target lands in `+360`,
///   the imported exchange runs on the gate, the gate's next word is
///   cleared, and the state becomes 3.
/// * 3: `+GATE` of 3 moves to state 4, of 1 exits the pass quietly, anything
///   else moves to state 6.
/// * 4: callee 11 (thiscall, four words) runs; a zero low byte moves to
///   state 6, else to 5.
/// * 5: `+GATE` of 3 makes the virtual call with (1, 0), else the shared
///   check of case 3 runs.
/// * 6: callee 12 (thiscall, one word `this+GATE`) runs; a zero low byte
///   makes the (0, 0) call, else the state becomes 7.
/// * 7: `+GATE` of 1 ends the function at once, anything else makes the
///   (0, 0) call.
///
/// The tail ends the pass when `+GATE` is 1; otherwise callee 13 (thiscall,
/// no stack words) runs and a non-zero low byte repeats the loop. The
/// contract answers callee 13 with 1 then 0, so every pass runs at most
/// twice. The security-cookie check ends the function.
///
/// Original: thiscall, one stack word. No return value is set.
lf_checker_rt::export!(thiscall, rw_00662BF0(this: u32, delta: u32) -> u32 {
    unsafe {
        const COUNTDOWN: u32 = 0x14;
        const INNER: u32 = 0x60;
        const STATE: u32 = 0x90;
        const INNER_SUB: u32 = 0x48;
        const GUARD: u32 = 0x019f923c;
        const FLAG_BYTE: u32 = 0x018b872f;
        const BASE_A: u32 = 0x011103a0;
        const BASE_B: u32 = 0x01110340;
        const COOKIE: u32 = 0x01057fb4;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn field(this: u32, off: u32) -> u32 {
            unsafe { rd32(this.wrapping_add(off)) }
        }
        #[inline(always)]
        unsafe fn vcall_esi(this: u32, a: u32, b: u32) {
            unsafe {
                let slot: extern "thiscall" fn(u32, u32, u32) -> u32 =
                    core::mem::transmute(rd32(rd32(this).wrapping_add(0x1c)) as usize);
                slot(this, a, b);
            }
        }
        #[inline(always)]
        unsafe fn cookie() {
            unsafe {
                let ck = lf_checker_rt::global::<u32>(COOKIE).read_unaligned();
                lf_checker_rt::callee_thiscall!(14, u32, ck);
            }
        }

        if (field(this, COUNTDOWN) as i32) > 0 {
            let n = field(this, COUNTDOWN).wrapping_sub(delta);
            wr32(this.wrapping_add(COUNTDOWN), if n == 0 { 0xffff_ffff } else { n });
        }
        let gate = this.wrapping_add(0x78c);
        loop {
            let mut done = false;
            match field(this, STATE) {
                0 => {
                    let inner = field(this, INNER);
                    lf_checker_rt::callee_thiscall!(1, u32, inner.wrapping_add(0xbb0));
                    lf_checker_rt::callee_thiscall!(2, u32, inner);
                    let below = [
                        field(this, 0x108),
                        field(this, 0x10c),
                        field(this, 0x110),
                        field(this, 0x114),
                        0,
                    ];
                    lf_checker_rt::callee_thiscall!(
                        3, u32, &below[4] as *const u32 as u32,
                        this.wrapping_add(0x118)
                    );
                    let r: u32 = lf_checker_rt::callee_cdecl!(
                        4, u32, field(this, 0x94), field(this, 0x150),
                        field(this, 0x154), field(this, 0x158), gate
                    );
                    if r & 0xff == 0 {
                        vcall_esi(this, 0, 0);
                    } else {
                        let obj = this.wrapping_add(0x798);
                        let f: extern "thiscall" fn(u32) -> u32 = core::mem::transmute(
                            rd32(rd32(obj).wrapping_add(8)) as usize,
                        );
                        f(obj);
                        wr32(this.wrapping_add(STATE), 1);
                    }
                }
                1 => {
                    let g = rd32(gate);
                    if g == 1 {
                        let obj = this.wrapping_add(0x798);
                        let f: extern "thiscall" fn(u32, u32) -> u32 = core::mem::transmute(
                            rd32(rd32(obj).wrapping_add(0x0c)) as usize,
                        );
                        f(obj, delta);
                    } else if g == 3 {
                        wr32(this.wrapping_add(STATE), 2);
                    } else {
                        wr32(this.wrapping_add(0x364), 1);
                        vcall_esi(this, 0, 0);
                    }
                }
                2 => {
                    let z = 0u32;
                    lf_checker_rt::callee_thiscall!(5, u32, &z as *const u32 as u32);
                    if lf_checker_rt::global::<u32>(GUARD).read_unaligned() != 0 {
                        let f: extern "stdcall" fn(u32) -> u32 = core::mem::transmute(
                            lf_checker_rt::global::<u32>(0x00e731cc).read_unaligned() as usize,
                        );
                        f(lf_checker_rt::relocated(GUARD));
                    }
                    let g2 = lf_checker_rt::global::<u32>(GUARD).read_unaligned();
                    let b = if lf_checker_rt::global::<u8>(FLAG_BYTE).read_unaligned() == 0 {
                        lf_checker_rt::relocated(BASE_B)
                    } else {
                        lf_checker_rt::relocated(BASE_A)
                    };
                    if g2 != 0 {
                        let f: extern "stdcall" fn(u32) -> u32 = core::mem::transmute(
                            lf_checker_rt::global::<u32>(0x00e731c8).read_unaligned() as usize,
                        );
                        f(lf_checker_rt::relocated(GUARD));
                    }
                    let inner = field(this, INNER);
                    let zc = 0u32;
                    lf_checker_rt::callee_thiscall!(
                        6, u32, &zc as *const u32 as u32,
                        field(inner, 0x540), field(inner, 0x544),
                        this.wrapping_add(0x98), b, field(this, 0x158),
                        this.wrapping_add(0x15c), field(this, 0x35c),
                        this.wrapping_add(0x588), field(this, 0x788)
                    );
                    lf_checker_rt::callee_thiscall!(
                        7, u32, field(inner, 0x24),
                        this.wrapping_add(0x56c), field(inner, 0x32f4)
                    );
                    let p1 = 0u32;
                    let p2 = 0u32;
                    let pc = 0u32;
                    let r8: u32 = lf_checker_rt::callee_thiscall!(
                        8, u32, &pc as *const u32 as u32,
                        &p2 as *const u32 as u32, 0x498, &p1 as *const u32 as u32
                    );
                    if r8 & 0xff == 0 {
                        wr32(this.wrapping_add(STATE), 6);
                    } else {
                        let r9: u32 = lf_checker_rt::callee_thiscall!(
                            9, u32, inner,
                            this.wrapping_add(0x880), this.wrapping_add(0xf38),
                            lf_checker_rt::relocated(BASE_B), 0xffff_ffff,
                            &p2 as *const u32 as u32, 0, &pc as *const u32 as u32
                        );
                        if r9 == 0 {
                            wr32(this.wrapping_add(STATE), 6);
                        } else {
                            let b880 = this.wrapping_add(0x880);
                            let r10: u32 = lf_checker_rt::callee_thiscall!(
                                10, u32, inner, field(b880, 0x38), field(b880, 0x3c)
                            );
                            wr32(this.wrapping_add(0x360), rd32(r10));
                            let fx: extern "stdcall" fn(u32, u32) -> u32 =
                                core::mem::transmute(
                                    lf_checker_rt::global::<u32>(0x00e731f0).read_unaligned()
                                        as usize,
                                );
                            fx(gate, 1);
                            wr32(gate.wrapping_add(4), 0);
                            wr32(this.wrapping_add(STATE), 3);
                        }
                    }
                }
                3 => {
                    let g = rd32(gate);
                    if g == 3 {
                        wr32(this.wrapping_add(STATE), 4);
                    } else if g != 1 {
                        wr32(this.wrapping_add(STATE), 6);
                    }
                }
                4 => {
                    let r: u32 = lf_checker_rt::callee_thiscall!(
                        11, u32, field(this, INNER).wrapping_add(INNER_SUB),
                        this.wrapping_add(0xe0), this.wrapping_add(0x158), 1, gate
                    );
                    wr32(this.wrapping_add(STATE), if r & 0xff == 0 { 6 } else { 5 });
                }
                5 => {
                    if rd32(gate) == 3 {
                        vcall_esi(this, 1, 0);
                    } else if rd32(gate) != 1 {
                        wr32(this.wrapping_add(STATE), 6);
                    }
                }
                6 => {
                    let r: u32 = lf_checker_rt::callee_thiscall!(
                        12, u32, field(this, INNER).wrapping_add(INNER_SUB), gate
                    );
                    if r & 0xff == 0 {
                        vcall_esi(this, 0, 0);
                    } else {
                        wr32(this.wrapping_add(STATE), 7);
                    }
                }
                7 => {
                    if rd32(gate) == 1 {
                        done = true;
                    } else {
                        vcall_esi(this, 0, 0);
                    }
                }
                _ => {}
            }
            if done {
                break;
            }
            if rd32(gate) == 1 {
                break;
            }
            let r: u32 = lf_checker_rt::callee_thiscall!(13, u32, this);
            if r & 0xff == 0 {
                break;
            }
        }
        cookie();
        0
    }
});
