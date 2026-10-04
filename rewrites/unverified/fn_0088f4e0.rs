// original: 0x0088F4E0 rage::audVoicePcAdpcm::vf5
// 0088F4E0 rage::audVoicePcAdpcm::vf5: clear flag bits 0 and 3, then forward
// to the child object at +0x140 (tail call).
export!(thiscall, rw_0088f4e0(this: *mut u8) -> u32 {
    unsafe {
        let flags = this.add(0x8C);
        *flags &= 0xF6;
        let next = *(this.add(0x140) as *const u32);
        callee_thiscall!(1, u32, next)
    }
});
