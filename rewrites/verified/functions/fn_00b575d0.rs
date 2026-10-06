// original: 0x00b575d0 setup_thread_local_queues
/// Set up the two thread-local work queues on an object.
///
/// Resolves the scale factor, reads the thread queue block through the TLS
/// slot named by the index global, allocates both queue headers through the
/// block's allocator, stamps each header's stored pointers, summed length and
/// ready flag, and runs the queue attach step over the object and over its
/// second half. Returns the second attach answer.
export!(thiscall, rw_00B575D0(obj: *mut u8, arg1: u32) -> u32 {
    unsafe {
        let scale = callee_cdecl!(1, u32, arg1, 1);
        let idx = *global::<u32>(TLS_QUEUE_INDEX);
        let tls = tls_slot(idx as usize);
        let inner = *((tls + 8) as *const u32);
        let vt = *(inner as *const u32);
        let tgt = *((vt as *const u8).add(8) as *const u32);
        let alloc: extern "thiscall" fn(u32, u32, u32, u32) -> u32 =
            core::mem::transmute(tgt as usize);
        let first = alloc(inner, scale, 0x10, 0);
        *(obj.add(0x14) as *mut u32) = first;
        *(obj.add(0x18) as *mut u32) = first;
        *(obj.add(0x1C) as *mut u32) = first.wrapping_add(scale);
        *obj.add(0x20) = 1;
        callee_thiscall!(3, u32, obj as u32, arg1, 1, 0, 1, 0);
        let second = alloc(inner, scale, 0x10, 0);
        *(obj.add(0x38) as *mut u32) = second;
        *(obj.add(0x3C) as *mut u32) = second;
        *(obj.add(0x40) as *mut u32) = second.wrapping_add(scale);
        *obj.add(0x44) = 1;
        callee_thiscall!(3, u32, (obj as u32).wrapping_add(0x24), arg1, 1, 0, 1, 0)
    }
});
