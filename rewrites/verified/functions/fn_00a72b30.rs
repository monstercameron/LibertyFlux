// original: 0x00a72b30 layered_gate_signed_tail (proposed)
/// True (1) when a chain of gates passes and a signed measurement clears
/// a global limit, else 0.
///
/// `cdecl`, no stack words. Fetches the record (callee 1) and the state
/// (callee 2 on the record); either null returns 0. The gate (callee 3)
/// is consulted twice: a pass on the first round compares
/// `byte[+0x273e] ^ byte[+0x273c]` against 0x7F, and a value above it
/// compares `byte[+0x273f] ^ byte[+0x273c]` the same way, returning 1
/// when the second lands at or below 0x7F. The second round must pass, a
/// state flag at `+0x328c` and a global enable must be set, then the
/// measurement (callee 4 on the state, callee 5 on its answer) must be
/// strictly below the negated global limit (signed). The tail call
/// (callee 6, stdcall with 0, 0, `0x80 - limit`) feeds the confirm call
/// (callee 7, thiscall with the callee-4 answer of the state and the tail
/// answer): a non-zero confirm returns 1, else 0.
lf_checker_rt::export!(cdecl, rw_00a72b30() -> u8 {
    unsafe {
        const ENABLE: u32 = 0x0103ce47;
        const LIMIT: u32 = 0x0103ce44;
        const K_HI: u32 = 0x273f;
        const K_MID: u32 = 0x273e;
        const K_LO: u32 = 0x273c;
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
        if (n as i32) >= (dl as i32).wrapping_neg() {
            return 0;
        }
        let t = u32::from(0x80u8.wrapping_sub(dl));
        let q = lf_checker_rt::callee_stdcall!(6, u32, 0, 0, t);
        let m2 = lf_checker_rt::callee_thiscall!(4, u32, st);
        (lf_checker_rt::callee_thiscall!(7, u32, m2, q) as u8 != 0) as u8
    }
});
