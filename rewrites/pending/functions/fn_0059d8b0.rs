// original: 0x0059d8b0 flag_forward_callee2
// Forward a value to the shared callee with selector 2.
//
// The original pushes its argument and the constant 2 and returns whatever
// the callee returns; the call itself is scripted by the checker.
export!(cdecl, rw_0059D8B0(a: u32) -> u32 {
    callee_cdecl!(1, u32, 2, a)
});
