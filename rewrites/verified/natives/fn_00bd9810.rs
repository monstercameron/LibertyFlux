// original: 0x00bd9810 UNOBFUSCATE_INT_ARRAY
// rw_unobfuscate_int_array: native UNOBFUSCATE_INT_ARRAY (handler 0x00BD9810).
//
// Forwards two array pointers to the int-array deobfuscator. No return slot.
export!(cdecl, rw_unobfuscate_int_array(ctx: *mut u32) -> u32 {
    unsafe {
        let args = *(ctx.add(2) as *mut *const u32);
        let ans = callee_cdecl!(1, u32, *args.add(0), *args.add(1));
        ans
    }
});
