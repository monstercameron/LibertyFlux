// original: 0x0088F500 rage::audVoicePcAdpcm::vf4
// 0088F500 rage::audVoicePcAdpcm::vf4: when the retrigger bit is set, poll
// the voice and program the child with the bit derived from the flags, then
// clear the bit.
export!(thiscall, rw_0088f500(this: *mut u8) -> () {
    unsafe {
        if (*this.add(0x8C) & 8) != 0 {
            callee_thiscall!(1, u32, this as u32);
            let flags = *this.add(0x8C);
            let sel = (((flags >> 3) | flags) >> 1) & 1;
            callee_thiscall!(2, u32, *(this.add(0x140) as *const u32), sel as u32);
            *this.add(0x8C) &= 0xF7;
        }
    }
});
