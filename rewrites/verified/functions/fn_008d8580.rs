// original: 0x008d8580 file_wrap_8580
/// Forward two arguments to a fixed-this streaming helper (thiscall/2).
export!(cdecl, rw_008d8580(a: u32, b: u32) -> u32 {
    unsafe {
        /// Fixed `this` pointer the original loads (file VA).
        const THIS: u32 = 0x01173750;
        callee_thiscall!(1, u32, relocated(THIS), a, b)
    }
});
