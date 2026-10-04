// original: 0x00dfdcff flag_block_update
// rs03f12: flag-block update (cdecl/0).
//
// Reads the flag-block base from its provider, then applies update step one
// to the word at offset 0x20 past it, returning the step's answer.
export!(cdecl, rw_rs03f12() -> u32 {
    unsafe {
        let base = callee_cdecl!(1, u32,);
        callee_cdecl!(2, u32, 1, base.wrapping_add(0x20))
    }
});
