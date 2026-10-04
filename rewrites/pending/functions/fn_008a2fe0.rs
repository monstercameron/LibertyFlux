// original: 0x008a2fe0 rage::audMultitrackSound::audMultitrackSound
/// Constructor for the multitrack sound object.
///
/// Runs the base constructor, installs this class's virtual table, writes the
/// two all-ones sentinel words, clears the armed flag byte, and returns
/// `this`.
export!(thiscall, rw_008a2fe0(this: *mut u8) -> u32 {
    const VTABLE: u32 = 0xe7ab64;
    unsafe {
        callee_thiscall!(1, u32, this as u32);
        let w = |off: usize| this.add(off) as *mut u32;
        *w(0x00) = relocated(VTABLE);
        *w(0x48) = 0xffff_ffff;
        *w(0x4c) = 0xffff_ffff;
        *this.add(0xb4) = 0;
        this as u32
    }
});
