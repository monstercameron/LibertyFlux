// original: 0x0097B110 audio_event_emit_if_present
/// Emit an event through the object's audio interface when present.
///
/// No-op for a null object. Returns nothing meaningful (exit eax is entry
/// garbage on the early path, unchecked). cdecl(arg0, obj).
export!(cdecl, rw_s103_97b110(a0: u32, a1: *const u8) -> u32 {
    unsafe {
        if !a1.is_null() {
            callee_thiscall!(1, u32, (a1 as usize as u32).wrapping_add(0x3C0), a0);
        }
        0
    }
});
