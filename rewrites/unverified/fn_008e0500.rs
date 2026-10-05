// original: 0x008e0500 tls_guarded_emit

/// Emit `obj` through its handler and the emitter under the thread-local
/// guard: link a frame record (object, previous link, channel tag) into the
/// current thread's guard slot, bump the global nesting counter, run the
/// handler when the object's word at `+8` is non-null, always run the
/// emitter on the object, then unlink and release the counter.
///
/// Only the first stack argument past the ignored one is read. The handler
/// receives the frame record's address; its contents are compared, its value
/// is not. Returns the previous link with its low byte set.
///
/// Original: cdecl, two stack arguments (the first is never read).
lf_checker_rt::export!(cdecl, rw_008e0500(_ignored: u32, obj: *mut u32) -> u32 {
    unsafe {
        let slot = *lf_checker_rt::global::<u32>(0x17ABA14);
        let tlsobj = lf_checker_rt::tls_slot(slot as usize) as *mut u32;
        let prev = *tlsobj.add(1);
        let frame = [obj as u32, prev, lf_checker_rt::relocated(0xE81E7C)];
        *tlsobj.add(1) = frame.as_ptr() as u32;
        let ctr = lf_checker_rt::global::<u32>(0x18B7A50);
        *ctr = (*ctr).wrapping_add(1);
        let target = *obj.add(2);
        if target != 0 {
            lf_checker_rt::callee_thiscall!(1, u32, target, frame.as_ptr() as u32);
        }
        lf_checker_rt::callee_cdecl!(2, u32, obj as u32);
        *ctr = (*ctr).wrapping_sub(1);
        *tlsobj.add(1) = prev;
        (prev & 0xFFFFFF00) | 1
    }
});
