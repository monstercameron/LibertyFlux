// original: 0x00C319B0 CPlane::vf110

/// Read a signed 16-bit model index at byte offset `0x2e`, look up its model
/// record in the global pointer table, then map the record type at offset
/// `0xc4` to an integer result: type `0x55` gives zero, `0x54` gives two,
/// `0x56` gives three, and all other types give one. This thiscall method has
/// no stack arguments and returns the result in EAX.
lf_checker_rt::export!(thiscall, rw_00c319b0(this: *mut u8) -> u32 {
    const MODEL_INDEX_OFFSET: usize = 0x2e;
    const MODEL_TYPE_OFFSET: usize = 0xc4;
    const MODEL_POINTER_TABLE_VA: u32 = 0x01295cd8;
    const TYPE_ZERO: u32 = 0x55;
    const TYPE_TWO: u32 = 0x54;
    const TYPE_THREE: u32 = 0x56;

    unsafe {
        let model_index = this.add(MODEL_INDEX_OFFSET).cast::<i16>().read_unaligned() as i32;
        let entry_va = MODEL_POINTER_TABLE_VA.wrapping_add((model_index as u32).wrapping_mul(4));
        let model_info = lf_checker_rt::global::<u32>(entry_va).read_unaligned();
        let model_type = (model_info as *const u8).add(MODEL_TYPE_OFFSET).cast::<u32>().read_unaligned();
        match model_type {
            TYPE_ZERO => 0,
            TYPE_TWO => 2,
            TYPE_THREE => 3,
            _ => 1,
        }
    }
});
