// original: 0x0059d8c0 forward_shifted_pair
// Forward `(second + 15, first)` to the shared callee.
//
// The addition wraps: edge trials cover `second` near `u32::MAX`.
export!(cdecl, rw_0059D8C0(a: u32, b: u32) -> u32 {
    callee_cdecl!(1, u32, b.wrapping_add(15), a)
});
