// original: 0x00648eb0 rage::rmPtfxShaderVar_Float4::vf1
/// Push the four-float value into a parameter block through the shared
/// writer: the value address and length go with the parameter index, the
/// block cursor and tag words.
export!(thiscall, rw_00648eb0(this: *const u8, param: *const u8) -> u32 {
    unsafe {
        let count = *(this.add(0x0c) as *const u32);
        let values = (this as u32).wrapping_add(0x20);
        let block = (param as u32).wrapping_add(0x14);
        let target = *(param.add(0x18) as *const u32);
        callee_thiscall!(1, u32, target, block, count, values, 0x10, 1, 5)
    }
});
