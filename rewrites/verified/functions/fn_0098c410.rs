// original: 0x0098C410 audio_voice_slot_gate

/// Accept a voice slot only when the flags byte at `this + 5` has bit 0x10
/// set and bits 0x20 and 0x40 clear, while bit 0x20 is set in the byte at
/// `this + 0x0A`. Return the predicate as a byte boolean in AL.
///
/// This is a `thiscall` leaf with no stack arguments.
lf_checker_rt::export!(thiscall, rw_audio_voice_slot_gate(this: u32) -> u32 {
    unsafe {
        const SLOT_FLAGS: u32 = 5;
        const GATE_FLAGS: u32 = 0x0A;
        let slot_flags = (this.wrapping_add(SLOT_FLAGS) as *const u8).read();
        let gate_flags = (this.wrapping_add(GATE_FLAGS) as *const u8).read();
        let accepted = slot_flags & 0x40 == 0
            && gate_flags & 0x20 != 0
            && slot_flags & 0x10 != 0
            && slot_flags & 0x20 == 0;
        u32::from(accepted)
    }
});
