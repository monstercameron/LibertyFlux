// original: 0x008d8ef0 file_empty_check_8ef0
/// Report whether both link slots of this object are clear.
///
/// Returns 1 when the dwords at 0xFE0 and 0xFE8 are both zero.
export!(thiscall, rw_008d8ef0(this: u32) -> u32 {
    unsafe {
        /// First link slot offset.
        const A: usize = 0xFE0 / 4;
        /// Second link slot offset.
        const B: usize = 0xFE8 / 4;
        let obj = this as *const u32;
        u32::from(obj.add(A).read() == 0 && obj.add(B).read() == 0)
    }
});
