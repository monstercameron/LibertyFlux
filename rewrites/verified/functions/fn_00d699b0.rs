// original: 0x00d699b0 step_word_store_flag
// s16f19: push a word through the slot step, then store a flag byte
// (thiscall/2).
//
// Runs the slot step over this record's aux pointer with the word, reloads
// the aux pointer (the step may have replaced it) and stores the word's
// low byte on it: the step pops its own argument, so the store
// reads one slot above the pushed word. The original merges the stored
// byte into the step's EAX answer; the
// empty path leaves caller garbage in EAX, so the channel is unchecked.
export!(thiscall, rw_s16f19(this: *mut u8, word: u32, flag: u32) -> u32 {
    unsafe {
        let aux = *((this.add(0x10)) as *const u32);
        if aux == 0 {
            return 0; // original leaves entry EAX here; callers ignore it
        }
        let step: extern "thiscall" fn(u32, u32) -> u32 =
            core::mem::transmute(callee_addr(1) as usize);
        let answer = step(aux, word);
        let aux_now = *((this.add(0x10)) as *const u32);
        let flag_byte = (flag & 0xFF) as u8;
        *((aux_now.wrapping_add(0x1C)) as *mut u8) = flag_byte;
        (answer & 0xFFFFFF00) | flag_byte as u32
    }
});
