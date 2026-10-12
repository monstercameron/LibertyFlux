// original: 0x00871FD0 rage::crExpressionProcessor::vf0

/// Delete an expression processor and conditionally release its thread object.
///
/// Clear the child and count fields at `+0x0c` and `+0x6c`, then install the
/// processor vtable at the object and embedded-object offsets `+0x00` and
/// `+0x60`. If the low flag bit is set, read the manager from TLS slot zero
/// and call its virtual release method at slot `+0x0c`, passing the manager in
/// ECX and this object as the stack argument. Return the original pointer.
///
/// Original calling convention: thiscall with one stack flag word.
lf_checker_rt::export!(thiscall, rw_00871fd0(this: u32, flags: u32) -> u32 {
    const PROCESSOR_VTABLE: u32 = 0x00e86afc;
    const TLS_MANAGER_FIELD: u32 = 8;
    const RELEASE_SLOT: u32 = 0x0c;
    unsafe {
        (this.wrapping_add(0x0c) as *mut u32).write_unaligned(0);
        (this.wrapping_add(0x6c) as *mut u32).write_unaligned(0);
        let relocated_vtable = lf_checker_rt::relocated(PROCESSOR_VTABLE);
        (this.wrapping_add(0x60) as *mut u32).write_unaligned(relocated_vtable);
        (this as *mut u32).write_unaligned(relocated_vtable);
        if flags & 1 != 0 {
            let thread_state = lf_checker_rt::tls_slot(0);
            let manager = ((thread_state.wrapping_add(TLS_MANAGER_FIELD)) as *const u32).read_unaligned();
            let vtable = (manager as *const u32).read_unaligned();
            let target = ((vtable.wrapping_add(RELEASE_SLOT)) as *const u32).read_unaligned();
            let release: extern "thiscall" fn(u32, u32) -> u32 =
                core::mem::transmute(target as usize);
            let _release_result = release(manager, this);
        }
    }
    this
});
