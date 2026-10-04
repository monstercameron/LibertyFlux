// original: 0x00648af0 rage::rmPtfxShaderVar_Bool::vf1
/// Push the boolean value into a parameter block through the shared writer:
/// the entry address is the global value table plus the value index, sent
/// with the parameter index, the block cursor and tag words.
export!(thiscall, rw_00648af0(this: *const u8, param: *const u8) -> u32 {
    unsafe {
        let value = *this.add(0x20) as u32;
        let entry = relocated(0x0106B550).wrapping_add(value.wrapping_mul(4));
        let count = *(this.add(0x0c) as *const u32);
        let block = (param as u32).wrapping_add(0x14);
        let target = *(param.add(0x18) as *const u32);
        callee_thiscall!(1, u32, target, block, count, entry, 4, 1, 7)
    }
});
