// original: 0x00ABED30 CUseLightsInAreaDC::invoke

/// Submit this light-area command to its six argument callback.
///
/// The receiver arrives in ECX. The callback receives a pointer to the
/// receiver's inline data at offset 16, the word at offset 48, the word at offset 52,
/// the byte at offset 56 zero-extended to a word, the fixed command tag
/// `LIGHT_AREA_TAG`, and a final zero. It uses the C calling convention.
lf_checker_rt::export!(thiscall, rw_00abed30(this: u32) -> () {
    const CALLBACK_SITE_ID: u32 = 1;
    const INLINE_DATA: u32 = 0x10;
    const FIRST_WORD: usize = 0x30;
    const SECOND_WORD: usize = 0x34;
    const AREA_BYTE: usize = 0x38;
    const LIGHT_AREA_TAG: u32 = 0xff5b_aaed;

    let receiver = this as *const u8;
    let first_word = unsafe { receiver.add(FIRST_WORD).cast::<u32>().read_unaligned() };
    let second_word = unsafe { receiver.add(SECOND_WORD).cast::<u32>().read_unaligned() };
    let area = unsafe { receiver.add(AREA_BYTE).read() };
    let _: u32 = lf_checker_rt::callee_cdecl!(
        CALLBACK_SITE_ID,
        u32,
        this.wrapping_add(INLINE_DATA),
        first_word,
        second_word,
        u32::from(area),
        LIGHT_AREA_TAG,
        0u32,
    );
});
