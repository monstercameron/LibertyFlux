// original: 0x008a29a0 rage::audSimpleSound::audSimpleSound
/// Constructor for the simple sound object.
///
/// Runs the base constructor (its answer is discarded: the original forces it
/// to all-ones), writes the two all-ones marker words, installs this class's
/// virtual table, and returns `this`.
export!(thiscall, rw_008a29a0(this: *mut u8) -> u32 {
    const VTABLE: u32 = 0xe7a9fc;
    unsafe {
        callee_thiscall!(1, u32, this as u32);
        *(this.add(0xb6) as *mut u16) = 0xffff;
        *(this.add(0xb4) as *mut u16) = 0xffff;
        *(this as *mut u32) = relocated(VTABLE);
        this as u32
    }
});
