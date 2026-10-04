// original: 0x0069a140 channel_init_accumulate_3
/// Install a channel vtable and accumulate the allocator answer.
///
/// Same shape as [`rw_00699820`] with a different vtable.
export!(cdecl, rw_0069a140(obj: u32, arg2: u32) -> () {
    unsafe {
        if obj == 0 {
            return;
        }
        *(obj as *mut u32) = relocated(0xFE39C4);
        let size = *((obj as *const u32).add(2));
        if size == 0 {
            return;
        }
        let f: extern "thiscall" fn(u32, u32) -> u32 =
            core::mem::transmute(callee_addr(1) as usize);
        let delta = f(arg2, size);
        *((obj as *mut u32).add(2)) = size.wrapping_add(delta);
    }
});
