// original: 0x008a1760 rage::audCollapsingStereoSound::audCollapsingStereoSound
/// Constructor for the collapsing-stereo sound object.
///
/// Runs the base constructor, installs this class's virtual table, writes the
/// two all-ones sentinel words, zeroes the accumulator words and sets the
/// channel mask. Returns `this`.
export!(thiscall, rw_008a1760(this: *mut u8) -> u32 {
    const VTABLE: u32 = 0xe7a894;
    unsafe {
        callee_thiscall!(1, u32, this as u32);
        let w = |off: usize| this.add(off) as *mut u32;
        *w(0x00) = relocated(VTABLE);
        *w(0x48) = 0xffff_ffff;
        *w(0x4c) = 0xffff_ffff;
        *w(0xc0) = 0;
        *w(0xb8) = 0;
        *w(0xbc) = 0;
        *(this.add(0xcc) as *mut u16) = 0x01ff;
        this as u32
    }
});
