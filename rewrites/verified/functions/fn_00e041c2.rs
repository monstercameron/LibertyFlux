// original: 0x00e041c2 decode_ptr_slot_2a0
/// Returns the decoded form of the pointer stored at 0x17AC298+8.
///
/// Loads the global slot at 0x17AC2A0 and passes it through the
/// pointer-decoding import (stubbed by the checker), returning the result.
export!(cdecl, rw_00e041c2() -> u32 {
    unsafe {
        let slot = *global::<u32>(0x17AC2A0);
        callee_stdcall!(1, u32, slot)
    }
});
