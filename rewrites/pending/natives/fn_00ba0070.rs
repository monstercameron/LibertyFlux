// original: 0x00ba0070 IS_PED_A_MISSION_PED
// IS_PED_A_MISSION_PED: forwards the ped handle and stores the engine
// answer's low byte (zero-extended) into the return slot. Returns the slot
// pointer: the original reloads EAX from the context for the store.
export!(cdecl, rw_fn_ba0070(ctx: *mut u8) -> u32 {
    unsafe {
        let args = *((ctx.add(8)) as *const *const u32);
        let ans: u32 = callee_cdecl!(1, u32, *args);
        let ret = *(ctx as *mut *mut u32);
        *ret = ans & 0xFF;
        ret as u32
    }
});
