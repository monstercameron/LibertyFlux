// original: 0x00648ad0 rage::rmPtfxShaderVar_Bool::vf2
/// Refresh the boolean value from a parameter block: when the index is
/// nonzero, follow the block's entry table (1-based) to the value slot and
/// store whether it is nonzero; otherwise keep the incoming pointer. Returns
/// the source word with its low byte replaced by the flag, like the original.
export!(thiscall, rw_00648ad0(this: *mut u8, param: *const u8) -> u32 {
    unsafe {
        let index = *(this.add(0x0c) as *const u32);
        let mut source = param as u32;
        if index != 0 {
            let entries = *(param.add(0x14) as *const *const u32);
            let slot = *entries.add(index as usize - 1);
            source = *(slot as *const u32);
        }
        let flag = (source != 0) as u32;
        *this.add(0x20) = flag as u8;
        (source & !0xFF) | flag
    }
});
