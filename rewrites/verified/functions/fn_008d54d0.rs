// original: 0x008D54D0 ForwardArgsAfterFixedBox
/// Emits the fixed box through the first call, then forwards the three
/// arguments to the second call and returns its answer. The two global loads
/// land in a dead frame slot and are not behavior.
export!(cdecl, rw_008D54D0(arg0: u32, arg1: f32, arg2: u32) -> u32 {
    unsafe {
        let mut buf = [
            0xbc23d70au32,
            0xbc23d70a,
            0x3f8147ae,
            0xbc23d70a,
            0x3f8147ae,
            0x3f8147ae,
            0xbc23d70a,
            0x3f8147ae,
        ];
        callee_cdecl!(0, u32, buf.as_mut_ptr() as u32, 0);
        callee_cdecl!(1, u32, arg0, arg1.to_bits(), arg2, 0)
    }
});
