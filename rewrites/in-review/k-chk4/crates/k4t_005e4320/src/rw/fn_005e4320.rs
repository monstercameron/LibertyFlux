// original: 0x005e4320 NativeImpl_CREATE_HTML_SCRIPT_OBJECT
// rw_005e4320: create an HTML script object and seed its default text.
//
// Allocates a pool slot through the pool allocator, initializes the text
// header (empty length, cleared flags), derives the slot index from the
// object address with signed division, and appends the default string.
// Returns the new slot index.
export!(stdcall, rw_005e4320(arg: u32) -> u32 {
    unsafe {
        let pool = relocated(POOL_BASE);
        let obj: u32 = callee_thiscall!(1, u32, pool);
        callee_cdecl!(2, u32, obj, arg, 0x20);
        *(obj.wrapping_add(OBJ_LEN) as *mut u32) = 0;
        *(obj.wrapping_add(OBJ_TEXT) as *mut u8) = 0;
        *(obj.wrapping_add(OBJ_FLAG) as *mut u8) = 0;
        let base = *global::<u32>(POOL_BASE);
        let stride = *global::<u32>(POOL_STRIDE);
        // Matches `cdq; idiv m32`: 64-bit signed dividend from the address
        // difference, 32-bit signed divisor. Contracts keep the divisor
        // nonzero with a fitting quotient, exactly like the game's pool.
        let diff = obj.wrapping_sub(base) as i32 as i64;
        let index = (diff / (stride as i32 as i64)) as u32;
        callee_stdcall!(3, u32, index, relocated(DEFAULT_TEXT));
        index
    }
});
