// original: 0x00661D10 rage::snEstablishSessionTask::vf3

/// Advance the establish-session task by one tick: a countdown, then a state
/// machine on the state word at `+STATE`.
///
/// `this` is the task object, `delta` a tick count. First the countdown at
/// `+COUNTDOWN`: when it is positive, `delta` is subtracted (wrapping); a
/// result of exactly zero becomes -1; a non-positive countdown is left alone.
///
/// Then the state word selects one of ten cases (any value above 9 exits at
/// once, changing nothing else):
///
/// * 0: requires the inner object (`+INNER`) flag at `+INNER_READY` to be
///   zero. Callee 1 (cdecl, nine words: two zero fill words the original
///   leaves uninitialised on its stack, then 0, 1, `this+A`, 1, `this+E`,
///   `this+S`, `this+P`) runs; a zero low byte moves to state 8, otherwise
///   callee 2 (thiscall, no stack words, `this = inner`) runs and the state
///   becomes 1.
/// * 1: when `+PROBE` is 3 and `+MODE` is 1, `+ACCUM` gains the top bits of
///   `+FLAGS` (`& !0x3f`) and the state becomes 2; otherwise the shared check
///   below runs.
/// * 2: callee 3 (thiscall, eleven words, `this = inner + 0x48`) runs with
///   the listed field words; a zero low byte moves to state 8, else to 3.
/// * 3: `+PROBE` of 3 moves to state 4, else the shared check runs.
/// * 4: callee 4 (thiscall, two words `this+X`, `this+P`) runs; a zero low
///   byte moves to state 8, else to 5.
/// * 5: `+PROBE` of 3 moves to state 6, else the shared check runs.
/// * 6: callee 5 (thiscall, ten words, `this = inner[+INNER_CB]`) runs. Its
///   second word is a pointer into the original's own stack frame (address
///   skipped in the contract, one pointed-to word snapshotted); the original
///   overwrites the low byte of its own ninth pushed word with 1, so that
///   word arrives as `(this + P) with low byte 1`. A zero low byte moves to
///   state 8, else to 7.
/// * 7: `+PROBE` of 3 makes the virtual call `this[0][+0x1c](this, 1, 0),
///   else the shared check runs.
/// * 8: reads `inner + 0x48 + 8` signed; inside 1..=3 callee 6 (thiscall, one
///   word `this+P`) runs and a non-zero low byte moves to state 9; every
///   other way leads to the virtual call `this[0][+0x1c](this, 0, 0)`.
/// * 9: `+PROBE` of 1 exits quietly, anything else makes the (0, 0) call.
///
/// The shared check exits quietly when `+PROBE` is 1 and moves to state 8
/// otherwise. Two loads in case 0 (`inner[+0x32f4]`, `+MODE`) are dead in the
/// original and are not repeated here. The original never sets a return
/// value; the contract compares none.
///
/// Original: thiscall, one stack word.
lf_checker_rt::export!(thiscall, rw_00661D10(this: u32, delta: u32) -> u32 {
    unsafe {
        const COUNTDOWN: u32 = 0x14;
        const INNER: u32 = 0x60;
        const STATE: u32 = 0x90;
        const PROBE: u32 = 0x94;
        const INNER_READY: u32 = 0x1e14;
        const INNER_CB: u32 = 0x24;
        const INNER_SUB: u32 = 0x48;
        const RANGE_OFF: u32 = 0x50;
        const MODE: u32 = 0x758;
        const FLAGS: u32 = 0x760;
        const FLAG_TOP_MASK: u32 = 0xffff_ffc0;
        const ACCUM: u32 = 0x58c;

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
        /// The shared check: quiet exit on probe 1, else state 8.
        #[inline(always)]
        unsafe fn shared(this: u32) {
            unsafe {
                if field(this, PROBE) != 1 {
                    wr32(this.wrapping_add(STATE), 8);
                }
            }
        }
        /// The virtual call through the object's table slot +0x1c.
        #[inline(always)]
        unsafe fn vcall(this: u32, a: u32, b: u32) {
            unsafe {
                let slot: extern "thiscall" fn(u32, u32, u32) -> u32 =
                    core::mem::transmute(rd32(rd32(this).wrapping_add(0x1c)) as usize);
                slot(this, a, b);
            }
        }

        if (field(this, COUNTDOWN) as i32) > 0 {
            let n = field(this, COUNTDOWN).wrapping_sub(delta);
            wr32(this.wrapping_add(COUNTDOWN), if n == 0 { 0xffff_ffff } else { n });
        }
        let inner = field(this, INNER);
        match field(this, STATE) {
            0 => {
                if field(inner, INNER_READY) != 0 {
                    wr32(this.wrapping_add(STATE), 8);
                    return 0;
                }
                let r: u32 = lf_checker_rt::callee_cdecl!(
                    1, u32, 0, 0, 0, 1,
                    this.wrapping_add(0xa0), 1,
                    this.wrapping_add(0xe8),
                    this.wrapping_add(0x758),
                    this.wrapping_add(0x94)
                );
                if r & 0xff == 0 {
                    wr32(this.wrapping_add(STATE), 8);
                } else {
                    lf_checker_rt::callee_thiscall!(2, u32, inner);
                    wr32(this.wrapping_add(STATE), 1);
                }
            }
            1 => {
                if field(this, PROBE) == 3 && field(this, MODE) == 1 {
                    wr32(
                        this.wrapping_add(ACCUM),
                        field(this, ACCUM) | (field(this, FLAGS) & FLAG_TOP_MASK),
                    );
                    wr32(this.wrapping_add(STATE), 2);
                } else {
                    shared(this);
                }
            }
            2 => {
                let combo = (field(this, FLAGS) & FLAG_TOP_MASK) | field(this, ACCUM);
                let r: u32 = lf_checker_rt::callee_thiscall!(
                    3, u32, inner.wrapping_add(INNER_SUB),
                    field(this, 0x9c),
                    field(this, 0x120),
                    field(this, 0x124),
                    this.wrapping_add(0xa0),
                    field(this, 0x75c),
                    combo,
                    field(this, 0x590),
                    field(this, 0x178),
                    field(this, 0x17c),
                    field(this, 0x764),
                    this.wrapping_add(0x94)
                );
                wr32(this.wrapping_add(STATE), if r & 0xff == 0 { 8 } else { 3 });
            }
            3 => {
                if field(this, PROBE) == 3 {
                    wr32(this.wrapping_add(STATE), 4);
                } else {
                    shared(this);
                }
            }
            4 => {
                let r: u32 = lf_checker_rt::callee_thiscall!(
                    4, u32, inner.wrapping_add(INNER_SUB),
                    this.wrapping_add(0x170),
                    this.wrapping_add(0x94)
                );
                wr32(this.wrapping_add(STATE), if r & 0xff == 0 { 8 } else { 5 });
            }
            5 => {
                if field(this, PROBE) == 3 {
                    wr32(this.wrapping_add(STATE), 6);
                } else {
                    shared(this);
                }
            }
            6 => {
                let flag = if field(this, MODE) == 1 { 0 } else { 1 };
                let slot9 = this.wrapping_add(0x94) & 0xffff_ff00 | 1;
                let frame_slot = slot9;
                let r: u32 = lf_checker_rt::callee_thiscall!(
                    5, u32, field(inner, INNER_CB),
                    this.wrapping_add(0xb0),
                    lf_checker_rt::relocated(0x01110340),
                    &frame_slot as *const u32 as u32,
                    flag,
                    4,
                    field(this, 0xa8),
                    field(this, 0xac),
                    1,
                    this.wrapping_add(0x768),
                    slot9
                );
                wr32(this.wrapping_add(STATE), if r & 0xff == 0 { 8 } else { 7 });
            }
            7 => {
                if field(this, PROBE) == 3 {
                    vcall(this, 1, 0);
                } else {
                    shared(this);
                }
            }
            8 => {
                let x = field(inner, RANGE_OFF) as i32;
                if (1..=3).contains(&x) {
                    let r: u32 = lf_checker_rt::callee_thiscall!(
                        7, u32, inner.wrapping_add(INNER_SUB),
                        this.wrapping_add(0x94)
                    );
                    if r & 0xff != 0 {
                        wr32(this.wrapping_add(STATE), 9);
                        return 0;
                    }
                }
                vcall(this, 0, 0);
            }
            9 => {
                if field(this, PROBE) != 1 {
                    vcall(this, 0, 0);
                }
            }
            _ => {}
        }
        0
    }
});
