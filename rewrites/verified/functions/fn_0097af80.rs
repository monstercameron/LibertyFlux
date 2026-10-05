// original: 0x0097af80 audio_iface_forward_if_armed
/// Flag gate: forward live flagged objects, ignore the rest.
///
/// Takes an unused first word and an object pointer on the stack. When the
/// pointer is null, or the object's clear flag is set, or its armed flag
/// is clear, the pointer itself is the result and nothing is forwarded.
/// Otherwise control transfers to the shared successor with both words,
/// returning whatever that call answers.
export!(cdecl, rw_0097af80(unused_: u32, obj: u32) -> u32 {
    unsafe {
        if obj == 0 {
            return obj;
        }
        if *((obj.wrapping_add(0x218)) as *const u8) != 0 {
            return obj;
        }
        if *((obj.wrapping_add(0x219)) as *const u8) == 0 {
            return obj;
        }
        callee_cdecl!(1, u32, unused_, obj)
    }
});
