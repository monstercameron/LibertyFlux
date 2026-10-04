// original: 0x0059d890 store_flag_and_forward
// Store a byte flag to global state, then forward it with selector 3.
//
// Only the low byte of the argument is significant (the original loads it
// with movzx); the same masked value is stored and passed on.
export!(cdecl, rw_0059D890(v: u32) -> u32 {
    let b = v & 0xFF;
    unsafe {
        *global::<u32>(0x17F5864) = b;
    }
    callee_cdecl!(1, u32, 3, b)
});
