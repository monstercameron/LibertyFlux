// original: 0x0069a1b0 channel_init_accumulate_4
/// Install a channel vtable and accumulate the allocator answer.
///
/// Same shape as [`rw_00699820`] with a different vtable.
export!(cdecl, rw_0069a1b0(obj: u32, arg2: u32) -> () {
    unsafe {
        if obj == 0 {
            return;
        }
        *(obj as *mut u32) = relocated(0xFE3A1C);
        let size = *((obj as *const u32).add(2));
        if size == 0 {
            return;
        }
        
        let delta = callee_thiscall!(1, u32, arg2, size);
        *((obj as *mut u32).add(2)) = size.wrapping_add(delta);
    }
});
