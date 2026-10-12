// original: 0x00AAC790 stream_release_gate

/// Compare a requested stream value with the active stream context's value.
///
/// The stdcall function receives one 32-bit requested value and returns a
/// boolean in AL. It reads a global stream index at image RVA `0x00c36f14`,
/// selects a pointer from the image table at RVA `0x00da8808`, follows the
/// context pointer at `+0x598`, then compares the word at `+0xb30` with the
/// request. The `-1` index follows the original zero-base load path; normal
/// proof trials use populated table slots supplied by the contract.
lf_checker_rt::export!(stdcall, rw_00aac790(requested_value: u32) -> u8 {
    const STREAM_INDEX_RVA: u32 = 0x00c3_6f14;
    const STREAM_TABLE_RVA: u32 = 0x00da_8808;
    const CONTEXT_POINTER_OFFSET: u32 = 0x598;
    const CURRENT_VALUE_OFFSET: u32 = 0xb30;
    const NO_STREAM_INDEX: u32 = u32::MAX;

    #[inline(always)]
    unsafe fn read_word(address: u32) -> u32 {
        unsafe { (address as *const u32).read_unaligned() }
    }

    unsafe {
        let image_base = lf_checker_rt::xbase();
        let stream_index = read_word(image_base.wrapping_add(STREAM_INDEX_RVA));
        let context = if stream_index == NO_STREAM_INDEX {
            0
        } else {
            read_word(
                image_base
                    .wrapping_add(STREAM_TABLE_RVA)
                    .wrapping_add(stream_index.wrapping_mul(4)),
            )
        };
        let current_context = read_word(context.wrapping_add(CONTEXT_POINTER_OFFSET));
        let current_value = read_word(current_context.wrapping_add(CURRENT_VALUE_OFFSET));
        u8::from(requested_value == current_value)
    }
});
