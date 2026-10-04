// original: 0x00699820 channel_init_accumulate
/// Install a channel vtable and accumulate the allocator answer.
///
/// When the object is non-null, writes the relocated vtable pointer, then
/// when its size word at offset 8 is non-zero calls the sizing helper with
/// the second argument and adds the answer into the size word. Returns
/// nothing observable (EAX keeps the entry value on the null path).
export!(cdecl, rw_00699820(obj: u32, arg2: u32) -> () {
    unsafe {
        if obj == 0 {
            return;
        }
        *(obj as *mut u32) = relocated(0xFE3B8C);
        let size = *((obj as *const u32).add(2));
        if size == 0 {
            return;
        }
        
        let delta = callee_thiscall!(1, u32, arg2, size);
        *((obj as *mut u32).add(2)) = size.wrapping_add(delta);
    }
});
