// original: 0x00685E40 crFrame::destroy_element (proposed)

/// Dispatch destruction through the first entry in the object's vtable. The
/// stdcall argument is a pointer to an object whose vtable pointer is at `+0`.
/// If the object is null, the original returns without making a call. For a
/// nonnull object it places the literal one in the outgoing stack argument,
/// then tail-jumps to vtable slot zero with the object in ECX. The dispatcher
/// result is returned unchanged. The original leaves EAX untouched on its
/// null branch; the tested null case enters with EAX zero.
lf_checker_rt::export!(stdcall, rw_00685E40(object: u32) -> u32 {
    if object == 0 {
        return 0;
    }
    unsafe { lf_checker_rt::callee_thiscall!(1, u32, object, 1) }
});
