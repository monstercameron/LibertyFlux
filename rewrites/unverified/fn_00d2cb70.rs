// original: 0x00D2CB70 CTaskComplexUseLadderOnRoute::vf20

/// Timer-gated route-ladder task update: either takes the gate path and
/// returns 0, or, when the gate fails, refreshes the sub-task's stored
/// position from a helper.
///
/// `this` is the task object (thiscall, one opaque stack word passed through
/// to the poll call, callee pops 4). The stack argument is never read here.
///
/// Gate: when byte `+0x44` is set, and after latching the game timer into
/// `+0x3c` when byte `+0x45` is set (clearing it), the task proceeds only if
/// `start + duration` (`+0x3c`, `+0x40`) has reached the game timer, compared
/// as SIGNED 32-bit. Unless flag bit 1 at `+0xc` is set, virtual slot 5 of
/// the task is polled as `wants(arg, 1, 0)`; a zero low byte fails the gate,
/// otherwise bit 2 is set at `+0xc`. The gate path returns 0.
///
/// Otherwise (gate flag clear, timer not reached, or poll refused): when the
/// pointer at `+0x34` is null, or the object at `+0x8` does not report type
/// `0x3a6` from its virtual slot 3, the object at `+0x8` is returned
/// unchanged. Else four floats (`+0x20..+0x28`, `+0x30`) are staged; if the
/// word at `+0x20` of the `+0x34` object is zero it is first initialised by
/// two direct calls (a thiscall/0 then a thiscall/1 on `+0x10`); then a
/// cdecl/3 helper fills four output floats from the staged values, which are
/// stored at `+0xc0..+0xcc` of the `+0x8` object, plus the staged `+0x30`
/// float at `+0xd8`. Returns the `+0x8` object.
///
/// Edge cases: all float moves are bitwise copies (no arithmetic, NaNs pass
/// through); the timer comparison is signed, so large unsigned values behave
/// as negatives; the low byte of the poll result alone decides.
lf_checker_rt::export!(thiscall, rw_00D2CB70(this: u32, arg: u32) -> u32 {
    unsafe {
        const TIMER: u32 = 0x0117_35B4;
        const TASK_FLAGS: u32 = 0x0c;
        const POLL_DONE_BIT: u32 = 0x1;
        const POLL_OK_BIT: u32 = 0x2;
        const SUB_TASK: u32 = 0x08;
        const LADDER_DATA: u32 = 0x34;
        const TIMER_START: u32 = 0x3c;
        const TIMER_DUR: u32 = 0x40;
        const TIMER_ARMED: u32 = 0x44;
        const TIMER_LATCH: u32 = 0x45;
        const POS_X: u32 = 0x20;
        const POS_Y: u32 = 0x24;
        const POS_Z: u32 = 0x28;
        const POS_W: u32 = 0x30;
        const EXPECTED_TYPE: u32 = 0x3a6;
        const INIT_FLAG: u32 = 0x20;
        const INIT_OBJ: u32 = 0x10;
        const OUT_X: u32 = 0xc0;
        const OUT_Y: u32 = 0xc4;
        const OUT_Z: u32 = 0xc8;
        const OUT_W: u32 = 0xcc;
        const OUT_EXTRA: u32 = 0xd8;
        const VT_POLL: u32 = 0x14;
        const VT_TYPE: u32 = 0x0c;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rd8(a: u32) -> u8 {
            unsafe { (a as *const u8).read() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn wr8(a: u32, v: u8) {
            unsafe { (a as *mut u8).write(v) }
        }
        #[inline(always)]
        unsafe fn timer() -> u32 {
            unsafe { lf_checker_rt::global::<u32>(TIMER).read() }
        }

        let mut gated = false;
        if rd8(this + TIMER_ARMED) != 0 {
            if rd8(this + TIMER_LATCH) != 0 {
                wr32(this + TIMER_START, timer());
                wr8(this + TIMER_LATCH, 0);
            }
            let end = rd32(this + TIMER_DUR).wrapping_add(rd32(this + TIMER_START));
            if (end as i32) <= (timer() as i32) {
                gated = true;
            }
        }
        if gated && (rd32(this + TASK_FLAGS) & POLL_DONE_BIT) == 0 {
            let vt = rd32(this);
            let poll: extern "thiscall" fn(u32, u32, u32, u32) -> u32 =
                unsafe { core::mem::transmute(rd32(vt + VT_POLL)) };
            if (poll(this, arg, 1, 0) & 0xFF) == 0 {
                gated = false;
            } else {
                wr32(this + TASK_FLAGS, rd32(this + TASK_FLAGS) | POLL_OK_BIT);
            }
        }
        if gated {
            return 0;
        }

        let sub = rd32(this + SUB_TASK);
        let data = rd32(this + LADDER_DATA);
        if data != 0 {
            let vt = rd32(sub);
            let get_type: extern "thiscall" fn(u32) -> u32 =
                unsafe { core::mem::transmute(rd32(vt + VT_TYPE)) };
            if get_type(sub) == EXPECTED_TYPE {
                let mut out = [rd32(this + POS_X), rd32(this + POS_Y), rd32(this + POS_Z), 0u32];
                let mut extra = rd32(this + POS_W);
                if rd32(data + INIT_FLAG) == 0 {
                    lf_checker_rt::callee_thiscall!(3, u32, data);
                    lf_checker_rt::callee_thiscall!(4, u32, data + INIT_OBJ, rd32(data + INIT_FLAG));
                }
                lf_checker_rt::callee_cdecl!(
                    5, u32,
                    out.as_mut_ptr() as u32,
                    core::ptr::addr_of_mut!(extra) as u32,
                    rd32(data + INIT_FLAG)
                );
                wr32(sub + OUT_X, out[0]);
                wr32(sub + OUT_Y, out[1]);
                wr32(sub + OUT_Z, out[2]);
                wr32(sub + OUT_W, out[3]);
                wr32(sub + OUT_EXTRA, extra);
            }
        }
        sub
    }
});
