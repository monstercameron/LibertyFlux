// original: 0x0089b550 aud_indexed_call_store
/// Runs the voice engine for an indexed entry and stores its answer.
///
/// Resolves the voice entry like slot 9 above, calls the engine worker for
/// the shared voice manager with the stack argument, and stores the
/// worker's answer into the entry's state word. Returns the answer.
export!(thiscall, rw_0089b550(this: *const u8, arg: u32) -> u32 {
    unsafe {
        let key = *this.add(0x40) as u32;
        let slot = *this.add(0xF7) as u32;
        let stride = *global::<u32>(0x0115D968);
        let table = *global::<u32>(0x0115D988);
        let row = table
            .wrapping_add(key.wrapping_mul(VOICE_ROW_STRIDE))
            .wrapping_add(VOICE_ROW_BIAS);
        let entry = *(row as *const u32);
        let target = (entry as usize)
            .wrapping_add((slot as usize).wrapping_mul(stride as usize))
            as *mut u8;
        let answer = callee_thiscall!(1, u32, relocated(0x0115DC18), arg);
        *(target.add(0x98) as *mut u32) = answer;
        answer
    }
});

