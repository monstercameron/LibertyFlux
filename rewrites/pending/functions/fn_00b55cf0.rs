// original: 0x00b55cf0 load_field0c_float_or_zero
/// Resolve a pair through the helper and return the float at +0x0c of it.
///
/// Calls the two-argument helper; a null answer yields +0.0, otherwise the
/// single-precision value at offset +0x0c of the returned object is loaded
/// and returned on the x87 stack.
export!(cdecl, rw_00b55cf0(a: u32, b: u32) -> f32 {
    unsafe {
        let obj: u32 = callee_cdecl!(1, u32, a, b);
        if obj != 0 {
            ((obj + 0x0c) as *const f32).read()
        } else {
            0.0
        }
    }
});
