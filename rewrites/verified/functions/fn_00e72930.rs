// original: 0x00e72930 release_and_null_16b9db8
/// Release of the pointer at 0x16B9DB8, then null the slot.
///
/// Unconditionally frees the pointer through the release helper (cdecl/1,
/// stubbed) and writes zero back. Returns the helper's answer, matching EAX.
export!(cdecl, rw_00e72930() -> u32 {
    unsafe {
        let answer: u32 = callee_cdecl!(1, u32, *global::<u32>(0x16B9DB8));
        *global::<u32>(0x16B9DB8) = 0;
        answer
    }
});
