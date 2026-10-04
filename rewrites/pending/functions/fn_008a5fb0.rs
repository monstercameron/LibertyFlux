// original: 0x008a5fb0 audio_call_unless_byte_ff
/// Call the audio worker on an object unless its tag byte says "none".
///
/// When byte +5 of the object is 0xFF the function returns with only AL
/// cleared (the upper bytes keep whatever the caller had in EAX, so only
/// the low byte is a meaningful result); otherwise it tail-calls the worker
/// with the object both as `this` and as the stack argument and returns its
/// answer.
export!(cdecl, rw_008a5fb0(p: *const u8) -> u32 {
    unsafe {
        if *p.add(5) == 0xFF {
            0
        } else {
            callee_thiscall!(1, u32, p as u32, p as u32)
        }
    }
});

