// original: 0x0099e220 audio_flag_gated_probe
/// Probe the shared audio object when the flag byte at +0xC1 is set.
///
/// Returns 0 with no call when the flag is clear; otherwise calls the probe
/// helper (thiscall/0 on a fixed global object, stubbed by the checker) and
/// returns 1 when the low byte of its answer is zero, else 0.
export!(thiscall, rw_0099e220(obj: u32) -> u32 {
    unsafe {
        if *((obj.wrapping_add(0xC1)) as *const u8) == 0 {
            return 0;
        }
        let probe: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(callee_addr(1) as usize);
        if probe(relocated(0x1284A60)) & 0xFF == 0 {
            1
        } else {
            0
        }
    }
});
