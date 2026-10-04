// original: 0x00661840 rage::snDestroyTask::vf7
/// Destroy task teardown: drain, forward, release the session object.
///
/// thiscall/2, returns void. Drains the pending submit when one is latched,
/// forwards both arguments to the shared teardown helper, releases the
/// session object and reports through a stack-built completion record.
export!(thiscall, rw_00661840(this_ptr: u32, a0: u32, a1: u32) -> u32 {
    unsafe {
        if ((this_ptr + 0x90) as *const u32).read() == 1 {
            let session = ((this_ptr + 0x60) as *const u32).read();
            callee_thiscall!(1, u32, session.wrapping_add(0x48), this_ptr.wrapping_add(0x90));
        }
        let session = ((this_ptr + 0x60) as *const u32).read();
        callee_thiscall!(2, u32, this_ptr, a0, a1);
        ((this_ptr + 0x60) as *mut u32).write(0);
        callee_thiscall!(3, u32, session);
        // The struct words below account for the push shifting esp: the
        // vtable constant lands at the pointed-to word itself.
        let mut record = [relocated(0x00fe3494), 0u32, 0u32, 0u32];
        record[1] = record.as_mut_ptr() as u32;
        callee_thiscall!(4, u32, session, record.as_mut_ptr() as u32);
        0
    }
});
