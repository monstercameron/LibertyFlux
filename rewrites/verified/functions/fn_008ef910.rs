// original: 0x008ef910 scaled_call_raw
/// Pass a record's scaled xor byte to the helper and return its raw answer.
///
/// Same call as [`rw_008ef8c0`] but returns immediately, so the result is
/// whatever the helper left in EAX (the scripted answer bits under the
/// checker, whose float stub also loads EAX).
export!(cdecl, rw_008ef910(p: u32) -> u32 {
    unsafe {
        let cl = *((p + 6) as *const u8) ^ *((p + 4) as *const u8);
        let y = (cl as f32) * *global::<f32>(0xFE86E8);
        callee_cdecl!(1, u32, y.to_bits(), 0x3E800000)
    }
});
