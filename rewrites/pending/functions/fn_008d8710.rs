// original: 0x008d8710 file_wrap_8710
/// Forward three arguments to a fixed-this file helper (thiscall/3).
export!(cdecl, rw_008d8710(a: u32, b: u32, d: u32) -> u32 {
    unsafe {
        /// Fixed `this` pointer the original loads (file VA).
        const THIS: u32 = 0x0103E8D0;
        callee_thiscall!(1, u32, relocated(THIS), a, b, d)
    }
});
