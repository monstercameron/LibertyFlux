// original: 0x0059d870 forward_with_slot_pointer
// Forward `(a1, a2, a3, 0, &a4slot)` to a cdecl callee.
//
// The last argument is a pointer to the caller's own fourth-argument slot,
// which the callee may write through (out-param). `&a4` addresses the
// incoming stack slot rustc homes the argument in; the contract's scripted
// out-param writes land at the same stack offset on both sides, which the
// stack comparison verifies.
export!(cdecl, rw_0059D870(a1: u32, a2: u32, a3: u32, a4: u32) -> u32 {
    let slot = &a4 as *const u32 as u32;
    callee_cdecl!(1, u32, a1, a2, a3, 0, slot)
});
