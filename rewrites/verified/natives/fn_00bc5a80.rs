// original: 0x00bc5a80 GET_CAR_FORWARD_VECTOR
/// Script native `GET_CAR_FORWARD_VECTOR` (hash 0x7E4F49B5).
///
/// Forwards two script arguments (a vehicle handle and a pointer to a
/// three-word direction vector) to the engine, but not directly: the
/// handler first stashes the vector pointer in the call context's array
/// at the current slot index (`ctx+0xc`), copies the three vector words
/// into the context's scratch buffer, bumps the slot index, and passes
/// the engine the handle plus a pointer to the copied vector.
export!(cdecl, rw_00bc5a80(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let handle = *args;
        let vector = *args.add(1) as *const u32;
        let slot_index = *(ctx.add(0xc) as *const u32);
        // Stash the vector pointer in the context array at the current index.
        *((ctx.add(0x10) as *mut u32).add(slot_index as usize)) = vector as u32;
        // Copy the three vector words into the context's scratch buffer and
        // hand the engine a pointer to the copy plus the handle.
        let buf = (ctx as u32).wrapping_add((slot_index.wrapping_add(2)) << 4);
        *(buf as *mut u32) = *vector;
        *((buf + 4) as *mut u32) = *vector.add(1);
        *((buf + 8) as *mut u32) = *vector.add(2);
        *(ctx.add(0xc) as *mut u32) = slot_index.wrapping_add(1);
        callee_cdecl!(1, u32, handle, buf)
    }
});
