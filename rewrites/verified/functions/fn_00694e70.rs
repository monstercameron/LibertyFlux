// original: 0x00694e70 filter_set_mode
/// Store the mode flag, refresh the derived key, return the key.
///
/// Same shape as `rs80_694e40` but for the single mode byte: stores it,
/// re-derives the key through the same helper, stores and returns the key.
export!(thiscall, rs80_694e70(this: *mut u8, mode: u32) -> u32 {
    unsafe {
        *((this).add(0x16)) = mode as u8;
        let key: u32 = callee_thiscall!(1, u32, this as u32);
        *((this).add(8) as *mut u32) = key;
        key
    }
});
