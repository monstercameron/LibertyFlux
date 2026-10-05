// original: 0x00c8a800 audio_slot_liveness_gate
/// Audio slot liveness gate (two words in, one word out).
///
/// Takes an opaque first word and an object pointer. Passes the gate only
/// when three conditions all hold: the first tag word in the object reads
/// as positive or negative zero, the second tag word reads above zero
/// (either check fails on a not-a-number), and the object's tag byte is
/// non-zero. On the passing path the first word is handed on unchanged
/// together with a pointer at the object's voice view, the engine mixer
/// object is selected, and that call's answer is the result. On any
/// failing path the result keeps the entry accumulator's upper bytes with
/// the low byte set to one when both float checks passed and zero
/// otherwise; the contract pins the entry accumulator so those upper
/// bytes are a declared input. The first word is never inspected, only
/// forwarded. The incoming second-word slot is overwritten in place on
/// the passing path, so the stack comparison is off and the forwarded
/// words are compared through the call log instead.
export!(stdcall, rw_00c8a800(first: u32, obj: u32) -> u32 {
    unsafe {
        let tag_zero = *(obj.wrapping_add(0x64) as *const f32);
        let tag_pos = *(obj.wrapping_add(0x68) as *const f32);
        let latch: u8 = u8::from(tag_zero == 0.0 && tag_pos > 0.0);
        if *(obj as *const u8) == 0 || latch == 0 {
            return 0xA5A55A00 | u32::from(latch);
        }
        let view = obj.wrapping_add(0x20);
        callee_thiscall!(1, u32, relocated(0x13B0EB0), first, view)
    }
});
