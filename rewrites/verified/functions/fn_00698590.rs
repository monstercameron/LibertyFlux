// original: 0x00698590 rage::crCreatureComponentMover::vf7
/// Forward a mover request with constant opcode words.
///
/// Calls the request handler with ECX set to the first argument and the
/// constant words (5, 6, id, table, 1), where `id` is the word at offset
/// 0x50 and `table` is a relocated data address. Returns its answer.
export!(thiscall, rw_00698590(this: u32, arg1: u32, _arg2: u32) -> u32 {
    unsafe {
        let id = *((this as *const u16).add(0x28)) as u32;
        
        callee_thiscall!(1, u32, arg1, 5, 6, id, relocated(0x1110090), 1)
    }
});
