// original: 0x0097B6C0 audio_event_emit_with_bank_probe
/// Probe the bank record, then always emit through the audio interface.
///
/// The probe runs only when the flag bit is set, the bank link is non-null
/// and the bank record is unclaimed; the emit is unconditional. A null object
/// faults on the flag read, like the original. cdecl(arg0, obj).
export!(cdecl, rw_s103_97b6c0(a0: u32, a1: *const u8) -> u32 {
    unsafe {
        if (*a1.add(0x26C) & 4) != 0 {
            let t = *(((a1 as usize) + 0xB30) as *const u32);
            if t != 0 && *(((t as usize) + 0x1304) as *const u32) == 0 {
                callee_thiscall!(1, u32, t.wrapping_add(0x210));
            }
        }
        callee_thiscall!(2, u32, (a1 as usize as u32).wrapping_add(0x3C0), a0)
    }
});
