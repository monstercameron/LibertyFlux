// original: 0x009061c0 input_drop_all (proposed)
/// Drop every table slot through the remover.
///
/// Calls the remover callee with `(index, 0)` for each index from 0 to
/// `0x5DB`, and returns the last answer. Cdecl with no arguments.
export!(cdecl, rw_009061c0() -> u32 {
    unsafe {
        /// Table length dropped.
        const LEN: u32 = 0x5DC;
        const REMOVE_ID: u32 = 1;
        let mut ans: u32 = 0;
        let mut i: u32 = 0;
        while i < LEN {
            ans = callee_cdecl!(REMOVE_ID, u32, i, 0u32);
            i = i.wrapping_add(1);
        }
        ans
    }
});
