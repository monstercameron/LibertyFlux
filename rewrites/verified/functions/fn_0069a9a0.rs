// original: 0x0069a9a0 rage::crAnimChannelRawInt::vf19
/// Channel allocation size: the key count at `this+0xC` times 4 plus 16.
export!(thiscall, rw_0069a9a0(this: u32) -> u32 {
    unsafe {
        let count = *((this + 12) as *const u16) as u32;
        count.wrapping_mul(4).wrapping_add(0x10)
    }
});
