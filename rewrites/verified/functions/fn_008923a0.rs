// original: 0x008923A0 audsound_store_word_7c
/// Stores the low 16 bits of the argument into the word at offset 0x7c.
///
/// `this` is the sound object, `v` a 32-bit value of which only the low word
/// is kept. The upper half of `v` is ignored. The return register is left
/// untouched (it keeps the caller's entry EAX), so no return channel is
/// compared. Original: 0x008923A0 (thiscall, one stack word).
export!(thiscall, rw_008923A0(this: *mut u8, v: u32) -> () {
    unsafe {
        *(this.add(0x7c) as *mut u16) = v as u16;
    }
});
