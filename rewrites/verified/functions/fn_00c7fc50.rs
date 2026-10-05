// original: 0x00c7fc50 CTaskComplexMobileChatScenario::vf6

/// Scenario blend weight: 25.0 when bit 1 of `this+0xc` is set, else -1.0.
///
/// Shifts the flag word at `this+0xc` right by one and tests bit 0 (i.e. the
/// original bit 1). Set yields the active weight 25.0, clear yields -1.0. The
/// original loads these from two four-byte constants in read-only data; the
/// values are embedded here as literals. The one stack word is popped but
/// never read. Returned in ST0.
///
/// Original: thiscall, one stack word (the callee pops 4 bytes), float result in ST0.
lf_checker_rt::export!(thiscall, rw_00c7fc50(this: u32, _u: u32) -> f32 {
    unsafe {
        const FLAG_OFF: u32 = 0x0c;
        const ACTIVE_W: f32 = 25.0;
        const IDLE_W: f32 = -1.0;
        let flags = ((this + FLAG_OFF) as *const u32).read_unaligned();
        if (flags >> 1) & 1 != 0 { ACTIVE_W } else { IDLE_W }
    }
});
