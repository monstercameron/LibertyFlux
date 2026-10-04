// original: 0x00a72be0 layered_gate_float_tail (proposed)
/// Twin of the signed-tail gate: the measurement must exceed the global
/// limit as floats instead.
///
/// `cdecl`, no stack words. Same shape: record (callee 1), state
/// (callee 2), two gate rounds (callee 3) with the early-1 xor test on
/// bytes `+0x274c/0x274e/0x274f` against 0x7F, the flag at `+0x328c` and
/// the global enable. The tail converts the measurement to float and the
/// limit byte to float and requires strictly greater, which the rewrite
/// states directly (integer-to-float conversion never yields NaN, so the
/// original's below-or-equal exit is exactly `!(m > lim)`). The tail call
/// (callee 6, stdcall with 0, `limit + 0x7f`, 0xFF) feeds the confirm
/// call (callee 7) as in the twin.
lf_checker_rt::export!(cdecl, rw_00a72be0() -> u8 {
    unsafe {
        const ENABLE: u32 = 0x0103ce47;
        const LIMIT: u32 = 0x0103ce44;
        const K_HI: u32 = 0x274f;
        const K_MID: u32 = 0x274e;
        const K_LO: u32 = 0x274c;
        const FLAG: u32 = 0x328c;
        const BOUND: u8 = 0x7f;
        let rb = |p: u32| (p as *const u8).read();
        let rec = lf_checker_rt::callee_cdecl!(1, u32,);
        if rec == 0 {
            return 0;
        }
        let st = lf_checker_rt::callee_thiscall!(2, u32, rec);
        if st == 0 {
            return 0;
        }
        if lf_checker_rt::callee_cdecl!(3, u32,) as u8 != 0 {
            let base = rb(st + K_LO);
            if rb(st + K_MID) ^ base > BOUND
                && rb(st + K_HI) ^ base <= BOUND
            {
                return 1;
            }
        }
        if lf_checker_rt::callee_cdecl!(3, u32,) as u8 == 0 {
            return 0;
        }
        if rb(st + FLAG) == 0 {
            return 0;
        }
        if rb(lf_checker_rt::relocated(ENABLE)) == 0 {
            return 0;
        }
        let m = lf_checker_rt::callee_thiscall!(4, u32, st);
        let n = lf_checker_rt::callee_cdecl!(5, u32, m);
        let dl = rb(lf_checker_rt::relocated(LIMIT));
        if !((n as i32 as f32) > f32::from(u32::from(dl) as u16 as u8)) {
            return 0;
        }
        let t = u32::from(dl.wrapping_add(0x7f));
        let q = lf_checker_rt::callee_stdcall!(6, u32, 0, t, 0xff);
        let m2 = lf_checker_rt::callee_thiscall!(4, u32, st);
        (lf_checker_rt::callee_thiscall!(7, u32, m2, q) as u8 != 0) as u8
    }
});
