// original: 0x00e63b40 setup_record_pair
/// Initialise 2 records of 0x3B64 bytes at 0x11E6BC0.
///
/// Each record is passed with `(0x4C, 0xC8, 0x947E90)` to the setup helper
/// (stdcall/4, stubbed by the checker). Returns the last answer.
export!(cdecl, rw_00e63b40() -> u32 {
    unsafe {
        const TABLE: u32 = 0x11E6BC0;
        const COUNT: usize = 2;
        const STRIDE: u32 = 0x3B64;
        let mut obj = relocated(TABLE);
        let mut last = 0u32;
        for _ in 0..COUNT {
            last = callee_stdcall!(1, u32, obj, 0x4C, 0xC8, relocated(0x947E90));
            obj = obj.wrapping_add(STRIDE);
        }
        last
    }
});
