// original: 0x00b9ac30 LOAD_ALL_PATH_NODES
// LOAD_ALL_PATH_NODES: coerces the script word to a bool, forwards it, and
// stores the engine answer's low byte (zero-extended) into the return slot.
// Returns the slot pointer, like the original's context reload.
export!(cdecl, rw_fn_b9ac30(ctx: *mut u8) -> u32 {
    unsafe {
        let args = *((ctx.add(8)) as *const *const u32);
        let arg = (ctx as u32 & 0xFFFFFF00) | ((*args != 0) as u32);
        let ans: u32 = callee_cdecl!(1, u32, arg);
        let ret = *(ctx as *mut *mut u32);
        *ret = ans & 0xFF;
        ret as u32
    }
});
