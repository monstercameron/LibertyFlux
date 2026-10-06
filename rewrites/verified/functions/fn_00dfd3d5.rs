// original: 0x00dfd3d5 forward_with_frame_addr_2
/// Forward two values and an out-slot address to a shared helper.
///
/// Passes a constant tag, its first two arguments, zero, and the address of its third argument slot to the shared helper, and returns the helper answer.
///
/// The helper takes the address of the third argument slot; the contract
/// skips the address and snapshots the one pointed-to word.
export!(cdecl, rw_00dfd3d5(first: u32, second: u32, third: u32) -> u32 {
    unsafe {
        let slot = third;
        callee_cdecl!(1, u32, relocated(0xe0a1a9), first, second, 0, &slot as *const u32 as u32)
    }
});
