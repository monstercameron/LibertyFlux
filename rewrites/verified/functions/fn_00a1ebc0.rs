// original: 0x00a1ebc0 cam_follow_approach_gate (proposed)

/// Runs the approach worker while a target is fresh and closing.
///
/// `a1` selects the target record for the predicate callee and `a2`
/// points at its closing-speed float; `a0`/`a3` are passed through to the
/// worker. When the predicate rejects, the freshness counter at
/// `this + COUNT_OFF` is decremented if positive, the closing bit in the
/// flag at `+FLAG_OFF` is cleared and the function returns. Otherwise the
/// counter is reloaded from its global, and when the speed's magnitude
/// exceeds `SPEED_EPS` (0.01) the closing bit is set. A set closing bit
/// returns at once; otherwise the worker callee runs on
/// (`a1`, `a0`, `this+W0_OFF`, `this+W1_OFF`, `a2`, `a3`) — note `a1`, not
/// `a2`, in `ecx` — and its answer feeds the sink callee. Returns nothing.
///
/// Original: 0x00a1ebc0 (thiscall, four stack words).
lf_checker_rt::export!(thiscall, rw_00a1ebc0(this: u32, a0: u32, a1: u32, a2: u32, a3: u32) -> u32 {
    unsafe {
        const C_PRED: u32 = 1;
        const C_WORKER: u32 = 2;
        const C_SINK: u32 = 3;
        const COUNT_OFF: u32 = 0x33c;
        const FLAG_OFF: u32 = 0x38e;
        const W0_OFF: u32 = 0x304;
        const W1_OFF: u32 = 0x308;
        const COUNT_GLOBAL: u32 = 0x0103_bfe8;
        const CLOSING_BIT: u8 = 2;
        const ABS_MASK: u32 = 0x7fff_ffff;
        const SPEED_EPS: f32 = f32::from_bits(0x3c23_d70a); // 0.01
        unsafe fn rd32(x: u32) -> u32 {
            unsafe { (x as *const u32).read_unaligned() }
        }
        let pred: u32 = lf_checker_rt::callee_thiscall!(C_PRED, u32, a1);
        if pred as u8 == 0 {
            let c = rd32(this + COUNT_OFF);
            if (c as i32) > 0 {
                ((this + COUNT_OFF) as *mut u32).write_unaligned(c - 1);
            }
            let flag = (this + FLAG_OFF) as *mut u8;
            flag.write(flag.read() & !CLOSING_BIT);
            return 0;
        }
        let g = lf_checker_rt::global::<u32>(COUNT_GLOBAL).read_unaligned();
        ((this + COUNT_OFF) as *mut u32).write_unaligned(g);
        let speed = f32::from_bits(rd32(a2) & ABS_MASK);
        if speed > SPEED_EPS {
            let flag = (this + FLAG_OFF) as *mut u8;
            flag.write(flag.read() | CLOSING_BIT);
        }
        if ((this + FLAG_OFF) as *const u8).read() & CLOSING_BIT != 0 {
            return 0;
        }
        let u = lf_checker_rt::callee_thiscall!(
            C_WORKER, u32, a1, a0, this + W0_OFF, this + W1_OFF, a2, a3
        );
        lf_checker_rt::callee_thiscall!(C_SINK, u32, u);
        0
    }
});
