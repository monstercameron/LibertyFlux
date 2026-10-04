// original: 0x00662440 rage::snHostSessionTask::vf3

/// Advance the host-session task: a countdown, then a six-case state machine
/// that repeats while its progress check keeps answering.
///
/// `this` is the task object, `delta` a tick count. The countdown at
/// `+COUNTDOWN` behaves as in the sibling establish-task tick: a positive
/// value drops by `delta` (wrapping), an exact zero becomes -1, a
/// non-positive value is left alone.
///
/// Then the loop runs. Each pass switches on the state word at `+STATE`
/// (above 5 falls straight through to the tail):
///
/// * 0: callee 1 (thiscall, no stack words, `this = inner + 0xbb0`) then
///   callee 2 (thiscall, no stack words, `this = inner`) run. When `+A4` is
///   zero, callee 3 (thiscall, four words: `+9c`, `+a8`, `this+B0`,
///   `this+P`) runs; otherwise callee 4 (thiscall, seven words: `+9c`, `+a4`,
///   `+a8`, `+ac`, `this+B0`, `+4bc`, `this+P`) runs. A zero low byte makes
///   the virtual call `this[0][+0x1c](this, 0, 0)`; otherwise state becomes 1.
/// * 1: `+PROBE` of 3 runs callee 5 (thiscall, no stack words) and moves to
///   state 2; 1 exits the pass quietly; anything else makes the (0, 0) call.
/// * 2: a non-zero `+9c` moves to state 4. Otherwise a global flag byte is
///   tested: bit 1 clear moves to state 4, bit 1 set loads sixteen global
///   bytes into two adjacent frame slots and calls callee 6 (thiscall, four
///   words: a frame pointer, `this+A`, 1, `this+P`). The pointer aims at the
///   lower half; all four words of both halves are snapshotted. A zero low
///   byte moves to state 4, else to 3.
/// * 3: `+PROBE` of 3 makes the virtual call with (1, 0); 1 exits quietly;
///   anything else moves to state 4.
/// * 4: callee 7 (thiscall, one word `this+P`) runs; a zero low byte makes
///   the (0, 0) call, otherwise state becomes 5.
/// * 5: `+PROBE` of 1 exits quietly, anything else makes the (0, 0) call.
///
/// The tail ends the pass (and the function) when `+PROBE` is 1; otherwise
/// callee 8 (thiscall, no stack words) runs and a non-zero low byte repeats
/// the loop from the switch. The contract answers callee 8 with 1 then 0, so
/// every pass runs at most twice and the repeat edge is exercised.
///
/// Original: thiscall, one stack word. No return value is set.
lf_checker_rt::export!(thiscall, rw_00662440(this: u32, delta: u32) -> u32 {
    unsafe {
        const COUNTDOWN: u32 = 0x14;
        const INNER: u32 = 0x60;
        const STATE: u32 = 0x90;
        const PROBE: u32 = 0x94;
        const INNER_SUB: u32 = 0x48;
        const FLAG_BYTE: u32 = 0x019f32ac;
        const GLOB_LO: u32 = 0x019f3278;
        const GLOB_HI: u32 = 0x019f3280;

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
        let probe = this.wrapping_add(PROBE);
        let inner = field(this, INNER);
        loop {
            match field(this, STATE) {
                0 => {
                    lf_checker_rt::callee_thiscall!(1, u32, inner.wrapping_add(0xbb0));
                    lf_checker_rt::callee_thiscall!(2, u32, inner);
                    let a4 = field(this, 0xa4);
                    let r: u32 = if a4 == 0 {
                        lf_checker_rt::callee_thiscall!(
                            3, u32, inner.wrapping_add(INNER_SUB),
                            field(this, 0x9c),
                            field(this, 0xa8),
                            this.wrapping_add(0xb0),
                            this.wrapping_add(0x94)
                        )
                    } else {
                        lf_checker_rt::callee_thiscall!(
                            4, u32, inner.wrapping_add(INNER_SUB),
                            field(this, 0x9c),
                            a4,
                            field(this, 0xa8),
                            field(this, 0xac),
                            this.wrapping_add(0xb0),
                            field(this, 0x4bc),
                            this.wrapping_add(0x94)
                        )
                    };
                    if r & 0xff == 0 {
                        vcall(this, 0, 0);
                    } else {
                        wr32(this.wrapping_add(STATE), 1);
                    }
                }
                1 => {
                    let p = rd32(probe);
                    if p == 3 {
                        lf_checker_rt::callee_thiscall!(5, u32, this);
                        wr32(this.wrapping_add(STATE), 2);
                    } else if p != 1 {
                        vcall(this, 0, 0);
                    }
                }
                2 => {
                    if field(this, 0x9c) != 0 {
                        wr32(this.wrapping_add(STATE), 4);
                    } else {
                        let flag: u8 = lf_checker_rt::global::<u8>(FLAG_BYTE).read_unaligned();
                        if flag & 2 == 0 {
                            wr32(this.wrapping_add(STATE), 4);
                        } else {
                            // Both halves, lower first, as the frame holds them.
                            let w0 = lf_checker_rt::global::<u32>(GLOB_LO).read_unaligned();
                            let w1 = lf_checker_rt::global::<u32>(GLOB_LO + 4).read_unaligned();
                            let w2 = lf_checker_rt::global::<u32>(GLOB_HI).read_unaligned();
                            let w3 = lf_checker_rt::global::<u32>(GLOB_HI + 4).read_unaligned();
                            let frame = [w0, w1, w2, w3];
                            let r: u32 = lf_checker_rt::callee_thiscall!(
                                6, u32, inner.wrapping_add(INNER_SUB),
                                frame.as_ptr() as u32,
                                this.wrapping_add(0xa0),
                                1,
                                this.wrapping_add(0x94)
                            );
                            wr32(this.wrapping_add(STATE), if r & 0xff == 0 { 4 } else { 3 });
                        }
                    }
                }
                3 => {
                    let p = rd32(probe);
                    if p == 3 {
                        vcall(this, 1, 0);
                    } else if p != 1 {
                        wr32(this.wrapping_add(STATE), 4);
                    }
                }
                4 => {
                    let r: u32 = lf_checker_rt::callee_thiscall!(
                        7, u32, inner.wrapping_add(INNER_SUB),
                        this.wrapping_add(0x94)
                    );
                    if r & 0xff == 0 {
                        vcall(this, 0, 0);
                    } else {
                        wr32(this.wrapping_add(STATE), 5);
                    }
                }
                5 => {
                    if rd32(probe) != 1 {
                        vcall(this, 0, 0);
                    }
                }
                _ => {}
            }
            if rd32(probe) == 1 {
                break;
            }
            let r: u32 = lf_checker_rt::callee_thiscall!(8, u32, this);
            if r & 0xff == 0 {
                break;
            }
        }
        0
    }
});
