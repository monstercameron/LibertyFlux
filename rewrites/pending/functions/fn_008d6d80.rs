// original: 0x008d6d80 flag_test
/// Follow two links and test bits 6..9 of a status word.
///
/// Returns 0 when the second link is null. Otherwise masks the word at
/// offset +0x28 with 0x3C0: full EAX is the masked value with its low byte
/// replaced by 1 on match (0x100) and 0 otherwise, because the original only
/// ever writes AL.
#[allow(non_snake_case)]
export!(cdecl, rw_008d6d80(handle: *const *const u32) -> u32 {
    unsafe {
        let inner = *handle;
        let obj = *(inner.byte_add(0x0C) as *const *const u32);
        if obj.is_null() {
            return 0;
        }
        let masked = *(obj.byte_add(0x28)) & 0x3C0;
        (masked & !0xFF) | ((masked == 0x100) as u32)
    }
});
