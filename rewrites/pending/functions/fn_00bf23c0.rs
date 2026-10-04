// original: 0x00bf23c0 decode_word_to_buffer
/// Decode one packed word straight into the caller's buffer.
///
/// Thinnest wrapper in the batch: decodes `this + idx*4 + 0x370` (`idx` is
/// the low byte of the last argument) through callee 1 directly into `out`.
/// Returns callee 1's answer.
export!(thiscall, rw_bf23c0(this_obj: u32, out: u32, idx_arg: u32) -> u32 {
    unsafe {
        let idx = (idx_arg & 0xff) as u32;
        let packed = *((this_obj + idx * 4 + 0x370) as *const u32);
        callee_cdecl!(1, u32, out, packed)
    }
});
