// original: 0x00648dd0 rage::rmPtfxShaderVar_Float3::vf2
/// Refresh the three-float value from a parameter block: when the index is
/// nonzero, follow the block's entry table (1-based) and copy all four words
/// of the sixteen-byte slot.
export!(thiscall, rw_00648dd0(this: *mut u8, param: *const u8) -> () {
    unsafe {
        let index = *(this.add(0x0c) as *const u32);
        if index != 0 {
            let entries = *(param.add(0x14) as *const *const u32);
            let slot = *entries.add(index as usize - 1) as *const u32;
            *(this.add(0x20) as *mut u32) = *slot;
            *(this.add(0x24) as *mut u32) = *slot.add(1);
            *(this.add(0x28) as *mut u32) = *slot.add(2);
            *(this.add(0x2c) as *mut u32) = *slot.add(3);
        }
    }
});
