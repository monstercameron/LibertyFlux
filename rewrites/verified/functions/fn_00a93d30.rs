// original: 0x00a93d30 stream_start_update (proposed)

/// Mark the stream busy and start an update pass if the gate allows.
///
/// Sets bit 1 of the state word at `[this]`, then asks the gate callee
/// about the argument. A zero low byte clears the bit again and returns
/// the gate answer. Otherwise the clock globals are reset (stamp 0,
/// counters 1), the first range is notified, the kick callee runs, and its
/// answer is returned. (The listed size 84 stops mid-function; the two
/// return sites span 90 bytes.)
///
/// Original: thiscall, one stack argument.
/// Three callees (thiscall 1 / stdcall 1 / cdecl 1 args).
lf_checker_rt::export!(thiscall, rw_00a93d30(this: u32, arg: u32) -> u32 {
    unsafe {
        const BUSY_BIT: u32 = 2;
        const STAMP_G: u32 = 0x012fb39c;
        const COUNT_A_G: u32 = 0x012fb3a0;
        const COUNT_B_G: u32 = 0x012fb3a4;
        const GATE: u32 = 0;
        const NOTIFY_FIRST: u32 = 1;
        const KICK: u32 = 2;
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        wr32(this, rd32(this) | BUSY_BIT);
        let gate: u32 = lf_checker_rt::callee_thiscall!(GATE, u32, this, arg);
        if gate & 0xff == 0 {
            wr32(this, rd32(this) & !BUSY_BIT);
            return gate;
        }
        wr32(lf_checker_rt::relocated(STAMP_G), 0);
        wr32(lf_checker_rt::relocated(COUNT_A_G), 1);
        wr32(lf_checker_rt::relocated(COUNT_B_G), 1);
        let _: u32 = lf_checker_rt::callee_stdcall!(NOTIFY_FIRST, u32, 0);
        lf_checker_rt::callee_cdecl!(KICK, u32, 0)
    }
});
