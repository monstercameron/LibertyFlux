// original: 0x00a0b1a0 radar_blip_flush_mission (proposed)
/// Drop the dead mission blips from the 256-slot table.
///
/// Each slot tagged 0x12 is liveness-checked; one that fails is reset to
/// the empty state. (Both callees are stubs under the checker, so the
/// proof is the call sequence; the original leaves eax holding whatever
/// the last call answered, which is undefined when no slot triggers, so
/// the return is not compared.) Thiscall, no stack words.
lf_checker_rt::export!(thiscall, rw_00a0b1a0(this: u32) -> u32 {
    unsafe {
        const MISSION_TAG: u8 = 0x12;
        const IS_LIVE: u32 = 0;
        const RESET: u32 = 1;
        const COUNT: u32 = 0x100;
        const STRIDE: u32 = 0x2c;
        let mut slot = this + 4;
        let mut i = 0u32;
        while i < COUNT {
            if (slot as *const u8).read() == MISSION_TAG {
                let h = ((slot + 4) as *const u32).read_unaligned();
                let live: u32 = lf_checker_rt::callee_cdecl!(IS_LIVE, u32, h);
                if (live & 0xff) == 0 {
                    lf_checker_rt::callee_thiscall!(RESET, u32, slot);
                }
            }
            i += 1;
            slot += STRIDE;
        }
        0
    }
});
