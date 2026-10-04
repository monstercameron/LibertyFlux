// original: 0x0097B620 audio_event_emit_nested
/// Emit an event with a code walked from a three-link chain.
///
/// Each null link returns 0 early. A null object faults on the first read,
/// like the original. cdecl(ignored, obj).
export!(cdecl, rw_s103_97b620(_a0: u32, a1: *const u8) -> u32 {
    unsafe {
        let t1 = *(((a1 as usize) + 0x2C4) as *const u32);
        if t1 == 0 {
            return 0;
        }
        let t2 = *(((t1 as usize) + 0x25C) as *const u32);
        if t2 == 0 {
            return 0;
        }
        let t3 = *(((t2 as usize) + 0x10) as *const u32);
        if t3 == 0 {
            return 0;
        }
        let code = core::ptr::read_unaligned((((t3 as usize) + 0x22)) as *const u32);
        callee_thiscall!(1, u32, (a1 as usize as u32).wrapping_add(0x3C0), code)
    }
});
