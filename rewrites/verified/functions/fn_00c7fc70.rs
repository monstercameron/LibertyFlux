// original: 0x00c7fc70 CTaskComplexMoveBetweenPointsScenario::vf6

/// Scenario blend weight with a waypoint-count gate.
///
/// Bit 1 of `this+0xc` set yields 25.0 at once. Otherwise the object must be
/// active (flag byte at `this+0x20` clear or word at `this+0x1c` non-zero) and
/// the waypoint count at `this+0x28` must be 0 or 1; then the weight is 25.0,
/// else -1.0. The constants come from read-only data in the original and are
/// embedded here as literals. The one stack word is popped but never read.
/// Returned in ST0.
///
/// Original: thiscall, one stack word (the callee pops 4 bytes), float result in ST0.
lf_checker_rt::export!(thiscall, rw_00c7fc70(this: u32, _u: u32) -> f32 {
    unsafe {
        const FLAG_OFF: u32 = 0x0c;
        const ACTIVE_OFF: u32 = 0x20;
        const STATE_OFF: u32 = 0x1c;
        const COUNT_OFF: u32 = 0x28;
        const ACTIVE_W: f32 = 25.0;
        const IDLE_W: f32 = -1.0;
        let flags = ((this + FLAG_OFF) as *const u32).read_unaligned();
        if (flags >> 1) & 1 != 0 {
            return ACTIVE_W;
        }
        let flag = ((this + ACTIVE_OFF) as *const u8).read();
        let state = ((this + STATE_OFF) as *const u32).read_unaligned();
        if flag != 0 && state == 0 {
            return IDLE_W;
        }
        let n = ((this + COUNT_OFF) as *const u32).read_unaligned();
        if n == 0 || n == 1 { ACTIVE_W } else { IDLE_W }
    }
});
