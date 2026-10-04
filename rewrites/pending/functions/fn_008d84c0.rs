// original: 0x008d84c0 NativeImpl_IS_MODEL_IN_CDIMAGE
/// Native `IS_MODEL_IN_CDIMAGE`: forward a slot key to the engine helper.
///
/// Calls `callee(MODULE, entry + a1)` as thiscall/1 and returns the answer.
export!(cdecl, rw_008d84c0(a1: u32, a2: u32) -> u32 {
    unsafe {
        let combined = (*global::<u32>(0x013053A8)
            .byte_add(a2.wrapping_mul(100) as usize)).wrapping_add(a1);
        callee_thiscall!(1, u32, relocated(0x0103E8D0), combined)
    }
});
