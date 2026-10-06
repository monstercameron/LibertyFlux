// original: 0x00893290 audsound_build_entries_if_armed
/// Builds the voice entries once the state word reads 1, then disarms.
///
/// When the dword at `this+0x78` is not 1, returns at once (returning entry
/// EAX untouched, so the contract pins entry EAX to 0 and the rewrite
/// returns 0). Otherwise calls the builder callee (thiscall, one stack word)
/// with 1 when the byte at `this+0x4e` is clear and 0 when set, stores 2 to
/// `this+0x78`, and returns the callee's answer.
/// Original: 0x00893290 (thiscall, no stack arguments).
export!(thiscall, rw_00893290(this: *mut u8) -> u32 {
    unsafe {
        const STATE: usize = 0x78;
        const ARMED: u32 = 1;
        const DONE: u32 = 2;
        const FLAG: usize = 0x4e;
        const BUILDER: u32 = 1;
        if *(this.add(STATE) as *const u32) != ARMED {
            return 0;
        }
        let bit = if *this.add(FLAG) == 0 { 1 } else { 0 };
        let r: u32 = callee_thiscall!(BUILDER, u32, this as u32, bit);
        *(this.add(STATE) as *mut u32) = DONE;
        r
    }
});
