// original: 0x008d85a0 file_wrap_85a0
/// Forward one argument to a fixed-this file helper (thiscall/1).
export!(cdecl, rw_008d85a0(a: u32) -> u32 {
    unsafe {
        /// Fixed `this` pointer the original loads (file VA).
        const THIS: u32 = 0x0103E8D0;
        callee_thiscall!(1, u32, relocated(THIS), a)
    }
});
