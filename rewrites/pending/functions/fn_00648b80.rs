// original: 0x00648b80 rage::rmPtfxShaderVar_Float::vf2
/// Refresh the float value from a parameter block: when the index is nonzero,
/// follow the block's entry table (1-based) and copy the value word; the
/// bits are copied as-is, so the copy is bit-exact.
export!(thiscall, rw_00648b80(this: *mut u8, param: *const u8) -> () {
    unsafe {
        let index = *(this.add(0x0c) as *const u32);
        if index != 0 {
            let entries = *(param.add(0x14) as *const *const u32);
            let slot = *entries.add(index as usize - 1);
            *(this.add(0x20) as *mut u32) = *(slot as *const u32);
        }
    }
});
